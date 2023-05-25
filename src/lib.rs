//! FFI-safe types used in the various Neotron APIs.
//!
//! Note that all types in this file that are exported in the `Api` structure
//! *must* be `#[repr(C)]` and ABI stable.

#![cfg_attr(not(test), no_std)]

// ============================================================================
// Imports
// ============================================================================

mod buffer;
mod option;
mod result;
mod slice;
mod string;

#[doc(inline)]
pub use result::FfiResult;

#[doc(inline)]
pub use string::FfiString;

#[doc(inline)]
pub use option::FfiOption;

#[doc(inline)]
pub use buffer::FfiBuffer;

#[doc(inline)]
pub use slice::FfiByteSlice;

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

// None

// ============================================================================
// Functions
// ============================================================================

// None

// ============================================================================
// End of File
// ============================================================================
