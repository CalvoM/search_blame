# search and blame

## General structure of cli app

Cli arguments:
- files to search.
- phrase to search.
- who to blame

```rust
struct Cli{
    files: String,
    text: String,
    blame: Option<String>,
}
```