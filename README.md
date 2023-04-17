# Sparse

A sparse matrix computation crate. This is EARLY DAYS of a PRACTICE PROJECT.
Feel free to ask anything but do not expect ANY support nor garanties

# Objectives

- Support for compressed column (CSC) operations.
- Generic scalar support (to be decided : f32, f64, complex, dense matrices...)
- Support for seemless Symbolic computations --- little symbolic-specific code (using the SymbolicScalar)
- Inspired by Tim Davis's "Direct Methodes for Sparse linear systems"
- Raw data compatibility with other CSC crates or C/C++ libs.

# Performance

The crate tries to be competitive with other librairies in the performance sense. Use of parallel computation is encouraged. Use of GPU computation is to be decided.

# Determinism

Execution is deterministic. (This might become a challenging requirement with GPU / parallel computation)

# Non-goals

These are subject to discussions

- Support for many representations : the core representation of the matrix should be CSC. Specific optimizations around CSC might be implemented (for instance CSC5).
- Ultra-generic storage. Crates like `nalgebra` can use many kind of storage. `spa-rs` will keep it simple and support owned, contiguous storage (Vec<> for now).

