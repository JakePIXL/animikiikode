# Animikiikode

A modern, safe programming language implemented in Rust, combining static typing with dynamic capabilities.

## Overview

Animikiikode is designed as a proof-of-concept interpreted language that runs in any environment. It features:

- Static typing with dynamic capabilities via `dyn` keyword
- Ownership-based memory management
- Built-in concurrency support
- Actor system for distributed computing
- Modern syntax inspired by Rust and Python

## Project Structure

- `/docs`: Language specification and documentation
- `/src`: Interpreter implementation
- `/tests`: Test suite

## Getting Started

```rust
// Example Animikiikode program
func main() {
    let message: dyn = "Hello, Animikiikode!";
    println(message);
}
```

## Building

```bash
cargo build
cargo test
```

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.
