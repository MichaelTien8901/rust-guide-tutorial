---
layout: default
title: Declarative Macros
parent: Part 4 - Advanced
nav_order: 3
---

# Declarative Macros

Create macros with `macro_rules!`.

## Basic Macro

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

## With Parameters

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);
```

## Repetition

```rust
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

fn main() {
    let v = vec_of_strings!["a", "b", "c"];
}
```

## Next Steps

Learn about [Procedural Macros]({% link part4/04-macros-procedural.md %}).
