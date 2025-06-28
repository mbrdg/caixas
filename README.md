# Caixas

A Rust library for composing 2D ASCII layouts using functional combinators.
Build complex text visualizations by combining simple rectangular "boxes" of characters.

## Overview

Box combinators enable intuitive visual programming - solving layout problems feels like playing with LEGO pieces.
Create ASCII art, tables, and diagrams by combining simple text boxes.

## Quick Start

```rust
//! This is a simple example that produces the following output:
//! +-------------+
//! |Hello, World!|
//! +-------------+

use caixas::Box;

let greeting = Box::from("Hello, World!").framed();
print!("{greeting}");
```

## Core API

### Constructors
- `Box::from(s)` - Single-row box from string or a character
- `Box::fill(char, height, width)` - Box filled with character
- `Box::space(height, width)` - Box filled with spaces
- `Box::empty()` - Zero-dimension neutral element

### Combinators
- `Box::beside(left, right)` - Horizontal composition
- `Box::above(top, bottom)` - Vertical composition
- `Box::hconcat(boxes)` - Horizontal concatenation
- `Box::vconcat(boxes)` - Vertical concatenation
- `Box::grid(boxes)` - 2D array arrangement

### Utilities
- `Box::framed(box)` - Add ASCII border

## Alignment

All combinators support alignment options:
- Vertical: `Top`, `Center` (default), `Bottom`
- Horizontal: `Left`, `Center` (default), `Right`

## Examples

Complete examples are available under `examples/` and can be run with:

```bash
cargo run --example framed
cargo run --example sierpinski
cargo run --example table
cargo run --example spiral
```

## Installation

```toml
[dependencies]
caixas = "0.1.0"
```

## Use Cases

- Debug visualization of data structures
- ASCII art and fractals
- Terminal tables and reports
- Simple text-based UIs

## Credits

Ported from the OCaml implementation in ["Box Combinators"](https://mmapped.blog/posts/41-box-combinators).
Inspired by "Programming in Scala" text layout concepts.
