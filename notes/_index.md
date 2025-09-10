# Rust Learning Index

#index #navigation #quick-reference

> Quick navigation between all available concepts and sections

## 📚 Core Concepts

| Concept | Key Topics | Difficulty | Prerequisites |
|---------|------------|------------|---------------|
| **[[variables]]** | Mutability, shadowing, scope, types | Beginner | None |
| **[[control]]** | If/else, loops, match, functions | Beginner | [[variables]] |
| **[[ownership]]** | Move semantics, borrowing, lifetimes | Intermediate | [[variables]], [[control#Functions]] |
| **[[enums]]** | Variants, pattern matching | Beginner | [[control#Pattern Matching with Match]] |
| **[[option]]** | `Some`/`None`, null safety | Intermediate | [[enums]] |
| **[[errors]]** | `Result<T,E>`, `?` operator | Intermediate | [[option]], [[enums]] |
| **[[generics]]** | Type parameters, bounds | Advanced | [[ownership]], [[control#Functions]] |
| **[[traits]]** | Behavior definitions, impl blocks | Advanced | [[generics]], [[ownership]] |
| **[[lifetimes]]** | Lifetime annotations, elision | Advanced | [[ownership]], [[generics]] |
| **[[collections]]** | Vec, String, HashMap, iterators | Intermediate | [[ownership]], [[option]], [[generics]] |

## 🚧 Future Topics (Not Yet Covered)

| Concept | Key Topics | Difficulty | Prerequisites |
|---------|------------|------------|---------------|
| **[[packaging]]** | Cargo, modules, crates, workspaces | Intermediate | [[traits]], [[generics]] |
| **[[testing]]** | Unit tests, integration tests, benchmarks | Intermediate | [[packaging]], [[errors]] |
| **[[iterators]]** | Iterator trait, closures, functional programming | Advanced | [[traits]], [[collections]] |
| **[[smart-pointers]]** | Box, Rc, RefCell, Arc, Mutex | Advanced | [[lifetimes]], [[traits]] |
| **[[concurrency]]** | Threads, channels, async/await | Advanced | [[smart-pointers]], [[packaging]] |
| **[[macros]]** | Declarative and procedural macros | Expert | [[packaging]], [[traits]] |
| **[[unsafe-rust]]** | Raw pointers, unsafe blocks, FFI | Expert | [[smart-pointers]], [[lifetimes]] |
| **[[advanced-types]]** | Type aliases, newtype, DSTs | Expert | [[traits]], [[lifetimes]] |
| **[[patterns]]** | Advanced pattern matching, guards | Intermediate | [[enums]], [[control]] |
| **[[io]]** | File I/O, networking, serialization | Intermediate | [[errors]], [[traits]] |

## 🏗️ Foundation Layer (Start Here)

### Variables & Basic Types
- **[[variables#Variable Bindings]]** - `let` statements and mutability
- **[[variables#Data Types]]** - Scalar and compound types
- **[[variables#Type Inference]]** - Rust's type system
- **[[variables#Shadowing]]** - Variable rebinding patterns
- **[[variables#Scope and Lifetime]]** - Where variables are valid

### Functions & Control Flow
- **[[control#Functions]]** - Function syntax and expressions vs statements
- **[[control#If Expressions]]** - Conditional logic and branching
- **[[control#Loops]]** - `loop`, `while`, `for` constructs
- **[[control#Pattern Matching with Match]]** - Exhaustive pattern matching

## 🧠 Memory Management (Rust's Core)

### Ownership System
- **[[ownership#Ownership Rules]]** - Move semantics and ownership transfer
- **[[ownership#References & Borrowing]]** - Immutable and mutable references
- **[[ownership#String vs &str]]** - Owned vs borrowed string data
- **[[ownership#Structs]]** - Custom data types and ownership

### Advanced Memory Concepts  
- **[[lifetimes#Lifetime Basics]]** - Annotating reference lifetimes
- **[[lifetimes#Lifetime Parameters in Functions]]** - Function signature lifetimes
- **[[lifetimes#Advanced Lifetime Concepts]]** - Complex lifetime relationships

## 🎯 Data Organization

### Enumerations & Pattern Matching
- **[[enums#Basic Enums]]** - Defining enum variants
- **[[enums#Enums with Data]]** - Storing data in variants
- **[[enums#Matching]]** - Pattern matching with `match`
- **[[control#If Let Pattern Matching]]** - Concise pattern matching

### Null Safety & Error Handling
- **[[option#Option Basics]]** - `Some` and `None` variants
- **[[option#Option Methods]]** - `unwrap`, `expect`, `map`, etc.
- **[[errors#Result<T,E> Enum]]** - Success and error cases
- **[[errors#The `?` Operator]]** - Error propagation
- **[[errors#Error Handling Patterns]]** - Best practices and custom errors

## ⚡ Advanced Type System

### Generic Programming
- **[[generics#Basic Generics]]** - Type parameters in functions and structs
- **[[generics#Method Definitions]]** - Generic methods and implementations  
- **[[generics#Traits as Bounds]]** - Constraining generic types

### Traits & Behavior
- **[[traits#Trait Definitions]]** - Defining shared behavior
- **[[traits#Implementing Traits]]** - Adding behavior to types
- **[[traits#Advanced Trait Concepts]]** - Associated types, default methods

## 📦 Collections & Data Structures

### Core Collections
- **[[collections#Vectors]]** - Dynamic arrays (`Vec<T>`)
- **[[collections#Strings]]** - Text data and UTF-8 encoding
- **[[collections#HashMaps]]** - Key-value mappings
- **[[collections#Iterator Fundamentals]]** - Functional programming patterns

### Memory & Performance
- **[[collections#Memory and Performance]]** - Performance characteristics
- **[[collections#Common Patterns and Best Practices]]** - Idiomatic usage

## 🎓 Learning Paths & Resources

### Meta Information
- **[[meta/concept-map]]** - How concepts relate to each other
- **[[meta/learning-paths]]** - Structured learning sequences
- **[[meta/study-tracker]]** - Progress tracking
- **[[meta/obsidian-setup]]** - Optimizing your knowledge graph

### Main Resources
- **[[rust-review-guide]]** - Comprehensive review and practice hub
- **Rustlings Exercises** - Hands-on practice in `exercises/`
- **Custom Review Exercises** - Additional practice in `exercises/00_review/`

## 🔍 Quick Lookup

### By Use Case

| What do you want to do? | Go to |
|-------------------------|-------|
| Store and manipulate text | [[collections#Strings]], [[variables#String Types]] |
| Handle missing values | [[option]] |
| Handle errors gracefully | [[errors]] |
| Create reusable code | [[generics]], [[traits]] |
| Organize related data | [[ownership#Structs]], [[enums]] |
| Iterate over data | [[collections#Iterator Fundamentals]], [[control#Loops]] |
| Understand ownership rules | [[ownership]], [[lifetimes]] |
| Pattern match on data | [[control#Pattern Matching with Match]], [[enums#Matching]] |
| Work with dynamic data | [[collections#Vectors]], [[collections#HashMaps]] |
| Build and organize projects | [[packaging]], [[testing]] |
| Write concurrent code | [[concurrency]], [[smart-pointers]] |
| Advanced iteration patterns | [[iterators]], [[patterns]] |
| Meta-programming | [[macros]], [[advanced-types]] |
| System-level programming | [[unsafe-rust]], [[io]] |

### By Concept Difficulty

#### Beginner (Week 1-2)
- [[variables]], [[control]], [[enums]]

#### Intermediate (Week 3-4)  
- [[ownership]], [[option]], [[errors]], [[collections]]
- **Future**: [[packaging]], [[testing]], [[patterns]], [[io]]

#### Advanced (Week 5+)
- [[generics]], [[traits]], [[lifetimes]]
- **Future**: [[iterators]], [[smart-pointers]], [[concurrency]]

#### Expert (Advanced Rust)
- **Future**: [[macros]], [[unsafe-rust]], [[advanced-types]]

### Common Learning Blockers

| Problem | Solution |
|---------|----------|
| "I don't understand borrowing" | Review [[ownership#References & Borrowing]] and [[ownership#Ownership Rules]] |
| "When to use Option vs Result?" | See [[option#Option vs Result]] and [[errors#When to Use What]] |
| "Generics are confusing" | Start with [[generics#Basic Generics]] then move to [[traits]] |
| "Lifetime errors everywhere" | Master [[ownership]] first, then [[lifetimes#Lifetime Basics]] |
| "Iterator methods are hard" | Begin with [[collections#Iterator Fundamentals]] |

## 🚀 Getting Started

### New to Rust?
1. Start with [[variables]] 
2. Move to [[control]]
3. Tackle [[ownership]]
4. Practice with [[rust-review-guide]]

### Coming back after a break?
1. Check [[rust-review-guide#Progress Dashboard]]
2. Review [[meta/concept-map]] for relationships
3. Focus on areas marked as "needs review"

### Preparing for advanced topics?
1. Ensure solid understanding of [[ownership]]
2. Practice with [[collections]] 
3. Master [[option]] and [[errors]]
4. Then tackle [[generics]] and [[traits]]

---

*💡 **Pro tip**: Use Obsidian's graph view to visualize connections between concepts you're learning!*

*📊 **Track your progress**: Update [[meta/study-tracker]] as you complete sections*