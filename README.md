A simple ternary operator macro in rust.

For example:
```rust
iff!( condition
 : expressionA
 ? expressionB );
```

expands to:
```rust
if condition {
    expressionA
} else {
    expressionB
}
```
