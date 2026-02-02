//! Closures Example
//!
//! Demonstrates closure syntax, capture modes, and Fn traits.

fn main() {
    println!("=== Closures ===\n");

    println!("--- Closure Syntax ---");
    closure_syntax();

    println!("\n--- Capturing Variables ---");
    capturing_variables();

    println!("\n--- Fn Traits ---");
    fn_traits();

    println!("\n--- Closures as Parameters ---");
    closures_as_parameters();

    println!("\n--- Returning Closures ---");
    returning_closures();

    println!("\n--- Practical Examples ---");
    practical_examples();
}

fn closure_syntax() {
    // Basic closure
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // With type annotations
    let add: fn(i32, i32) -> i32 = |a, b| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // Multiple statements with braces
    let complex = |x: i32| {
        let y = x * 2;
        let z = y + 10;
        z
    };
    println!("complex(5) = {}", complex(5));

    // No parameters
    let say_hello = || println!("Hello!");
    say_hello();

    // Immediately invoked
    let result = (|x, y| x + y)(3, 4);
    println!("Immediate: {}", result);

    // Type inference from usage
    let closure = |x| x;
    let s = closure(String::from("hello")); // Now closure expects String
    println!("Inferred type: {}", s);
    // let n = closure(5); // Error: expected String, found integer
}

fn capturing_variables() {
    // Borrowing immutably (Fn)
    let x = 4;
    let borrow = || println!("x = {}", x);
    borrow();
    println!("Can still use x: {}", x);

    // Borrowing mutably (FnMut)
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("count = {}", count);
    };
    increment();
    increment();
    // Can't use count while increment exists
    drop(increment);
    println!("Final count: {}", count);

    // Taking ownership (FnOnce)
    let s = String::from("hello");
    let consume = move || {
        println!("Consumed: {}", s);
        // s is moved into closure
    };
    consume();
    // s is no longer valid

    // move keyword forces ownership
    let v = vec![1, 2, 3];
    let owns_v = move || println!("v = {:?}", v);
    owns_v();
    // v is moved, can't use it here
}

fn fn_traits() {
    // Fn - can be called multiple times, borrows immutably
    fn call_fn<F: Fn()>(f: F) {
        f();
        f(); // Can call multiple times
    }

    let x = 5;
    call_fn(|| println!("Fn: x = {}", x));

    // FnMut - can be called multiple times, borrows mutably
    fn call_fn_mut<F: FnMut()>(mut f: F) {
        f();
        f();
    }

    let mut count = 0;
    call_fn_mut(|| {
        count += 1;
        println!("FnMut: count = {}", count);
    });

    // FnOnce - can only be called once, may consume captured values
    fn call_fn_once<F: FnOnce() -> String>(f: F) -> String {
        f()
    }

    let s = String::from("hello");
    let result = call_fn_once(move || {
        let mut owned = s;
        owned.push_str(" world");
        owned
    });
    println!("FnOnce result: {}", result);
}

fn closures_as_parameters() {
    // Generic with Fn bound
    fn apply<F, T, R>(f: F, value: T) -> R
    where
        F: Fn(T) -> R,
    {
        f(value)
    }

    let double = |x: i32| x * 2;
    println!("apply(double, 5) = {}", apply(double, 5));

    // Using impl Fn (simpler syntax)
    fn apply_simple(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    println!("apply_simple = {}", apply_simple(|x| x + 10, 5));

    // Multiple closure parameters
    fn compose<F, G, A, B, C>(f: F, g: G, x: A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        g(f(x))
    }

    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    println!("compose = {}", compose(add_one, double, 5)); // (5+1)*2 = 12
}

fn returning_closures() {
    // Must use Box<dyn Fn...> for returned closures
    fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + n)
    }

    let add_5 = make_adder(5);
    let add_10 = make_adder(10);

    println!("add_5(3) = {}", add_5(3));
    println!("add_10(3) = {}", add_10(3));

    // Using impl Fn in return position (when type is known)
    fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x * n
    }

    let times_3 = make_multiplier(3);
    println!("times_3(4) = {}", times_3(4));
}

fn practical_examples() {
    // Sorting with closures
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    numbers.sort_by(|a, b| a.cmp(b));
    println!("Sorted: {:?}", numbers);

    numbers.sort_by(|a, b| b.cmp(a)); // Reverse
    println!("Reverse sorted: {:?}", numbers);

    // Custom sort
    let mut words = vec!["banana", "apple", "cherry"];
    words.sort_by_key(|s| s.len());
    println!("By length: {:?}", words);

    // Map with closure
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);

    // Filter with closure
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Evens: {:?}", evens);

    // Fold with closure
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // Closure in struct
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut expensive = Cacher::new(|num| {
        println!("Calculating...");
        num * 2
    });

    println!("First call: {}", expensive.value(5));
    println!("Second call: {}", expensive.value(5)); // Uses cached value
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure_capture() {
        let x = 5;
        let add_x = |y| y + x;
        assert_eq!(add_x(3), 8);
    }

    #[test]
    fn test_closure_mut_capture() {
        let mut count = 0;
        let mut inc = || count += 1;
        inc();
        inc();
        drop(inc);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_closure_move() {
        let v = vec![1, 2, 3];
        let owns = move || v.len();
        assert_eq!(owns(), 3);
    }
}
