//! A Result type which is FFI safe.

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

/// All API functions which can fail return this type.
///
/// We don't use the `Result` type from the standard library because that isn't
/// FFI safe and may change layout between compiler versions.
#[repr(C)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum FfiResult<T, E> {
    /// The operation succeeded (like [`core::result::Result::Ok`]).
    Ok(T),
    /// The operation failed (like [`core::result::Result::Err`]).
    Err(E),
}

impl<T, E> FfiResult<T, E> {
    /// Obtain the inner value, or panic - just like `core::Result::unwrap`.
    pub fn unwrap(self) -> T
    where
        E: core::fmt::Debug,
    {
        let r: core::result::Result<T, E> = self.into();
        r.unwrap()
    }
}

impl<T, E> core::clone::Clone for FfiResult<T, E>
where
    T: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Ok(arg0) => Self::Ok(arg0.clone()),
            Self::Err(arg0) => Self::Err(arg0.clone()),
        }
    }
}

impl<T, E> From<core::result::Result<T, E>> for crate::FfiResult<T, E> {
    fn from(value: core::result::Result<T, E>) -> Self {
        match value {
            core::result::Result::Ok(x) => crate::FfiResult::Ok(x),
            core::result::Result::Err(x) => crate::FfiResult::Err(x),
        }
    }
}

impl<T, E> From<crate::FfiResult<T, E>> for core::result::Result<T, E> {
    fn from(value: crate::FfiResult<T, E>) -> Self {
        match value {
            crate::FfiResult::Ok(x) => core::result::Result::Ok(x),
            crate::FfiResult::Err(x) => core::result::Result::Err(x),
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
    fn make_result() {
        let native: core::result::Result<i32, ()> = Ok(1234);
        let ffi: FfiResult<i32, ()> = native.into();
        println!("ffi = {:?}", ffi);
    }

    #[test]
    #[should_panic]
    fn result_unwrap() {
        let value: FfiResult<i32, ()> = FfiResult::Err(());
        let _x = value.unwrap();
    }
}

// ============================================================================
// End of File
// ============================================================================
