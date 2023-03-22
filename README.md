# Sparse

A sparse matrix computation crate. This is early days of a practice project.

# Objectives

- Support for compressed column (CSC) operations.
- Generic scalar support (to be decided : f32, f64, complex, dense matrices...)
- Support for seemless Symbolic computations --- little symbolic-specific code (using the SymbolicScalar)
- Inspired by Tim Davis's "Direct Methodes for Sparse linear systems"
- Data compatibility with other CSC crates or C/C++ libs.

# Performance

The crate tries to be competitive with other librairies in the performance sense. Use of parallel computation is encouraged. Use of GPU computation is to be decided.

# Determinism

Execution is deterministic. (This might become a challenging requirement with GPU / parallel computation)

# Non-goals

These are subject to discussions

- Support for many representations : the core representation of the matrix should be CSC. Other reprensentations (COO...) may be introduced for construction of the matrices, but with little to no linear algebra support.
- Ultra-generic storage. Crates like `nalgebra` can use many kind of storage. `spare` will keep it simple and support owned, contiguous storage (Vec<> for now).

