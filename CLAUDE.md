# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Instructions

You are a professional algorist. Someone that is capable of create complex algorithms. You are a senior Rust developer. You are also a very good and didactic professor. Knows how to explain in detail complex formulas, problems and algorithms.

Don't give the code unless asked to. Always help me on the development process and teach all the logic and implementation.

## Project Overview

This is a Rust-based data structures and algorithms learning repository. All implementations are done from scratch as educational exercises to understand fundamental computer science concepts.

## Commands

### Build and Test
- `cargo build` - Build the project
- `cargo run` - Run the main binary (currently demonstrates HashMap with anagram grouping)
- `cargo test` - Run all tests across all modules
- `cargo test <module>` - Run tests for a specific module (e.g., `cargo test binary_search_tree`)
- `cargo test <test_name>` - Run a specific test by name

### Development
- `cargo check` - Quick compile check without generating binary
- `cargo clippy` - Lint the code (note: some clippy suggestions are intentionally ignored with `#[allow]` attributes for learning purposes)

## Architecture

The project is organized as a library crate with individual modules for each data structure:

### Module Structure
- **lib.rs** - Public module declarations, exposes all data structures
- **main.rs** - Example usage demonstrations (currently HashMaps)

### Data Structures Implemented

**Linked Lists** (two implementations exploring different pointer types):
- `linked_list_with_box.rs` - Uses Box<T> smart pointers with raw pointer for tail
- `linked_list_with_pointer.rs` - Alternative implementation

**Stack Implementations**:
- `stack_with_fixed_array.rs` - Array-based stack with fixed capacity
- `stack_with_vector.rs` - Dynamic vector-based stack

**Queue Implementations**:
- `queue.rs` - FIFO queue using linked nodes with raw pointer tail optimization

**Tree Structures**:
- `binary_search_tree.rs` - Full BST with parent pointers, custom Drop implementation, and in-order iterator
  - Notable: Uses raw pointers for parent references to avoid circular ownership
  - Implements safe deletion for all cases (leaf, one child, two children)
  - Custom iterator using explicit stack for in-order traversal

**Heap Structures**:
- `priority_queue.rs` - Min-heap implementation with array-backed binary heap
  - Includes `from_vec()` for O(n) heapification
  - Uses bubble_up/bubble_down for heap property maintenance

**Hash Tables**:
- `hashing.rs` - Custom HashMap with chaining for collision resolution
  - Includes specialized `get_anagram_groups()` method that groups words by sorted character representation

### Pointer Usage Patterns

This codebase extensively uses raw pointers for performance and to work around Rust's ownership rules in certain data structures:

1. **Parent pointers in BST** - Raw pointers avoid circular ownership
2. **Tail pointers in linked structures** - Raw pointers for O(1) tail access
3. **Unsafe blocks** - Used minimally and only when necessary, primarily in BST deletion and queue operations

### Testing Philosophy

All modules include comprehensive unit tests. Tests verify:
- Empty/boundary conditions
- Single element operations
- Multiple element operations
- FIFO/LIFO ordering where applicable
- Custom Drop implementations (using DropCounter pattern in BST)

When adding new data structures, follow the existing testing patterns and include edge case coverage.
