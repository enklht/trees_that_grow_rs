# Trees that Grow in Rust

This small Rust project explores the "Trees That Grow" approach for extensible abstract syntax trees (ASTs).

The code mirrors examples from the original paper (see references) and demonstrates how to represent extensible
algebraic data types in Rust using traits and parameterized enums. The examples in `src/main.rs` include:

- A basic `Typ` enum to represent simple types (integers and function types).
- A generic `Descriptor` trait that parameterizes extensions to the `Exp` enum.
- Multiple concrete descriptor implementations that illustrate different extension strategies (undescibed/empty,
  type-checked, partially evaluated, and type-parameterized expressions).
- Example functions: a type checker for `ExpTC`, a printer for extensible expressions using a `Printer` trait, and
  other explanatory code snippets.

This implementation is more compact compared to [the implementation by guygastineau](https://github.com/guygastineau/rust-trees-that-grow).
In addition, this project explores further aspects of the paper that are not shown in that repository:
it includes an example showing how to model type-class-like behaviour for extensible data types (section 3.7) and an example of a parameterized version (corresponding to section 3.9 of the paper).

References

- "Trees That Grow" (original paper): <https://www.microsoft.com/en-us/research/wp-content/uploads/2016/11/trees-that-grow.pdf>
- A Rust implementation inspired by the paper: <https://github.com/guygastineau/rust-trees-that-grow>
