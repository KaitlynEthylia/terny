![Github.com/KaitlynEthylia/terny](https://img.shields.io/badge/Github-KaitlynEthylia%2Fterny-cec2fc?logo=github&style=for-the-badge)
![Crates.io/crates/terny](https://img.shields.io/crates/v/terny?color=%23f7b679&logo=rust&style=for-the-badge)
![Unlicense](https://img.shields.io/crates/l/terny?color=bfdfff&logo=unlicense&style=for-the-badge)
![Docs.rs/terny](https://img.shields.io/docsrs/terny?color=AAff99&logo=docs.rs&style=for-the-badge)


A simple ternary operator macro in rust. the `iff!` macro is the only item exported by this crate, it simply takes three expressions, seperated by `?` and `:`, and expands them info an if else statement.

# Usage
---
```rust
iff!( condition
 ? expressionA
 : expressionB );
```

expands to:
```rust
if condition {
    expressionA
} else {
    expressionB
}
```

It can also be used in assignment:
```rust
let value = iff!(condition
 ? expressionA
 : expressionB );
```

It can also be used inline:
```rust
let value = iff!(condition ? exressionA : expressionB);
```

It may also be nested, although the code starts to look less clean at this point
```rust
let value = iff!(conditionA ? iff!(conditionB ? expressionA : expressionB) : expressionC);
```

Currently using the `?` operator within `iff` is not possible, as there is no way (that I know of) to check for whitespace. This should be changeable once [Tracking issue 54725](https://github.com/rust-lang/rust/issues/54725) becomes stable.
