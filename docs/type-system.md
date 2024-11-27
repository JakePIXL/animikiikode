# Animikiikode Type System

## Base Types
- Integer (Signed): `i8`, `i16`, `i32`, `i64`
- Integer (Unsigned): `u8`, `u16`, `u32`, `u64`
- Float: `f32`, `f64`
- Boolean: `bool`
- String: `string`

## Type Coercion Rules
1. Static + Dynamic = First Static Type
2. Dynamic + Dynamic = Dynamic
3. Static + Static = Static

## Collections
- `Vec<T>` - Dynamic array
- `HashMap<K, V>` - Key-value store
- Collection Operations:
  - `push(vec, item)` - Add to collection
  - `pop(vec) -> T` - Remove from collection

## Dynamic Typing
The `dyn` keyword allows for dynamic typing within a statically typed system:
```rust
let dyn x = "Hello"; // Type determined at runtime
let y: i32 = 42;     // Type determined at compile time
```
