//! FFI-safe types used in the various Neotron APIs.
//!
//! Note that all types in this file that are exported in the `Api` structure
//! *must* be `#[repr(C)]` and ABI stable.

use crate::slice::FfiByteSlice;

// ============================================================================
// Imports
// ============================================================================

// None

// ============================================================================
// Constants
// ============================================================================

// None

// ============================================================================
// Types
// ============================================================================

/// A Rust UTF-8 string, but compatible with FFI.
///
/// Assume the lifetime is only valid until the callee returns to the caller. Is
/// not null-terminated.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FfiString<'a>(FfiByteSlice<'a>);

impl<'a> FfiString<'a> {
    /// Create a new string slice we can send over the FFI.
    pub fn new(s: &'a str) -> FfiString<'a> {
        FfiString(FfiByteSlice::new(s.as_bytes()))
    }

    /// Turn this FFI string into a Rust string slice.
    pub fn as_str(&'a self) -> &'a str {
        unsafe { core::str::from_utf8_unchecked(self.0.as_slice()) }
    }
}

impl<'a> From<&'a str> for FfiString<'a> {
    /// Create a new FFI string from a string slice.
    fn from(input: &'a str) -> FfiString<'a> {
        FfiString::new(input)
    }
}

impl core::fmt::Debug for FfiString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
        let s = unsafe { core::str::from_utf8_unchecked(buffer) };
        write!(f, "{:?}", s)
    }
}

impl core::fmt::Display for FfiString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let buffer = unsafe { core::slice::from_raw_parts(self.0.data, self.0.data_len) };
        let s = unsafe { core::str::from_utf8_unchecked(buffer) };
        write!(f, "{}", s)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_string() {
        let s: FfiString = "Hello, world!".into();
        let output = s.to_string();
        assert_eq!(&output, "Hello, world!");
    }
}

// ============================================================================
// End of File
// ============================================================================
