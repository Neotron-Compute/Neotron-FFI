//! FFI-safe types used in the various Neotron APIs.
//!
//! Note that all types in this file that are exported in the `Api` structure
//! *must* be `#[repr(C)]` and ABI stable.

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

/// All API functions which take/return optional values return this type.
///
/// We don't use the `Option` type from the standard library because that isn't
/// FFI safe and may change layout between compiler versions.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiOption<T> {
    /// There is some data (the same as `core::option::Option::Some`)
    Some(T),
    /// There is no data (the same as `core::option::Option::None`)
    None,
}

impl<T> FfiOption<T> {
    /// Obtain the inner value, or panic - just like `core::Option::unwrap`.
    pub fn unwrap(self) -> T {
        let o: core::option::Option<T> = self.into();
        o.unwrap()
    }
}

impl<T> From<core::option::Option<T>> for crate::FfiOption<T> {
    fn from(value: core::option::Option<T>) -> Self {
        match value {
            core::option::Option::Some(x) => crate::FfiOption::Some(x),
            core::option::Option::None => crate::FfiOption::None,
        }
    }
}

impl<T> From<crate::FfiOption<T>> for core::option::Option<T> {
    fn from(value: crate::FfiOption<T>) -> core::option::Option<T> {
        match value {
            crate::FfiOption::Some(x) => core::option::Option::Some(x),
            crate::FfiOption::None => core::option::Option::None,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_option() {
        let native: core::option::Option<i32> = Some(1234);
        let ffi: FfiOption<i32> = native.into();
        println!("ffi = {:?}", ffi);
    }

    #[test]
    #[should_panic]
    fn option_unwrap() {
        let value: FfiOption<i32> = FfiOption::None;
        let _x = value.unwrap();
    }
}

// ============================================================================
// End of File
// ============================================================================
