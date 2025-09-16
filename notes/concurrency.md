# Concurrency & Parallelism

#concurrency #threads #async-await #channels #parallelism

> Safe concurrent programming with threads, async/await, and message passing

The Rust standard library uses a _1:1_ model of thread implementation, whereby a program uses one OS thread per one language thread.
There are crates that implement other models of threading that make different tradeoffs to the 1:1 model, like Rust’s async system.

## Spawn

We can spawn OS threads using `std::thread::spawn`
``` rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

We pass a closure for the thread to run. When the main thread ends, all spawned threads are shutdown irrespective of whether they've completed.

`thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run. 

> Note: 
> This is not the same as `pthread_yield`.
> If the intention is to yield the current time-slice you may want to use [`yield_now`](https://doc.rust-lang.org/std/thread/fn.yield_now.html "fn std::thread::yield_now") instead.

The order of thread execution is entirely dependent on the OS scheduler.

## Message Passing

TODO
## `Mutex<T>` Concurrency Primitive

Mutexes are used to provide mutually exclusive access to data regions we want to share between threads. The `Mutex<T>` is a primitive used for guarding against concurrent access to a shared value.

The `Mutex<T>` itself is an *immutable reference* that guards access to a *mutable reference*, and therefore provides *interior mutability* like [[smart-pointers#`RefCell<T>`|RefCell<T>]].

In the same way we used `RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`.

`RefCell<T>` is to `Rc<T>` as `Arc<T>` is to `Mutex<T>`.

Using `Rc<T>` comes with the risk of creating reference cycles, where two `Rc<T>` values refer to each other, causing memory leaks. 

Similarly, `Mutex<T>` comes with the risk of creating _deadlocks_, when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever
### Example with Single Thread

``` rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
	    // panic if lock will never become available
        let mut num = m.lock().unwrap();
        // acquired lock, now we can safely mutate the shared data
        *num = 6;
    }

    println!("m = {m:?}");
}
```

To acquire the mutex, the thread must call `lock`, which:
- Blocks until lock is available
- Fails if another thread holding the lock panics 
If another thread holding the lock panics, that means the mutex would never be unlocked.

The type of the mutex in the above snippet is `Mutex<i32>`, so it *must* be unwrapped in order to access the inner value.

What actually happens is:
``` rust
Mutex::new(5) -> Mutex<i32> -> Mutex<i32>.lock() -> LockResult<MutexGuard,Err>
```

The `LockResult` is returned by the call to `lock` and will be a `MutexGuard` in the non-error case.

`MutexGuard` is a smart pointer, so `Deref` accesses the inner data, and `Drop`  releases the lock automatically.
### Example with Multiple Threads

The following snippet will fail because we are trying to move the `counter` Mutex value to multiple owners, but this violates the "single owner" rule of the borrow checker.

``` rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

	
    for _ in 0..10 {
	    // ERROR: 
	    // Since it gets moved in the first iteration, it will fail the second iteration
	    // since `counter` value was moved in the previous iteration of the loop.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
    // wait for each thread to finish executing with join
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Since we want to share an immutable reference between multiple owners, we can try to use [[smart-pointers#`Rc<T>`|Rc<T>]] to wrap the `Mutex<T>`, but it is *not thread-safe*, so errors out:

``` rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    // ERROR: `Rc<Mutex<i32>>` cannot be sent between threads safely
    // ERROR: the trait `Send` is not implemented for `Rc<Mutex<i32>>`
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

The reason `Rc<T>` is not thread-safe is because the reference counter used by it doesn't use any concurrency primitives to guard against concurrent access. 

The solution is to use the thread-safe version: `Arc<T>` or "atomic reference counter".

## `Arc<T>`: Atomic Reference Counter

`Arc<T>` uses an atomic reference counter which incurs performance penalty but allows for safe access across threads.

Wrapping the mutex creates an *atomically-reference counted mutually-exclusive value* or `Arc<Mutex<T>>` and enables:
- Multiple owners of an immutable reference across threads
- Mutable access to a single value across threads

``` rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

`Muted<T>` isn't the best fit for the above use-case, since we're just incrementing a variable from multiple threads and printing the final result. Instead we could use `AtomicI8` or similar from the [atomics library](https://doc.rust-lang.org/std/sync/atomic/index.html).

## `Send` and `Sync` Traits

