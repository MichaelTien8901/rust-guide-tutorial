//! Declarative Macros Example
//!
//! Demonstrates macro_rules! for code generation.
//!
//! # Macro Expansion Flow
//! ```text
//!     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
//!     │ Macro Call   │────►│   Pattern    │────►│  Generated   │
//!     │ my_macro!(x) │     │   Matching   │     │    Code      │
//!     └──────────────┘     └──────────────┘     └──────────────┘
//!            │                    │                    │
//!            ▼                    ▼                    ▼
//!     ┌──────────────────────────────────────────────────────┐
//!     │  Fragment Specifiers:                                │
//!     │  $x:expr  - expressions     $x:ty    - types         │
//!     │  $x:ident - identifiers     $x:pat   - patterns      │
//!     │  $x:stmt  - statements      $x:block - blocks        │
//!     │  $x:item  - items           $x:tt    - token tree    │
//!     └──────────────────────────────────────────────────────┘
//! ```

fn main() {
    println!("=== Declarative Macros ===\n");

    println!("--- Basic Macros ---");
    basic_macros();

    println!("\n--- Multiple Patterns ---");
    multiple_patterns();

    println!("\n--- Repetition ---");
    repetition_macros();

    println!("\n--- Fragment Specifiers ---");
    fragment_specifiers();

    println!("\n--- Practical Macros ---");
    practical_macros();

    println!("\n--- DSL Example ---");
    dsl_example();
}

// ============================================
// Basic Macro Definition
// ============================================

/// Simplest macro - no arguments
macro_rules! say_hello {
    () => {
        println!("  Hello from macro!");
    };
}

/// Macro with one expression argument
macro_rules! print_expr {
    ($e:expr) => {
        println!("  Expression: {} = {:?}", stringify!($e), $e);
    };
}

fn basic_macros() {
    say_hello!();

    print_expr!(1 + 2);
    print_expr!(vec![1, 2, 3]);
    print_expr!("hello".to_uppercase());
}

// ============================================
// Multiple Patterns
// ============================================

/// Macro with multiple pattern arms
macro_rules! calculate {
    // Pattern: add two numbers
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    // Pattern: multiply two numbers
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    // Pattern: square a number
    (square $a:expr) => {
        $a * $a
    };
}

/// Macro that handles different arities
macro_rules! greet {
    () => {
        println!("  Hello, World!")
    };
    ($name:expr) => {
        println!("  Hello, {}!", $name)
    };
    ($greeting:expr, $name:expr) => {
        println!("  {}, {}!", $greeting, $name)
    };
}

fn multiple_patterns() {
    println!("  add 2, 3 = {}", calculate!(add 2, 3));
    println!("  mul 4, 5 = {}", calculate!(mul 4, 5));
    println!("  square 6 = {}", calculate!(square 6));

    greet!();
    greet!("Alice");
    greet!("Good morning", "Bob");
}

// ============================================
// Repetition Patterns
// ============================================

/// Macro with repetition: $(...)* for zero or more
macro_rules! make_vec {
    // Match zero or more expressions separated by commas
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

/// Repetition with +: one or more matches
macro_rules! sum {
    ( $( $x:expr ),+ ) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )+
            total
        }
    };
}

/// Repetition with separator
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn repetition_macros() {
    let v = make_vec![1, 2, 3, 4, 5];
    println!("  make_vec![1,2,3,4,5] = {:?}", v);

    let total = sum!(1, 2, 3, 4, 5);
    println!("  sum!(1,2,3,4,5) = {}", total);

    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("  hashmap! = {:?}", map);
}

// ============================================
// Fragment Specifiers
// ============================================

/// Using different fragment specifiers
macro_rules! demo_fragments {
    // ident: identifier
    (ident $name:ident) => {
        let $name = 42;
        println!("  ident: {} = {}", stringify!($name), $name);
    };
    // ty: type
    (ty $t:ty) => {
        println!("  ty: size of {} = {}", stringify!($t), std::mem::size_of::<$t>());
    };
    // pat: pattern
    (pat $p:pat) => {
        let value = Some(42);
        match value {
            $p => println!("  pat: matched {:?}", value),
            _ => println!("  pat: no match"),
        }
    };
    // block: block expression
    (block $b:block) => {
        let result = $b;
        println!("  block result: {:?}", result);
    };
    // tt: token tree (most flexible)
    (tt $($t:tt)*) => {
        println!("  tt: {}", stringify!($($t)*));
    };
}

fn fragment_specifiers() {
    demo_fragments!(ident my_var);
    demo_fragments!(ty i32);
    demo_fragments!(ty String);
    demo_fragments!(pat Some(_));
    demo_fragments!(block { 1 + 2 + 3 });
    demo_fragments!(tt hello world 123);
}

// ============================================
// Practical Macros
// ============================================

/// Debug print with file and line
macro_rules! debug_print {
    ($val:expr) => {
        println!(
            "  [{}:{}] {} = {:?}",
            file!(),
            line!(),
            stringify!($val),
            $val
        );
    };
}

/// Implement a trait for multiple types
macro_rules! impl_display_for_numbers {
    ( $( $t:ty ),* ) => {
        $(
            impl Displayable for $t {
                fn display(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    };
}

trait Displayable {
    fn display(&self) -> String;
}

impl_display_for_numbers!(i32, i64, u32, u64, f32, f64);

/// Enum with string conversion
macro_rules! string_enum {
    ( $name:ident { $( $variant:ident ),* $(,)? } ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $name {
            $( $variant, )*
        }

        impl $name {
            fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => stringify!($variant), )*
                }
            }

            fn variants() -> &'static [&'static str] {
                &[ $( stringify!($variant), )* ]
            }
        }
    };
}

string_enum!(Color {
    Red,
    Green,
    Blue,
    Yellow
});

fn practical_macros() {
    let x = vec![1, 2, 3];
    debug_print!(x);
    debug_print!(x.len());

    let num: i32 = 42;
    println!("  Displayable i32: {}", num.display());

    let color = Color::Blue;
    println!("  Color::Blue as_str: {}", color.as_str());
    println!("  Color variants: {:?}", Color::variants());
}

// ============================================
// DSL Example
// ============================================

/// Simple HTML-like DSL using brace syntax
/// (angle brackets are problematic in macros)
macro_rules! html {
    // Self-closing tag: html!(br)
    ( $tag:ident ) => {
        format!("<{} />", stringify!($tag))
    };
    // Tag with content: html!(p { "Hello" })
    ( $tag:ident { $content:expr } ) => {
        format!("<{}>{}</{}>", stringify!($tag), $content, stringify!($tag))
    };
    // Nested tags: html!(div { html!(p { "text" }) })
    ( $tag:ident { $( $inner:tt )+ } ) => {
        format!("<{}>{}</{}>", stringify!($tag), html!($($inner)+), stringify!($tag))
    };
}

/// Simple test assertion DSL
macro_rules! test_that {
    ($val:expr, equals, $expected:expr) => {
        assert_eq!(
            $val, $expected,
            "Expected {:?} to equal {:?}",
            $val, $expected
        );
        println!("  ✓ {} equals {}", stringify!($val), stringify!($expected));
    };
    ($val:expr, is_some) => {
        assert!($val.is_some(), "Expected {:?} to be Some", $val);
        println!("  ✓ {} is Some", stringify!($val));
    };
    ($val:expr, is_none) => {
        assert!($val.is_none(), "Expected {:?} to be None", $val);
        println!("  ✓ {} is None", stringify!($val));
    };
    ($val:expr, contains, $item:expr) => {
        assert!(
            $val.contains(&$item),
            "Expected {:?} to contain {:?}",
            $val,
            $item
        );
        println!("  ✓ {} contains {}", stringify!($val), stringify!($item));
    };
}

fn dsl_example() {
    // HTML DSL (using brace syntax to avoid macro ambiguity)
    let br = html!(br);
    println!("  {}", br);

    let para = html!(p { "Hello, World!" });
    println!("  {}", para);

    // Test DSL
    test_that!(2 + 2, equals, 4);
    test_that!(Some(42), is_some);
    test_that!(None::<i32>, is_none);
    test_that!(vec![1, 2, 3], contains, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_macro() {
        assert_eq!(calculate!(add 10, 20), 30);
        assert_eq!(calculate!(mul 5, 6), 30);
        assert_eq!(calculate!(square 7), 49);
    }

    #[test]
    fn test_make_vec() {
        let v = make_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);

        let empty: Vec<i32> = make_vec![];
        assert!(empty.is_empty());
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum!(1), 1);
        assert_eq!(sum!(1, 2, 3), 6);
        assert_eq!(sum!(10, 20, 30, 40), 100);
    }

    #[test]
    fn test_hashmap() {
        let map = hashmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_string_enum() {
        assert_eq!(Color::Red.as_str(), "Red");
        assert_eq!(Color::variants().len(), 4);
    }
}
