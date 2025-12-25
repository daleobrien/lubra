# CLAUDE.md

This file provides comprehensive guidance for producing high-quality code in the Lubra project. It is targeted at AI assistants (like Claude) but is also useful for human developers. Follow these principles, patterns, and best practices to maintain code quality and consistency.

## Table of Contents
- [Code Quality Principles](#code-quality-principles)
- [Rust Best Practices](#rust-best-practices)
- [Project-Specific Patterns](#project-specific-patterns)
- [Error Handling](#error-handling)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Standards](#documentation-standards)
- [Performance Considerations](#performance-considerations)
- [Code Generation](#code-generation)
- [Refactoring Guidelines](#refactoring-guidelines)
- [Common Pitfalls](#common-pitfalls)

---

## Code Quality Principles

### Clarity Over Cleverness
- Write code that is immediately understandable to future maintainers
- Avoid clever tricks or obscure language features unless they provide significant benefit
- If bit manipulation is complex, add inline comments explaining the logic
- Use descriptive variable names even in short functions

### Minimal Complexity
- Keep functions focused on a single responsibility
- Break down complex operations into smaller, well-named functions
- Avoid deep nesting (prefer early returns and guard clauses)
- Maximum cognitive complexity per function should be manageable in a single reading

### Consistency
- Follow existing patterns in the codebase rigorously
- Match the style of surrounding code when making changes
- Use the same approach for similar problems across modules
- Don't introduce new patterns without strong justification

### Maintainability
- Make code easy to modify and extend
- Consider future developers who will read and change this code
- Leave the codebase better than you found it
- Refactor nearby code only if it directly supports your changes

---

## Rust Best Practices

### Type Safety
- Leverage Rust's type system to prevent bugs at compile time
- Use `Option<T>` and `Result<T, E>` instead of sentinel values or panics
- Make impossible states unrepresentable through type design
- Prefer `struct` types over tuples for anything beyond 2-3 related values

Example:
```rust
// Good: Clear types
struct TilePosition {
    i: i32,
    j: i32,
}

// Avoid: Unclear tuple
type TilePosition = (i32, i32);
```

### Ownership and Borrowing
- Prefer borrowing (`&T`, `&mut T`) over ownership transfer
- Use `.clone()` only when necessary and document why
- Avoid `Rc<RefCell<T>>` unless absolutely required for shared mutable state
- Design APIs to take the minimal ownership needed

### Error Handling
- Never use `.unwrap()` or `.expect()` in library code without justification
- Use `.unwrap()` only when panic is truly impossible (document why)
- Prefer `?` operator for propagating errors over explicit `match`
- Create custom error types for library crates using `thiserror` or similar

Example:
```rust
// Good: Propagate errors
fn process_tile(&self, pos: TilePosition) -> Result<Tile, TileError> {
    let neighbors = self.get_neighbors(pos)?;
    Ok(self.build_tile(neighbors))
}

// Avoid: Unwrap without justification
fn process_tile(&self, pos: TilePosition) -> Tile {
    let neighbors = self.get_neighbors(pos).unwrap(); // What if this fails?
    self.build_tile(neighbors)
}
```

### Iterator Patterns
- Use iterator chains instead of manual loops when clearer
- Prefer `.filter().map()` over `if` inside `.map()`
- Use `.collect()` to build collections, not manual loops
- Consider `.fold()` or `.reduce()` for aggregations

Example:
```rust
// Good: Clear iterator chain
let valid_tiles: Vec<_> = tiles
    .iter()
    .filter(|t| t.is_valid())
    .map(|t| t.to_rendered())
    .collect();

// Acceptable for complex logic:
let mut valid_tiles = Vec::new();
for tile in &tiles {
    if tile.is_valid() {
        valid_tiles.push(tile.to_rendered());
    }
}
```

### Memory Efficiency
- Use `Vec::with_capacity()` when final size is known
- Consider `&[T]` instead of `&Vec<T>` for function parameters
- Use `&str` instead of `&String` for string parameters
- Be mindful of copying large structures unnecessarily

### Const and Static
- Use `const` for compile-time constants (numbers, bit flags)
- Use `static` for runtime-initialized shared data
- Prefer `const fn` for functions used in const contexts
- Document the meaning and usage of bit flag constants

---

## Project-Specific Patterns

### Bit Manipulation Conventions

This project heavily uses bit manipulation for encoding tile states, node connections, and adjacent tile information.

#### Bit Flag Constants
- Node flags: `N1` through `N6` (bits 0-5) represent the 6 nodes of a hexagonal tile
- Inverted flag: `IN` (bit 30) indicates inverted rendering
- Wall flag: `WA` (bit 31) indicates wall tiles
- Adjacent node flags: `A1`-`A6`, `B1`-`B6`, etc. for neighboring tile nodes
- Masks: `NL` (node layer, bits 0-5), `NX` (extended bits 0-29)

#### Bit Operations
```rust
// Good: Clear bit operations with constants
let has_node_1 = tile & N1 != 0;
let nodes = tile & NL;  // Extract only node bits
let is_inverted = tile & IN != 0;

// Good: Combining multiple nodes
let nodes_1_and_3 = N1 | N3;
let connected = tile & nodes_1_and_3 == nodes_1_and_3;

// Add explanatory comments for complex operations
// Check if adjacent tile's node 4 connects to our node 1
let adj_connected = (adjacent & (A4 << 6)) != 0;
```

#### Hex Coordinate System
- Hexagonal tiles use `(i, j)` coordinate system with alternating row offsets
- Use `NODE_TO_COORD` lookup array for mapping node indices to adjacent tile positions
- Different coordinate mappings (`K0`-`K5`) handle the alternating hexagonal grid
- Always validate coordinates against grid boundaries

```rust
// Good: Use coordinate helper functions
let neighbors = self.get_adjacent_coordinates(tile_pos);

// Good: Validate bounds
if !self.is_valid_position(i, j) {
    return None;
}
```

### Net and Path Concepts
- A "net" is a connected path through tile nodes, represented as a bit pattern
- Nets can span multiple tiles via adjacent connections
- `to_bits()` collapses a vector of nets into a single bit pattern
- `get_mass()` calculates weight for tile selection (favors simpler patterns)
- Always check if nets intersect before merging with `interects()`

```rust
// Good: Working with nets
let combined_bits = to_bits(&nets);
if !interects(&nets) {
    // Safe to merge non-intersecting nets
    merge_nets(&mut nets);
}
```

### Code Generation Patterns

This project generates significant amounts of code:
- SVG path fragments in `lib/src/renders/fragment.rs`
- Net lookup tables in `lib/src/maze/nets_*.rs` and `mass_*.rs`
- Generated code is checked into version control
- Regenerate only when core tile logic changes

#### When Writing Generators
```rust
// Good: Clear separation of generation logic
pub fn generate_fragment_map() -> HashMap<u32, String> {
    let mut map = HashMap::new();

    // Generate for each valid pattern
    for pattern in valid_patterns() {
        let svg_path = compute_svg_path(pattern);
        map.insert(pattern, svg_path);
    }

    map
}

// Good: Use writer helper for file output
let code = generate_code();
writer::write_to_file("fragment.rs", &code)?;
```

### Module Organization
- Keep `mod.rs` files minimal - just re-exports
- One primary struct/type per file (exceptions for small helpers)
- Group related functionality: `maze/` for maze logic, `renders/` for output
- `common/` crate for shared utilities used across workspace

---

## Error Handling

### Library Code
```rust
// Good: Define error types
#[derive(Debug)]
pub enum MazeError {
    InvalidCoordinate { i: i32, j: i32 },
    NoValidTile { position: TilePosition },
    IntersectingNets { nets: Vec<u32> },
}

// Good: Return Results
pub fn place_tile(&mut self, pos: TilePosition) -> Result<(), MazeError> {
    if !self.is_valid_position(pos) {
        return Err(MazeError::InvalidCoordinate { i: pos.i, j: pos.j });
    }
    // ... rest of logic
}
```

### Application Code (CLI/Binaries)
```rust
// Acceptable: Unwrap with context in main
fn main() {
    let maze = Maze::new(100, 64)
        .expect("Failed to create maze");

    maze.generate()
        .expect("Failed to generate maze");
}

// Better: Handle errors gracefully
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maze = Maze::new(100, 64)?;
    maze.generate()?;
    Ok(())
}
```

### Validation
- Validate inputs at module boundaries
- Return errors for invalid states rather than panicking
- Document preconditions in function documentation
- Use debug assertions for internal invariants

```rust
// Good: Validate at boundary
pub fn set_tile(&mut self, i: i32, j: i32, tile: u32) -> Result<(), MazeError> {
    if !self.is_valid_position(i, j) {
        return Err(MazeError::InvalidCoordinate { i, j });
    }

    // Internal invariant check in debug builds only
    debug_assert!(tile & !NX == 0, "Tile has invalid high bits set");

    self.tiles[self.index(i, j)] = tile;
    Ok(())
}
```

---

## Testing Guidelines

### Unit Tests
- Test each function's core logic independently
- Use `#[cfg(test)]` module at the bottom of each file
- Name tests clearly: `test_<functionality>_<scenario>`
- Include edge cases, boundary conditions, and error cases

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bits_empty_nets() {
        let nets: Vec<u32> = vec![];
        assert_eq!(to_bits(&nets), 0);
    }

    #[test]
    fn test_to_bits_single_net() {
        let nets = vec![N1 | N3];
        assert_eq!(to_bits(&nets), N1 | N3);
    }

    #[test]
    fn test_to_bits_multiple_nets() {
        let nets = vec![N1, N3, N5];
        assert_eq!(to_bits(&nets), N1 | N3 | N5);
    }

    #[test]
    fn test_interects_returns_false_for_non_overlapping() {
        let nets = vec![N1, N3, N5];
        assert!(!interects(&nets));
    }

    #[test]
    fn test_interects_returns_true_for_overlapping() {
        let nets = vec![N1 | N2, N2 | N3];
        assert!(interects(&nets));
    }
}
```

### Integration Tests
- Place in `tests/` directory at crate root
- Test module interactions and public APIs
- Validate generated code works correctly
- Test maze generation with known seeds for reproducibility

```rust
// tests/maze_generation.rs
use lubra_lib::maze::Maze;

#[test]
fn test_maze_generation_reproducible() {
    let seed = 42;
    let maze1 = Maze::with_seed(10, 10, seed);
    let maze2 = Maze::with_seed(10, 10, seed);
    assert_eq!(maze1.to_svg(), maze2.to_svg());
}
```

### Property-Based Tests
- Use `quickcheck` or `proptest` for testing invariants
- Validate bit operations maintain expected properties
- Test that all generated mazes are valid

### Test Data
- Use small, hand-crafted examples for unit tests
- Use generated examples for integration tests
- Document what each test case is validating

---

## Documentation Standards

### Function Documentation
Use Rust doc comments (`///`) for all public items:

```rust
/// Converts a vector of net bit patterns into a single combined bit pattern.
///
/// This function performs a bitwise OR across all nets, combining them into
/// a single u32 that represents all active nodes.
///
/// # Arguments
/// * `nets` - A vector of u32 values, each representing a net's bit pattern
///
/// # Returns
/// A single u32 with all net bits combined, or 0 if the vector is empty
///
/// # Examples
/// ```
/// use common::bit_utils::to_bits;
/// use common::consts::{N1, N3, N5};
///
/// let nets = vec![N1, N3 | N5];
/// let combined = to_bits(&nets);
/// assert_eq!(combined, N1 | N3 | N5);
/// ```
pub fn to_bits(nets: &Vec<u32>) -> u32 {
    if nets.is_empty() {
        return 0u32;
    }
    nets.iter().fold(0, |a, b| a | b)
}
```

### Module Documentation
Add module-level documentation to `mod.rs` or top of file:

```rust
//! Bit manipulation utilities for hexagonal maze tiles.
//!
//! This module provides functions for working with bit-encoded tile states,
//! including net combination, mass calculation, and intersection detection.
//!
//! # Bit Encoding
//! - Bits 0-5: Node states (N1-N6)
//! - Bits 6-29: Adjacent tile node states
//! - Bit 30: Inverted flag (IN)
//! - Bit 31: Wall flag (WA)
```

### Inline Comments
- Explain "why" not "what" (code shows what)
- Document non-obvious algorithms or bit manipulations
- Explain constants and magic numbers
- Add TODO/FIXME comments for known issues

```rust
// Good: Explains why
// We multiply by 20 for simple 2-node nets because they're
// the most common pattern and we want them to appear frequently
match net.count_ones() {
    2 => mass *= 20,  // Favor simple connections
    3 => mass *= 5,   // Allow some complexity
    4 => mass *= 2,   // Discourage overly complex tiles
    _ => mass *= 1,
}

// Avoid: States the obvious
// Multiply mass by 20 for nets with 2 bits set
```

### Complex Algorithm Documentation
For complex algorithms, add block comments:

```rust
/// Reduces the maze by merging small nets into larger ones.
///
/// Algorithm:
/// 1. Sort nets by length (shortest first)
/// 2. Remove nets below minimum threshold
/// 3. Find intersection points (tiles where 2+ nets meet)
/// 4. For each intersection with exactly 2 connections:
///    - Attempt to merge the connecting nets
///    - Update intersection mappings
/// 5. Regenerate visible/hidden net assignments
///
/// This process simplifies the maze by creating longer paths
/// and reducing the total number of distinct nets.
```

---

## Performance Considerations

### Critical Performance Paths

#### Maze Generation
- Primary bottleneck: tile selection validation
- Use pre-generated lookup tables (`nets_*.rs`, `mass_*.rs`)
- Avoid runtime calculation of valid tile patterns
- Shuffle tiles in batches to reduce allocation

#### SVG Rendering
- Use glyph registry to deduplicate fragments
- Generate SVG paths once, reference with `<use href>`
- Batch string operations rather than incremental concatenation
- Consider streaming for very large mazes

#### Net Operations
```rust
// Good: Pre-allocate when size is known
let mut visible_nets = Vec::with_capacity(net_count);

// Good: Use bit operations instead of iteration
let has_nodes = tile & NL != 0;

// Avoid: Repeated allocations
for _ in 0..100 {
    let mut v = Vec::new();  // Allocates every iteration
    // ...
}
```

### Memory Efficiency
- Reuse allocations where possible
- Use bit packing to reduce memory footprint
- Consider `Box<[T]>` for fixed-size arrays (no spare capacity)
- Profile before optimizing (use `cargo flamegraph` or similar)

### When to Optimize
1. First, make it work correctly
2. Add tests to ensure correctness
3. Profile to find actual bottlenecks
4. Optimize only proven hot paths
5. Validate that optimization helped

Don't prematurely optimize. The generated lookup tables and bit operations are already highly optimized.

---

## Code Generation

### Generator Organization
Located in `generate/` crate with multiple binaries:
- `generate_svg` - Creates SVG path fragments
- `generate_nets` - Creates net lookup tables

### Writing New Generators

#### Structure
```rust
// generate/src/my_generator.rs

/// Generates lookup table for X functionality
pub fn generate_x_table() -> HashMap<u32, Vec<u32>> {
    let mut table = HashMap::new();

    // 1. Enumerate all valid inputs
    for input in all_valid_inputs() {
        // 2. Compute output for each input
        let output = compute_output(input);

        // 3. Validate output meets invariants
        debug_assert!(validate_output(&output));

        // 4. Store in table
        table.insert(input, output);
    }

    table
}

/// Converts generated table to Rust source code
pub fn format_as_rust_code(table: &HashMap<u32, Vec<u32>>) -> String {
    let mut code = String::new();
    code.push_str("// Generated code - do not edit by hand\n\n");
    code.push_str("pub static TABLE: phf::Map<u32, &'static [u32]> = phf_map! {\n");

    for (key, values) in table {
        code.push_str(&format!("    0x{:08x} => &{:?},\n", key, values));
    }

    code.push_str("};\n");
    code
}
```

#### Best Practices
- Make generators deterministic (same input → same output)
- Validate generated code compiles before writing to file
- Use `cargo fmt` on generated output
- Include clear header comment marking code as generated
- Version generated files in git for easy diff review

#### Testing Generated Code
```rust
#[test]
fn test_generated_table_completeness() {
    // Ensure all expected patterns have entries
    for pattern in expected_patterns() {
        assert!(TABLE.contains_key(&pattern),
                "Missing entry for pattern 0x{:08x}", pattern);
    }
}

#[test]
fn test_generated_values_valid() {
    // Validate each entry meets invariants
    for (key, values) in TABLE.entries() {
        assert!(validate_values(key, values),
                "Invalid values for key 0x{:08x}", key);
    }
}
```

### Regeneration Workflow
```bash
# Full regeneration (when core logic changes)
./full.sh

# Regenerate only SVG fragments
cd generate && cargo run --bin generate_svg

# Regenerate only net tables
cd generate && cargo run --bin generate_nets

# Verify changes compile
cargo build --release

# Review diffs before committing
git diff lib/src/renders/fragment.rs
git diff lib/src/maze/nets_*.rs
```

---

## Refactoring Guidelines

### When to Refactor
- When adding a feature that duplicates existing logic
- When a function becomes too long (>100 lines is a warning sign)
- When cyclomatic complexity is high (deep nesting, many branches)
- When tests are difficult to write due to tight coupling
- When the same pattern appears 3+ times (DRY principle)

### When NOT to Refactor
- Don't refactor working generated code manually
- Don't refactor code you don't understand - study it first
- Don't refactor without tests to ensure behavior preservation
- Don't refactor "because it looks ugly" if it works and is clear

### Safe Refactoring Process
1. Ensure existing tests pass
2. Add tests for the code being refactored if missing
3. Make small, incremental changes
4. Run tests after each change
5. Commit working states frequently
6. Use compiler to guide changes (let types break first)

### Refactoring Patterns

#### Extract Function
```rust
// Before: Long function with embedded logic
pub fn process_maze(&mut self) {
    // ... 50 lines of setup ...

    // Validate neighbors
    let mut valid = true;
    for n in &neighbors {
        if n.i < 0 || n.i >= self.width {
            valid = false;
            break;
        }
        if n.j < 0 || n.j >= self.height {
            valid = false;
            break;
        }
    }

    // ... 50 more lines ...
}

// After: Extracted helper function
pub fn process_maze(&mut self) {
    // ... 50 lines of setup ...

    let valid = self.are_neighbors_valid(&neighbors);

    // ... 50 more lines ...
}

fn are_neighbors_valid(&self, neighbors: &[Coordinate]) -> bool {
    neighbors.iter().all(|n| {
        n.i >= 0 && n.i < self.width &&
        n.j >= 0 && n.j < self.height
    })
}
```

#### Replace Magic Numbers with Named Constants
```rust
// Before: Magic numbers
if tile & 0b111111 != 0 {
    // Process nodes
}

// After: Named constant
const NODE_MASK: u32 = 0b111111;
if tile & NODE_MASK != 0 {
    // Process nodes
}

// Even better: Use existing constant
if tile & NL != 0 {
    // Process nodes
}
```

#### Simplify Conditional Logic
```rust
// Before: Nested conditions
if condition_a {
    if condition_b {
        if condition_c {
            do_work();
        }
    }
}

// After: Early returns
if !condition_a {
    return;
}
if !condition_b {
    return;
}
if !condition_c {
    return;
}
do_work();
```

---

## Common Pitfalls

### Bit Manipulation Errors
```rust
// Wrong: Using equality instead of bit test
if tile & N1 == N1 {  // Correct
if tile == N1 {       // Wrong: only matches if ONLY N1 is set

// Wrong: Forgetting parentheses with mixed operations
if tile & N1 | N2 != 0  // Wrong: parsed as (tile & N1) | (N2 != 0)
if (tile & (N1 | N2)) != 0  // Correct

// Wrong: Shifting before masking
let nodes = (tile >> 6) & NL  // Wrong: shifts out node bits
let nodes = tile & NL         // Correct
```

### Coordinate System Confusion
```rust
// Wrong: Using rectangular grid logic for hexagonal grid
let adjacent = (i + 1, j + 1);  // Not how hex grids work

// Correct: Use coordinate helper
let adjacent = self.get_adjacent_coordinates(i, j, direction);

// Wrong: Assuming consistent neighbor offsets
let neighbor_offset = (1, 0);  // Offset changes based on tile position!

// Correct: Use NODE_TO_COORD lookup
let neighbor_offset = NODE_TO_COORD[tile_variant][direction];
```

### Iterator Pitfalls
```rust
// Wrong: Collecting unnecessarily
if tiles.iter().collect::<Vec<_>>().len() > 0 {  // Wasteful allocation

// Correct: Direct check
if tiles.iter().any(|_| true) {  // Or just !tiles.is_empty()

// Wrong: Cloning in iterator when unnecessary
tiles.iter().map(|t| t.clone()).filter(|t| t.is_valid())

// Correct: Borrow, then clone only what's needed
tiles.iter().filter(|t| t.is_valid()).cloned()
```

### Ownership Issues
```rust
// Wrong: Taking ownership unnecessarily
fn process_tile(tile: Tile) -> u32 {  // Consumes tile
    tile.count_nodes()
}

// Correct: Borrow instead
fn process_tile(tile: &Tile) -> u32 {  // Borrows tile
    tile.count_nodes()
}

// Wrong: Cloning to solve borrow checker errors
let tile = self.tile.clone();  // Avoid unless necessary
process(&tile);
more_processing(&tile);

// Correct: Structure code to avoid clone
let result = {
    let tile = &self.tile;
    process(tile)
};
more_processing(&self.tile);
```

### Floating Point Comparisons
```rust
// Wrong: Direct equality with floats
if x == 0.1 + 0.2 {  // Might not equal 0.3 due to floating point error

// Correct: Use epsilon comparison
const EPSILON: f64 = 1e-10;
if (x - 0.3).abs() < EPSILON {

// Better: Use integer math when possible
let x_cents = 10 + 20;  // 30 cents, no floating point error
```

### Error Handling
```rust
// Wrong: Silently ignoring errors
let _ = self.process_tile(pos);  // Error is dropped

// Correct: Handle or propagate
self.process_tile(pos)?;

// Wrong: Panic in library code
let tile = self.get_tile(pos).expect("tile must exist");

// Correct: Return Result
let tile = self.get_tile(pos)?;
```

---

## Pre-Submission Checklist

Before considering code complete:

- [ ] All new functions have documentation comments
- [ ] Public functions have usage examples in docs
- [ ] Tests cover normal cases, edge cases, and error cases
- [ ] No compiler warnings (`cargo clippy` passes)
- [ ] Code is formatted (`cargo fmt` applied)
- [ ] Complex bit operations have explanatory comments
- [ ] Error cases return `Result` rather than panicking
- [ ] No unnecessary `.clone()` or `.unwrap()` calls
- [ ] Generated code has been regenerated if needed
- [ ] Changes don't break existing tests
- [ ] Performance-critical paths haven't regressed
- [ ] Constants are used instead of magic numbers
- [ ] Coordinate system operations use helper functions

---

## Additional Resources

### Project Documentation
- `WARP.md` - Comprehensive project overview and architecture
- `lib/src/renders/fragment.rs` - Generated SVG fragments (reference, don't edit)
- `common/src/consts.rs` - All bit flag constants and coordinate mappings

### Rust Resources
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust fundamentals
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Best practices for public APIs
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/) - Common issues caught by clippy

### Tools
- `cargo clippy` - Lint for common mistakes and improvements
- `cargo fmt` - Format code according to Rust style guide
- `cargo test` - Run all tests
- `cargo doc --open` - Generate and view documentation
- `cargo expand` - View macro expansions (requires cargo-expand)

---

## Revision History

- 2025-12-23: Initial version - Comprehensive best practices guide
