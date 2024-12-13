# Off-by-one Error in Rust Loop

This repository demonstrates a common off-by-one error in Rust and provides a solution.

The original code (`bug.rs`) uses a `loop` and index to iterate over a vector. However, it doesn't properly handle the case where the vector is empty or when the index reaches the end of the vector. This leads to a runtime panic.

The solution (`bugSolution.rs`) demonstrates a safer and more idiomatic way to iterate over vectors in Rust, using iterators to avoid index management and potential out-of-bounds errors.