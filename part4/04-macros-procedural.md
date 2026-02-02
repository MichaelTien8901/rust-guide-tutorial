---
layout: default
title: Procedural Macros
parent: Part 4 - Advanced
nav_order: 4
---

# Procedural Macros

Custom derive, attribute, and function-like macros.

## Derive Macros

```rust
use my_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

## Creating a Derive Macro

```rust
// In proc-macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}
```

{: .note }
Procedural macros require a separate crate with `proc-macro = true`.

## Next Steps

Learn about [Advanced Traits]({% link part4/05-advanced-traits.md %}).
