# Animikiikode Memory Model

## Ownership System

### Unique Ownership
Uses `~` prefix for move semantics:
```rust
let data: ~String = ~"Hello";
```

### Shared Ownership
Uses `@` prefix for reference counting:
```rust
let shared: @Vec<i32> = @[1,2,3];
```

### Weak References
Uses `#weak` attribute:
```rust
#weak
let cache_ref = cache;
```

## Resource Management

### File Resources
```rust
#own
struct File {
    handle: FileHandle,
    path: String
}
```

### Thread Safety
```rust
#sync
struct Counter {
    value: i32
}
```

## Concurrency Model

### Channels
```rust
let (send, recv) = channel();
send.push(data);
let value = recv.next();
```

### Async/Await
```rust
async func example() {
    let result = await operation();
    process(result);
}
```

### Actor System
```rust
#actor
struct Worker {
    state: WorkerState
}

impl Worker {
    func handle(msg: Message) {
        // Process message
    }
}
```

## Best Practices
- Use `~` (unique ownership) for most data
- Use `@` (shared ownership) for multiple owners
- Use `#weak` for breaking reference cycles
- Mark thread-safe types with `#sync`
- Use `.with()` for synchronized access
- Prefer channels over shared memory for concurrency
