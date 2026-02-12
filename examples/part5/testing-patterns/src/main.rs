//! Testing Patterns Example
//!
//! Demonstrates testing patterns with proptest and mocking.
//!
//! # Testing Pyramid
//! ```text
//!                      ┌───────────┐
//!                      │   E2E     │  Few, slow, comprehensive
//!                      └───────────┘
//!                    ┌───────────────┐
//!                    │  Integration  │  Some, moderate speed
//!                    └───────────────┘
//!                  ┌───────────────────┐
//!                  │    Unit Tests     │  Many, fast, focused
//!                  └───────────────────┘
//! ```

use std::collections::HashMap;

fn main() {
    println!("=== Testing Patterns ===\n");

    println!("--- Basic Unit Tests ---");
    println!("  Run with: cargo test");

    println!("\n--- Test Organization ---");
    test_organization();

    println!("\n--- Mocking Patterns ---");
    mocking_patterns();

    println!("\n--- Test Fixtures ---");
    test_fixtures();

    println!("\n--- Property-Based Testing ---");
    println!("  See tests module for proptest examples");

    println!("\n--- Integration Testing ---");
    println!("  Place integration tests in tests/ directory");
}

// ============================================
// Code Under Test
// ============================================

/// A simple calculator for demonstration
pub struct Calculator {
    memory: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    pub fn add(&mut self, x: f64, y: f64) -> f64 {
        let result = x + y;
        self.memory = result;
        result
    }

    pub fn subtract(&mut self, x: f64, y: f64) -> f64 {
        let result = x - y;
        self.memory = result;
        result
    }

    pub fn divide(&mut self, x: f64, y: f64) -> Result<f64, &'static str> {
        if y == 0.0 {
            return Err("Division by zero");
        }
        let result = x / y;
        self.memory = result;
        Ok(result)
    }

    pub fn recall_memory(&self) -> f64 {
        self.memory
    }

    pub fn clear_memory(&mut self) {
        self.memory = 0.0;
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Test Organization
// ============================================

fn test_organization() {
    println!("  Test organization patterns:");
    println!("    1. Unit tests: #[cfg(test)] mod tests {{ ... }}");
    println!("    2. Integration tests: tests/*.rs");
    println!("    3. Doc tests: ```rust examples in documentation");
    println!("    4. Benchmark tests: benches/*.rs (with criterion)");
}

// ============================================
// Mocking Patterns
// ============================================

/// Trait for dependency injection
trait DataStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

/// Real implementation
struct RealDataStore {
    data: HashMap<String, String>,
}

impl RealDataStore {
    fn new() -> Self {
        RealDataStore {
            data: HashMap::new(),
        }
    }
}

impl DataStore for RealDataStore {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

/// Service using the data store
struct UserService<S: DataStore> {
    store: S,
}

impl<S: DataStore> UserService<S> {
    fn new(store: S) -> Self {
        UserService { store }
    }

    fn get_user_name(&self, user_id: &str) -> Option<String> {
        self.store.get(&format!("user:{}", user_id))
    }

    fn set_user_name(&mut self, user_id: &str, name: &str) {
        self.store.set(&format!("user:{}", user_id), name);
    }
}

fn mocking_patterns() {
    // Using real implementation
    let store = RealDataStore::new();
    let mut service = UserService::new(store);

    service.set_user_name("1", "Alice");
    let name = service.get_user_name("1");
    println!("  Real store: {:?}", name);

    // In tests, you'd use a mock implementation
    println!("  Mocking approach:");
    println!("    1. Define trait for dependencies");
    println!("    2. Implement trait for real and mock types");
    println!("    3. Inject mock in tests, real in production");
}

// ============================================
// Test Fixtures
// ============================================

/// Test fixture pattern
struct TestContext {
    calculator: Calculator,
    test_values: Vec<(f64, f64)>,
}

impl TestContext {
    fn new() -> Self {
        TestContext {
            calculator: Calculator::new(),
            test_values: vec![(1.0, 2.0), (10.0, 5.0), (-3.0, 7.0), (0.0, 100.0)],
        }
    }

    fn with_memory(mut self, value: f64) -> Self {
        self.calculator.memory = value;
        self
    }
}

fn test_fixtures() {
    let ctx = TestContext::new().with_memory(42.0);
    println!("  Fixture with memory: {}", ctx.calculator.recall_memory());

    println!("  Fixture patterns:");
    println!("    1. Builder pattern for test data");
    println!("    2. Factory functions");
    println!("    3. Setup/teardown with Drop trait");
}

// ============================================
// Tests Module
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================
    // Basic Unit Tests
    // ==================

    #[test]
    fn test_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        let mut calc = Calculator::new();
        assert_eq!(calc.subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_divide_success() {
        let mut calc = Calculator::new();
        assert_eq!(calc.divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        let mut calc = Calculator::new();
        assert_eq!(calc.divide(10.0, 0.0), Err("Division by zero"));
    }

    // ==================
    // Testing Memory
    // ==================

    #[test]
    fn test_memory_after_operation() {
        let mut calc = Calculator::new();
        calc.add(5.0, 3.0);
        assert_eq!(calc.recall_memory(), 8.0);
    }

    #[test]
    fn test_clear_memory() {
        let mut calc = Calculator::new();
        calc.add(5.0, 3.0);
        calc.clear_memory();
        assert_eq!(calc.recall_memory(), 0.0);
    }

    // ==================
    // Parameterized Tests
    // ==================

    #[test]
    fn test_add_multiple_cases() {
        let cases = vec![
            (1.0, 2.0, 3.0),
            (0.0, 0.0, 0.0),
            (-1.0, 1.0, 0.0),
            (100.0, -50.0, 50.0),
        ];

        let mut calc = Calculator::new();
        for (a, b, expected) in cases {
            assert_eq!(
                calc.add(a, b),
                expected,
                "Failed for {} + {} = {}",
                a,
                b,
                expected
            );
        }
    }

    // ==================
    // Mock Tests
    // ==================

    struct MockDataStore {
        get_returns: Option<String>,
        set_called_with: Option<(String, String)>,
    }

    impl MockDataStore {
        fn new() -> Self {
            MockDataStore {
                get_returns: None,
                set_called_with: None,
            }
        }

        fn returning(mut self, value: &str) -> Self {
            self.get_returns = Some(value.to_string());
            self
        }
    }

    impl DataStore for MockDataStore {
        fn get(&self, _key: &str) -> Option<String> {
            self.get_returns.clone()
        }

        fn set(&mut self, key: &str, value: &str) {
            self.set_called_with = Some((key.to_string(), value.to_string()));
        }
    }

    #[test]
    fn test_user_service_with_mock() {
        let mock = MockDataStore::new().returning("Bob");
        let service = UserService::new(mock);

        let name = service.get_user_name("123");
        assert_eq!(name, Some("Bob".to_string()));
    }

    #[test]
    fn test_user_service_sets_correctly() {
        let mock = MockDataStore::new();
        let mut service = UserService::new(mock);
        service.set_user_name("456", "Alice");

        // Note: With mock we'd verify it was called
        // This simplified version just shows the pattern
    }

    // ==================
    // Fixture Tests
    // ==================

    #[test]
    fn test_with_fixture() {
        let ctx = TestContext::new();

        for (a, b) in &ctx.test_values {
            let mut calc = Calculator::new();
            let result = calc.add(*a, *b);
            assert_eq!(result, a + b);
        }
    }

    #[test]
    fn test_fixture_with_memory() {
        let ctx = TestContext::new().with_memory(100.0);
        assert_eq!(ctx.calculator.recall_memory(), 100.0);
    }

    // ==================
    // Error Handling Tests
    // ==================

    #[test]
    fn test_result_ok() {
        let mut calc = Calculator::new();
        let result = calc.divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_result_err() {
        let mut calc = Calculator::new();
        let result = calc.divide(10.0, 0.0);
        assert!(result.is_err());
    }

    // ==================
    // Floating Point Tests
    // ==================

    #[test]
    fn test_floating_point_approximate() {
        let mut calc = Calculator::new();
        let result = calc.divide(1.0, 3.0).unwrap();

        // Don't use exact equality for floats
        let expected = 0.333333;
        assert!((result - expected).abs() < 0.001);
    }
}

// ==================
// Property-Based Tests (with proptest)
// ==================

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        // Addition is commutative
        #[test]
        fn add_commutative(a in -1000.0..1000.0f64, b in -1000.0..1000.0f64) {
            let mut calc = Calculator::new();
            let ab = calc.add(a, b);
            let ba = calc.add(b, a);
            prop_assert!((ab - ba).abs() < f64::EPSILON);
        }

        // Addition identity
        #[test]
        fn add_identity(a in -1000.0..1000.0f64) {
            let mut calc = Calculator::new();
            let result = calc.add(a, 0.0);
            prop_assert!((result - a).abs() < f64::EPSILON);
        }

        // Subtraction inverse of addition
        #[test]
        fn subtract_inverse(a in -1000.0..1000.0f64, b in -1000.0..1000.0f64) {
            let mut calc = Calculator::new();
            let added = calc.add(a, b);
            let subtracted = calc.subtract(added, b);
            prop_assert!((subtracted - a).abs() < 0.0001);
        }

        // Division by non-zero always succeeds
        #[test]
        fn divide_nonzero_succeeds(
            a in -1000.0..1000.0f64,
            b in prop::num::f64::NORMAL.prop_filter("non-zero", |x| x.abs() > 0.0001)
        ) {
            let mut calc = Calculator::new();
            let result = calc.divide(a, b);
            prop_assert!(result.is_ok());
        }

        // Memory always reflects last operation
        #[test]
        fn memory_reflects_last_operation(a in -1000.0..1000.0f64, b in -1000.0..1000.0f64) {
            let mut calc = Calculator::new();
            let result = calc.add(a, b);
            prop_assert!((calc.recall_memory() - result).abs() < f64::EPSILON);
        }
    }

    // String property tests
    proptest! {
        #[test]
        fn string_reverse_twice_is_identity(s in "\\PC*") {
            let reversed: String = s.chars().rev().collect();
            let double_reversed: String = reversed.chars().rev().collect();
            prop_assert_eq!(s, double_reversed);
        }

        #[test]
        fn vec_sort_is_idempotent(mut v in prop::collection::vec(0..100i32, 0..100)) {
            v.sort();
            let sorted = v.clone();
            v.sort();
            prop_assert_eq!(v, sorted);
        }
    }
}

// ==================
// Doc Tests Example
// ==================

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let mut calc = testing_patterns::Calculator::new();
/// assert_eq!(calc.add(2.0, 2.0), 4.0);
/// ```
///
/// # Edge Cases
///
/// ```
/// let mut calc = testing_patterns::Calculator::new();
/// assert_eq!(calc.add(0.0, 0.0), 0.0);
/// assert_eq!(calc.add(-1.0, 1.0), 0.0);
/// ```
pub fn documented_add(calc: &mut Calculator, x: f64, y: f64) -> f64 {
    calc.add(x, y)
}
