# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Lubra is a Rust-based maze generation and rendering system that creates complex geometric mazes using hexagonal tiles. The project generates SVG output of mazes with configurable properties and supports various net reduction algorithms to create more intricate patterns.

## Core Concepts

### Hexagonal Tile System
- Mazes are built from hexagonal tiles, each with 6 nodes (vertices)
- Tiles can have connections between their nodes, represented as bit patterns
- Each tile tracks which nodes are connected (forming "nets" - connected paths through the maze)
- Tile coordinates use a specialized hexagonal grid system (i, j) where positioning alternates

### Bit Representation
- Nodes and connections are encoded as 32-bit integers using bit flags
- Lower 6 bits represent the 6 nodes of a tile (N1-N6)
- Higher bits represent adjacent tile connections and state flags
- Complex paths can span multiple tiles and are encoded with up to 30 bits (5 adjacent tiles × 6 nodes)
- Constants like `N1`, `N2`, `N3`, `N4`, `N5`, `N6` define individual node flags
- `NL` (node layer) and `NX` masks are used for bit manipulation
- The `IN` flag indicates inverted rendering, `WA` flag indicates wall tiles

### Net System
- A "net" is a connected path through tiles
- Nets can be reduced/merged to create more complex patterns
- The system tracks visible and hidden nets separately for rendering optimization
- Generated lookup tables (`nets_large.rs`, `nets_small.rs`, `mass_large.rs`, `mass_small.rs`) map tile configurations to valid net patterns

### Fragment-Based Rendering
- Each unique tile configuration has a pre-generated SVG path fragment
- Fragments are stored in `MAP_PATHS` (a perfect hash map) keyed by bit pattern
- The `generate` workspace creates these fragments programmatically
- Fragments can be simple (single tile) or complex (spanning adjacent tiles)

## Workspace Structure

### `lib/` - Core Library
Main maze logic and rendering engine:
- `maze/maze.rs` - Core `Maze` struct with generation and net manipulation
- `maze/tile.rs` - Tile representation and path-finding logic
- `maze/adjacent.rs` - Adjacent tile connection logic
- `maze/random.rs` - Randomized tile selection
- `maze/nets_*.rs` - Generated lookup tables for valid net configurations
- `maze/mass_*.rs` - Generated mass/weight tables for tile selection
- `renders/fragment.rs` - Generated SVG path fragments (MAP_PATHS)
- `renders/svg.rs` - SVG document builder with layer system

### `cli/` - Command Line Interface
Executable that generates sample mazes:
- `main.rs` - Example maze generation code
- Creates mazes with various sizes and configurations
- Outputs to SVG format

### `generate/` - Code Generation
Builds lookup tables and SVG fragments:
- `main_svg.rs` - Generates SVG path fragments for all tile configurations
- `main_nets.rs` - Generates net lookup tables for tile combinations
- `frag_*.rs` - Individual fragment generators for different node patterns (1-6 nodes, various combinations)
- `svg_fragments.rs` - Orchestrates all fragment generation
- `path_adj.rs` - Generates adjacent path variations
- `writer.rs` - Writes generated code to source files

Fragment naming: `frag_15` means nodes 1 and 5 are connected, `frag_123` means 1-2-3 form a path, etc.

### `common/` - Shared Utilities
- `types/` - Core data structures (Coordinate, Point, bits)
- `consts.rs` - Bit flags and node coordinate mappings (NODE_TO_COORD)
- `bit_utils.rs` - Bit manipulation utilities
- `shapes/` - Geometric primitives (line, arc, rectangle)
- `svg/` - SVG element builders
- `node_combinations.rs` - Generates valid node combinations
- `macros/` - Code generation macros

### `document/` - Documentation Generation
Creates visual documentation of tile types:
- `create_svgs.rs` - Generates SVG examples
- `create_tiles.rs` - Creates tile grid documentation
- `create_typst.rs` - Generates Typst documentation
- `variants.rs` - All possible tile variants

### `test/` - Test Assets
Contains generated PNG maze examples (random-01.png through random-104.png)

## Common Development Commands

### Build the project
```bash
cargo build --release
```

### Run the CLI to generate mazes
```bash
# Quick run from CLI directory
cd cli && cargo run

# Or use helper script
./maze.sh
```

### Regenerate SVG fragments and lookup tables
```bash
# Generate both fragments and nets (full rebuild)
./full.sh

# Generate only SVG fragments
cd generate && cargo run --bin generate_svg

# Generate only net lookup tables
cd generate && cargo run --bin generate_nets
```

### Generate documentation visuals
```bash
cd document && cargo run
```

### Run combined generation and CLI
```bash
./both.sh  # Generates SVGs then runs maze CLI
```

### Run tests
```bash
cargo test
```

## Key Algorithms

### Maze Generation (`maze.rs`)
1. `create_random_grid()` - Randomly places tiles ensuring valid adjacency
2. Uses ChaCha8Rng for reproducible randomness with optional seed
3. Shuffles tile placement order to avoid bias from previous tiles
4. Validates each tile against its 12 neighbors (hexagonal grid)

### Net Reduction (`maze.rs`)
1. `reduce_nets()` - Merges small nets into larger ones
2. `sort_nets_by_length()` - Orders nets by size
3. `remove_short_nets()` - Removes nets below threshold
4. `get_intersections()` - Finds tiles where nets cross
5. Attempts to merge nets at intersection points with 2 connections

### Tile Selection (`tile.rs`)
- `find_best_path()` - Searches for matching adjacent tile patterns
- Uses `ADJ_CHECKS` lookup table for valid adjacent configurations
- `generate_hidden_and_visible_nets()` - Separates nets into rendering layers
- Returns `CataloguedNet` with visible nets and adjacent nets to hide

### SVG Rendering (`svg.rs`)
- Multi-layer rendering (6 layers) for depth control
- Glyph registry deduplicates repeated fragments
- Coordinate transformation flips Y-axis for correct rendering
- Uses `<use>` elements with href for efficient SVG output

## Important Implementation Details

### Coordinate System
- `NODE_TO_COORD` array maps node indices to 12 surrounding tile positions
- Different mappings (K0-K5) handle the alternating hexagonal grid offsets
- Coordinates are (column, row) pairs relative to a reference tile
- Adjacent tile checking requires understanding node-to-neighbor mapping

### Random Tile Generation
- `random_tile()` function uses weighted selection
- Validates against 12 surrounding tiles' exposed nodes
- Tiles must connect validly to neighbors without creating invalid patterns
- Uses generated lookup tables for fast validation

### Code Generation Workflow
1. `generate_svg` creates SVG fragments by computing geometric paths
2. Generates `lib/src/renders/fragment.rs` with MAP_PATHS hash map
3. `generate_nets` computes all valid net combinations for tile patterns
4. Generates `lib/src/maze/nets_*.rs` and `mass_*.rs` files
5. These generated files are checked into version control
6. Re-run generation only when tile logic or constraints change

### Rendering Pipeline
1. Maze generates tiles with net configurations (bit patterns)
2. For each tile, look up SVG fragment in MAP_PATHS
3. Handle adjacent tiles - some nets span multiple tiles
4. Layer visible and hidden fragments for proper overlap
5. Svg struct builds final document with glyphs and transformations

## Testing
Unit tests exist throughout the codebase for:
- Bit manipulation utilities (`bit_utils.rs`)
- Tile operations (`tile.rs`)
- Shape generation (`shapes/`)
- SVG builders (`svg/`)
- Maze generation (`maze.rs`)
- Mathematical utilities (`renders/math.rs`)

Run tests with `cargo test` from workspace root.

## Output Files
- CLI generates mazes in current directory as SVG files
- Naming pattern: `{width}-by-{height}-sample-normal-{seed}.svg`
- Example: `100-by-64-sample-normal-00.svg`

## Architecture Notes

### Why Generated Code?
- Pre-computing SVG fragments eliminates runtime path calculation
- Lookup tables enable O(1) validation of tile configurations
- Generated code is faster than dynamic computation
- Perfect hash maps (PHF) provide optimal lookup performance

### Hexagonal Grid Challenges
- Not a simple rectangular grid - requires special coordinate handling
- 12 adjacent tiles per position (not 8 like rectangular grids)
- Node connections must maintain consistency across tile boundaries
- Alternating row offsets require per-tile coordinate mapping (K0-K5)

### Net Merging Complexity
- Intersecting nets may have multiple valid merge options
- Simple merges (2 connections) are handled automatically
- Complex intersections (3+ connections) require manual resolution
- System tracks which adjacent nets to hide during rendering

## Performance Considerations
- Use `--release` builds for maze generation (significant speedup)
- Large mazes (>100x100) may take considerable time for net reduction
- The `seed` parameter ensures reproducible maze generation
- Generated lookup tables are large but necessary for performance