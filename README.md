# basezip — Rust Compression & Archiving Tool

## Project Overview

`basezip` is a command-line compression and archiving utility written entirely in Rust.

The goal is to build a practical file compression tool from the ground up by implementing established compression algorithms and designing a custom archive format.

Rather than relying on existing compression libraries, this project focuses on understanding and implementing the fundamental techniques behind modern compression systems, including Run-Length Encoding (RLE), Huffman coding, LZ77, and DEFLATE.

The long-term objective is to create a utility capable of compressing individual files and entire directory structures into a single archive, then extracting them while preserving their original contents and structure.

## Project Goals

- Implement compression and decompression algorithms independently.
- Understand binary data representation, bit-level operations, and byte streams.
- Explore statistical and dictionary-based compression techniques.
- Design a custom archive format for storing files, directories, and metadata.
- Support recursive directory traversal and extraction.
- Build a reliable CLI with predictable behavior and meaningful error handling.
- Benchmark compression ratio, execution time, and memory usage.

## Learning & Implementation Roadmap

### Phase 1 — Binary I/O Foundations

**Goal:** Understand how compressed data is represented and manipulated at the byte and bit levels.

- [x] Implement binary file reading and writing.
- [x] Understand bytes, bits, bit ordering, and endianness.
- [x] Implement a bit reader and bit writer.
- [x] Handle EOF, buffering, and I/O errors.
- [x] Design basic round-trip tests.

### Phase 2 — Fundamental Compression Algorithms

**Goal:** Implement simple algorithms to understand how redundancy can be represented more efficiently.

**Run-Length Encoding (RLE)**

- [x] Implement RLE encoding.
- [x] Implement RLE decoding.
- [x] Handle edge cases and expansion scenarios.

**Huffman Coding**

- [x] Build a frequency table.
- [x] Construct a Huffman binary tree.
- [x] Generate variable-length prefix codes.
- [x] Implement bit-level encoding and decoding.
- [x] Store and reconstruct the Huffman tree or code lengths.

### Phase 3 — Dictionary-Based Compression

**Goal:** Understand how repeated sequences can be represented through references instead of duplicated data.

**LZ77**

- [ ] Understand sliding windows and backward references.
- [ ] Implement a basic LZ77 encoder.
- [ ] Implement an LZ77 decoder.
- [ ] Handle overlapping matches.
- [ ] Experiment with window sizes and match-finding strategies.

### Phase 4 — DEFLATE

**Goal:** Combine dictionary-based and statistical compression into a practical compression algorithm.

- [ ] Understand how DEFLATE combines LZ77 and Huffman coding.
- [ ] Implement DEFLATE block structures.
- [ ] Support fixed Huffman coding.
- [ ] Support dynamic Huffman coding.
- [ ] Implement DEFLATE decompression.
- [ ] Validate compatibility against existing DEFLATE implementations.

### Phase 5 — Custom Archive Format

**Goal:** Design a container format capable of storing multiple files and directories.

- [ ] Define archive headers and format versioning.
- [ ] Store file paths and metadata.
- [ ] Represent directory entries.
- [ ] Store compressed file contents.
- [ ] Support multiple compression methods.
- [ ] Add integrity verification and corruption detection.
- [ ] Preserve directory structure during extraction.

### Phase 6 — Filesystem & CLI Integration

**Goal:** Turn the compression engine into a usable command-line application.

Example interface:

```bash
# Compress a file
basezip compress document.txt

# Compress a directory recursively
basezip compress my-project/

# Extract an archive
basezip decompress document.txt
basezip decompress my-project.basezip

# List archive contents
basezip list my-project.basezip

# Inspect archive information
basezip info my-project.basezip
```

Implementation tasks:

- [ ] Implement recursive directory traversal.
- [ ] Preserve relative paths and file metadata where supported.
- [ ] Implement archive extraction.
- [ ] Handle existing files and conflicting paths safely.
- [ ] Provide useful CLI options and error messages.

### Phase 7 — Optimization & Reliability

**Goal:** Improve performance and make the utility reliable for real-world usage.

- [ ] Benchmark compression ratio and speed.
- [ ] Optimize LZ77 match searching.
- [ ] Improve memory efficiency and buffering.
- [ ] Test empty files, large files, and binary data.
- [ ] Test corrupted and truncated archives.
- [ ] Add property-based and fuzz testing.
- [ ] Compare results against established compression utilities.

## Technical Principles

- **Language:** Rust
- **Approach:** Implement algorithms independently before considering external dependencies.
- **Architecture:** Separate compression algorithms, archive handling, filesystem operations, and CLI logic.
- **Testing:** Verify round-trip correctness: decompression must reproduce the original input exactly.
- **Design:** Keep compression algorithms independent of filesystem and archive concerns.

## Final Objective

Build a standalone Rust compression and archiving utility that can:

1. Compress individual files and complete directory structures.
2. Implement and use multiple compression algorithms.
3. Store compressed data in a custom archive format.
4. Extract archives without losing file contents or directory structure.
5. Provide reliable performance, error handling, and integrity checks.

The project is intended as a systems-programming exercise focused on understanding how compression engines and archive utilities work internally—not simply wrapping existing compression libraries.
