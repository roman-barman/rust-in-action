# Code examples from "Rust in Action"

This repository contains code examples from the book **"Rust in Action"** by Tim McNamara. The projects are organized by chapter, covering various aspects of Rust programming from systems programming to high-level applications.

## Chapter Overview

### Chapter 1: Introducing Rust
*   **hello2**: A basic "Hello, world!" example.
*   **penguins**: A simple CSV parser that processes penguin data.

### Chapter 2: Rust Foundations
*   **grep-lite**: A simplified version of the `grep` command-line tool using regex and `clap`.
*   **mandelbrot**: A terminal-based Mandelbrot set fractal renderer.

### Chapter 3: Compound Data Types
*   **file-reader**: A simulation of a file system, demonstrating custom types and error handling.

### Chapter 5: Data in Depth
*   **cpu**: A simulation of a CPU (inspired by CHIP-8) that executes opcodes from memory.
*   **f32-visualizing**: Tools for visualizing how `f32` floating-point numbers are stored in memory.
*   **q7**: An exploration of fixed-point numbers.

### Chapter 6: Memory
*   **memory**: Examples and experiments related to memory management and raw pointers.

### Chapter 7: Files and Storage
*   **actionkv**: A simple key-value store with both in-memory and on-disk storage backends.

### Chapter 8: Networking
*   **dns-resolver**: A custom DNS resolver that sends UDP queries to DNS servers.
*   **macgen**: A utility for generating random MAC addresses.
*   **mget**: A hand-rolled HTTP client built from the ground up using `smoltcp`.

### Chapter 9: Time and Timekeeping
*   **clock**: A utility to get and set the system time, including an NTP client implementation.

### Chapter 10: Processes, Threads, and Containers
*   **render-hex**: A multi-threaded SVG generator that converts input bytes into a visual "sketch".

### Chapter 11: Kernel
*   **fledgeos**: A bare-metal "Hello World" operating system kernel written in Rust.

---

## How to use

Each project is a standalone Cargo package. To run any of them, navigate to the project directory and use `cargo run`.

```bash
cd src/ch2/grep-lite
cargo run -- "search_pattern" filename.txt
```