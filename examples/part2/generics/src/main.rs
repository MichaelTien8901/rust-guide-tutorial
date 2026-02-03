//! Generics Example
//!
//! Demonstrates generic types, functions, structs, enums,
//! and methods with type parameters.

use std::fmt::Display;

fn main() {
    println!("=== Generics ===\n");

    println!("--- Generic Functions ---");
    generic_functions();

    println!("\n--- Generic Structs ---");
    generic_structs();

    println!("\n--- Generic Enums ---");
    generic_enums();

    println!("\n--- Generic Methods ---");
    generic_methods();

    println!("\n--- Trait Bounds ---");
    trait_bounds_examples();

    println!("\n--- Multiple Type Parameters ---");
    multiple_type_params();

    println!("\n--- Const Generics ---");
    const_generics();
}

fn generic_functions() {
    // Generic function
    fn identity<T>(value: T) -> T {
        value
    }

    let num = identity(42);
    let text = identity("hello");
    let float = identity(3.14);

    println!("identity(42) = {}", num);
    println!("identity(\"hello\") = {}", text);
    println!("identity(3.14) = {}", float);

    // Generic with trait bound
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}

fn generic_structs() {
    // Generic struct with one type parameter
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: {:?}", int_point);
    println!("Float point: {:?}", float_point);

    // Generic struct with multiple type parameters
    #[derive(Debug)]
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("Mixed point: {:?}", mixed);

    // Generic wrapper
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
    }

    impl<T> Wrapper<T> {
        fn new(value: T) -> Self {
            Wrapper { value }
        }

        fn get(&self) -> &T {
            &self.value
        }
    }

    let wrapped = Wrapper::new("hello");
    println!("Wrapped value: {}", wrapped.get());
}

fn generic_enums() {
    // Standard library Option and Result are generic enums

    // Custom generic enum
    #[derive(Debug)]
    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("hello");

    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    // Pattern matching with generic enum
    fn process<L: Display, R: Display>(either: Either<L, R>) {
        match either {
            Either::Left(l) => println!("Got left: {}", l),
            Either::Right(r) => println!("Got right: {}", r),
        }
    }

    process(Either::<i32, &str>::Left(100));
    process(Either::<i32, &str>::Right("world"));
}

fn generic_methods() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // Methods for all Point<T>
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    // Methods only for specific types
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 3.0, y: 4.0 };

    println!("int_point.x = {}", int_point.x());
    // int_point.distance_from_origin(); // Won't compile!

    println!("float_point.x = {}", float_point.x());
    println!(
        "Distance from origin: {}",
        float_point.distance_from_origin()
    );

    // Method with different generic types - mixup example
    // Note: This requires a separate impl block to avoid conflicts
    #[derive(Debug)]
    struct MixPoint<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> MixPoint<T, U> {
        fn mixup<V, W>(self, other: MixPoint<V, W>) -> MixPoint<T, W> {
            MixPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = MixPoint { x: 5, y: 10.4 };
    let p2 = MixPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixup result: p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn trait_bounds_examples() {
    // Single bound
    fn print_it<T: Display>(value: T) {
        println!("Value: {}", value);
    }

    print_it(42);
    print_it("hello");

    // Multiple bounds with +
    fn print_and_clone<T: Display + Clone>(value: T) {
        let cloned = value.clone();
        println!("Original: {}, Clone: {}", value, cloned);
    }

    print_and_clone("test");

    // where clause for clarity
    fn complex_function<T, U>(t: T, u: U) -> String
    where
        T: Display + Clone,
        U: Display,
    {
        format!("{} and {}", t, u)
    }

    let result = complex_function("hello", 42);
    println!("Complex result: {}", result);

    // Returning impl Trait
    fn make_iterator() -> impl Iterator<Item = i32> {
        vec![1, 2, 3].into_iter()
    }

    for num in make_iterator() {
        println!("From iterator: {}", num);
    }
}

fn multiple_type_params() {
    // Struct with multiple type parameters
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    impl<T, U> Pair<T, U> {
        fn new(first: T, second: U) -> Self {
            Pair { first, second }
        }

        fn swap(self) -> Pair<U, T> {
            Pair {
                first: self.second,
                second: self.first,
            }
        }
    }

    let pair = Pair::new(1, "one");
    println!("Pair: {:?}", pair);

    let swapped = pair.swap();
    println!("Swapped: {:?}", swapped);

    // Function with multiple type parameters
    fn zip_with<T, U, V, F>(t: T, u: U, f: F) -> V
    where
        F: FnOnce(T, U) -> V,
    {
        f(t, u)
    }

    let result = zip_with(5, 3, |a, b| a + b);
    println!("Zipped result: {}", result);

    let result = zip_with("hello", 5, |s, n| format!("{} x {}", s, n));
    println!("Zipped strings: {}", result);
}

fn const_generics() {
    // Const generics (arrays with compile-time size)
    #[derive(Debug)]
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    impl<T: Default + Copy, const N: usize> Array<T, N> {
        fn new() -> Self {
            Array {
                data: [T::default(); N],
            }
        }

        fn len(&self) -> usize {
            N
        }
    }

    let arr: Array<i32, 5> = Array::new();
    println!("Array length: {}", arr.len());
    println!("Array data: {:?}", arr.data);

    // Function with const generic
    fn repeat<T: Clone + std::fmt::Debug, const N: usize>(value: T) -> [T; N] {
        std::array::from_fn(|_| value.clone())
    }

    let repeated: [i32; 3] = repeat(42);
    println!("Repeated: {:?}", repeated);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_identity() {
        fn identity<T>(x: T) -> T {
            x
        }

        assert_eq!(identity(42), 42);
        assert_eq!(identity("test"), "test");
    }

    #[test]
    fn test_generic_struct() {
        #[derive(PartialEq, Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 1, y: 2 };
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_trait_bounds() {
        fn double<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
            x + x
        }

        assert_eq!(double(5), 10);
        assert_eq!(double(2.5), 5.0);
    }
}
