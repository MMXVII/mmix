//! Encapsulates the behavior of an MMIX virtual machine including the instruction set.
//!
//! This modules provides any functionality that belongs to the behavior of an MMIX. This includes
//! the instruction set which can be found in a submodule called `is` and code that emulates the
//! control unit (including the von Neumann cycle or a comparable architecture) grouped in a
//! submodule `cu`.

pub mod cu;
pub mod is;
