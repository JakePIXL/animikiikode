# Animikiikode Language Specification

## Core Features

### Variable Declaration
```rust
let x: i32 = 20;
let dyn y = "Hello World!";
```

### Functions
```rust
func example_func(x: i32, y: dyn) -> i32 {
    let result = x + y;
    result // implicit return
}
```

### Control Flow
```rust
// If/Else
if condition {
    // code
} else {
    // code
}

// Loops
while condition {
    // code
}

for item in iterator {
    // code
}
```

### Standard Library

#### Input/Output
- `print(value: dyn)` - Basic output
- `println(value: dyn)` - Print with newline
- `input() -> string` - Basic input

#### File Operations
- `read_file(path: string)`
- `write_file(path: string, content: string)`

#### Type Conversion
- `to_string(value: dyn) -> string`
- `to_int(value: dyn) -> i32`
- `to_float(value: dyn) -> f64`
- `to_bool(value: dyn) -> bool`

### Module System
```rust
// Declaration
mod module_name;      // Single module
pub mod module_name;  // Public module

// Imports
use module_name::item;
use module_name::{item1, item2};

// Visibility
// pub(crate) - Project visible
// pub - Public interface
// private (default) - Module only
```
