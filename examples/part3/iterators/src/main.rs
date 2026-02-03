//! Iterators Example
//!
//! Demonstrates iterator creation, adapters, and consumers.

fn main() {
    println!("=== Iterators ===\n");

    println!("--- Creating Iterators ---");
    creating_iterators();

    println!("\n--- Iterator Adapters ---");
    iterator_adapters();

    println!("\n--- Consuming Iterators ---");
    consuming_iterators();

    println!("\n--- Custom Iterator ---");
    custom_iterator();

    println!("\n--- Iterator Patterns ---");
    iterator_patterns();
}

fn creating_iterators() {
    let v = vec![1, 2, 3];

    // iter() - immutable references
    print!("iter(): ");
    for val in v.iter() {
        print!("{} ", val);
    }
    println!();
    println!("v still valid: {:?}", v);

    // iter_mut() - mutable references
    let mut v2 = vec![1, 2, 3];
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("After iter_mut(): {:?}", v2);

    // into_iter() - takes ownership
    let v3 = vec![1, 2, 3];
    print!("into_iter(): ");
    for val in v3.into_iter() {
        print!("{} ", val);
    }
    println!();
    // v3 is no longer valid

    // Ranges as iterators
    print!("Range: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // Inclusive range
    print!("Inclusive range: ");
    for i in 1..=3 {
        print!("{} ", i);
    }
    println!();
}

fn iterator_adapters() {
    let v = vec![1, 2, 3, 4, 5];

    // map - transform each element
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map (doubled): {:?}", doubled);

    // filter - keep elements matching predicate
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter (evens): {:?}", evens);

    // filter_map - filter and transform
    let strings = vec!["1", "two", "3", "four", "5"];
    let numbers: Vec<i32> = strings.iter().filter_map(|s| s.parse().ok()).collect();
    println!("filter_map: {:?}", numbers);

    // take and skip
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    let skip_two: Vec<&i32> = v.iter().skip(2).collect();
    println!("take(3): {:?}", first_three);
    println!("skip(2): {:?}", skip_two);

    // take_while and skip_while
    let until_four: Vec<&i32> = v.iter().take_while(|&&x| x < 4).collect();
    println!("take_while(< 4): {:?}", until_four);

    // enumerate - add indices
    for (i, val) in v.iter().enumerate() {
        println!("  Index {}: {}", i, val);
    }

    // zip - combine iterators
    let names = vec!["Alice", "Bob", "Carol"];
    let ages = vec![25, 30, 35];
    let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("zip: {:?}", combined);

    // chain - concatenate iterators
    let a = vec![1, 2];
    let b = vec![3, 4];
    let chained: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("chain: {:?}", chained);

    // flatten - flatten nested iterators
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<_> = nested.iter().flatten().collect();
    println!("flatten: {:?}", flat);

    // flat_map - map then flatten
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
    println!("flat_map: {:?}", chars);

    // rev - reverse
    let reversed: Vec<&i32> = v.iter().rev().collect();
    println!("rev: {:?}", reversed);

    // peekable - peek without consuming
    let mut iter = v.iter().peekable();
    if let Some(&first) = iter.peek() {
        println!("Peeked: {}", first);
    }
    println!("Next: {:?}", iter.next());
}

fn consuming_iterators() {
    let v = vec![1, 2, 3, 4, 5];

    // collect - gather into collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("collect: {:?}", doubled);

    // sum and product
    let sum: i32 = v.iter().sum();
    let product: i32 = v.iter().product();
    println!("sum: {}, product: {}", sum, product);

    // count
    let count = v.iter().count();
    println!("count: {}", count);

    // fold - reduce with accumulator
    let sum_fold = v.iter().fold(0, |acc, x| acc + x);
    println!("fold (sum): {}", sum_fold);

    let concat = vec!["Hello", " ", "World"]
        .iter()
        .fold(String::new(), |acc, s| acc + s);
    println!("fold (concat): {}", concat);

    // reduce - like fold but uses first element
    let sum_reduce = v.iter().copied().reduce(|a, b| a + b);
    println!("reduce: {:?}", sum_reduce);

    // find - first matching element
    let first_even = v.iter().find(|&&x| x % 2 == 0);
    println!("find (first even): {:?}", first_even);

    // position - index of first match
    let pos = v.iter().position(|&x| x == 3);
    println!("position of 3: {:?}", pos);

    // any and all
    let has_even = v.iter().any(|&x| x % 2 == 0);
    let all_positive = v.iter().all(|&x| x > 0);
    println!("any even: {}, all positive: {}", has_even, all_positive);

    // min and max
    let min = v.iter().min();
    let max = v.iter().max();
    println!("min: {:?}, max: {:?}", min, max);

    // min_by and max_by
    let words = vec!["a", "abc", "ab"];
    let longest = words.iter().max_by(|a, b| a.len().cmp(&b.len()));
    println!("longest word: {:?}", longest);

    // nth
    let third = v.iter().nth(2);
    println!("nth(2): {:?}", third);

    // last
    let last = v.iter().last();
    println!("last: {:?}", last);
}

// Custom iterator
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn custom_iterator() {
    let counter = Counter::new(5);
    let values: Vec<u32> = counter.collect();
    println!("Counter: {:?}", values);

    // Using iterator methods on custom iterator
    let sum: u32 = Counter::new(5).sum();
    println!("Counter sum: {}", sum);

    // Chaining with other iterators
    let product: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Complex counter operation: {}", product);
}

fn iterator_patterns() {
    // Processing lines
    let text = "line 1\nline 2\nline 3";
    let lines: Vec<&str> = text.lines().collect();
    println!("Lines: {:?}", lines);

    // Splitting
    let csv = "a,b,c,d";
    let fields: Vec<&str> = csv.split(',').collect();
    println!("CSV fields: {:?}", fields);

    // Windowing
    let numbers = vec![1, 2, 3, 4, 5];
    let windows: Vec<_> = numbers.windows(2).collect();
    println!("Windows(2): {:?}", windows);

    // Chunking
    let chunks: Vec<_> = numbers.chunks(2).collect();
    println!("Chunks(2): {:?}", chunks);

    // Partitioning
    let (evens, odds): (Vec<&i32>, Vec<&i32>) = numbers.iter().partition(|&&x| x % 2 == 0);
    println!("Evens: {:?}, Odds: {:?}", evens, odds);

    // Grouping with fold
    let words = vec!["apple", "banana", "apricot", "blueberry"];
    let mut grouped: std::collections::HashMap<char, Vec<&str>> = std::collections::HashMap::new();
    for word in words {
        let first = word.chars().next().unwrap();
        grouped.entry(first).or_default().push(word);
    }
    println!("Grouped by first letter: {:?}", grouped);

    // Unzip
    let pairs = vec![(1, "a"), (2, "b"), (3, "c")];
    let (numbers, letters): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    println!("Unzipped: {:?} and {:?}", numbers, letters);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_filter() {
        let v = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = v.iter().map(|x| x * 2).filter(|x| *x > 4).collect();
        assert_eq!(result, vec![6, 8, 10]);
    }

    #[test]
    fn test_fold() {
        let v = vec![1, 2, 3, 4, 5];
        let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_custom_iterator() {
        let counter = Counter::new(3);
        let v: Vec<u32> = counter.collect();
        assert_eq!(v, vec![1, 2, 3]);
    }
}
