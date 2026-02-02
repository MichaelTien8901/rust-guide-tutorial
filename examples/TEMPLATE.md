# Example Template

Use this template when creating new example projects.

## Directory Structure

```
examples/partN/example-name/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs (or lib.rs for library examples)
```

## Cargo.toml Template

```toml
[package]
name = "example-name"
version = "0.1.0"
edition = "2021"
description = "Brief description of what this example demonstrates"
publish = false

# Link back to the tutorial
[package.metadata]
tutorial-chapter = "partN/chapter-name"

[dependencies]
# Add dependencies as needed
# Keep dependencies minimal for clarity

[dev-dependencies]
# Test dependencies if needed
```

## README.md Template

```markdown
# Example Name

Brief description of what this example demonstrates.

## Concepts Covered

- Concept 1
- Concept 2
- Concept 3

## Prerequisites

List any prerequisites or prior knowledge needed.

## Running the Example

\`\`\`bash
cargo run
\`\`\`

## Expected Output

\`\`\`
Expected output here
\`\`\`

## Code Walkthrough

Explain the key parts of the code:

1. **Step 1**: Explanation
2. **Step 2**: Explanation
3. **Step 3**: Explanation

## Exercises

Try these modifications to deepen understanding:

1. Exercise 1
2. Exercise 2

## Related Chapters

- [Chapter Name](link-to-chapter)
```

## src/main.rs Template

```rust
//! Example: Example Name
//!
//! This example demonstrates [concept being demonstrated].
//!
//! Run with: `cargo run`

fn main() {
    // Clear entry point with descriptive comments
    println!("Example: Example Name");
    println!("======================\n");

    // Main demonstration code
    // Break into logical sections with comments

    // Section 1: Setup
    // ...

    // Section 2: Main logic
    // ...

    // Section 3: Output/results
    // ...

    println!("\nExample complete!");
}

// Helper functions with documentation
/// Brief description of function
fn helper_function() {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Test the example works correctly
    }
}
```

## Guidelines

### Code Style

- Use clear, descriptive variable names
- Add comments explaining the "why", not just the "what"
- Follow Rust naming conventions (snake_case for functions/variables)
- Keep examples focused on a single concept
- Use `rustfmt` for consistent formatting

### Documentation

- Every public item should have documentation
- Include expected output in README
- Link to the relevant tutorial chapter
- Provide exercises for readers to try

### Testing

- Add at least one test per example
- Test the happy path at minimum
- Examples should compile and run without errors

### Dependencies

- Keep dependencies minimal
- Use well-known, maintained crates
- Pin versions for reproducibility
- Document why each dependency is needed

## Example Checklist

Before committing a new example:

- [ ] Cargo.toml has correct metadata
- [ ] README.md explains the example
- [ ] Code compiles without warnings (`cargo build`)
- [ ] Code passes clippy (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Tests pass (`cargo test`)
- [ ] Example runs successfully (`cargo run`)
- [ ] Documentation is complete
- [ ] Links to tutorial chapter work
