//! Types representing immutable borrowed byte slices.

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

/// A Rust u8 slice, but compatible with FFI. Assume the lifetime is only valid
/// until the callee returns to the caller.
#[repr(C)]
#[derive(Clone)]
pub struct FfiByteSlice<'a> {
    /// A pointer to the data
    pub data: *const u8,
    /// The number of bytes we are pointing at
    pub data_len: usize,
    /// A phantom object to hold the lifetime
    _phantom: core::marker::PhantomData<&'a [u8]>,
}

impl<'a> FfiByteSlice<'a> {
    /// Create a new byte slice we can send over the FFI.
    pub fn new(s: &'a [u8]) -> FfiByteSlice<'a> {
        FfiByteSlice {
            data: s.as_ptr(),
            data_len: s.len(),
            _phantom: core::marker::PhantomData,
        }
    }

    /// Make an empty slice.
    pub fn empty() -> FfiByteSlice<'static> {
        static EMPTY: &[u8] = &[];
        FfiByteSlice::new(EMPTY)
    }

    /// Turn this byte slice into a Rust byte slice.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data, self.data_len) }
    }
}

impl core::fmt::Debug for FfiByteSlice<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let slice = self.as_slice();
        write!(f, "[ ")?;
        if let Some((last, rest)) = slice.split_last() {
            for i in rest.iter() {
                write!(f, "0x{:02x}, ", i)?;
            }
            write!(f, "0x{:02x} ", last)?;
        }
        write!(f, "]")
    }
}

impl<'a> From<&'a [u8]> for FfiByteSlice<'a> {
    /// Convert from a Rust byte slice into an FFI compatible byte slice
    fn from(input: &'a [u8]) -> FfiByteSlice<'a> {
        FfiByteSlice::new(input)
    }
}

impl<'a> core::cmp::PartialEq for FfiByteSlice<'a> {
    /// Check if two ApiByteSlices are equal.
    ///
    /// We just make some actual slices and compare then.
    fn eq(&self, rhs: &Self) -> bool {
        if self.data_len != rhs.data_len {
            return false;
        }
        let this_slice = self.as_slice();
        let that_slice = rhs.as_slice();
        this_slice == that_slice
    }
}

impl<'a> core::cmp::Eq for FfiByteSlice<'a> {}

impl<'a> core::cmp::Ord for FfiByteSlice<'a> {
    /// Compare two ApiByteSlices.
    ///
    /// We just make some actual slices and compare then.
    fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
        let this_slice = self.as_slice();
        let that_slice = rhs.as_slice();
        this_slice.cmp(that_slice)
    }
}

impl<'a> core::cmp::PartialOrd for FfiByteSlice<'a> {
    /// Compare two ApiByteSlices.
    ///
    /// We are `Ord` so we can defer to that.
    fn partial_cmp(&self, rhs: &Self) -> core::option::Option<core::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_slices() {
        let data1 = &[1, 2, 3, 4];
        let data2 = &[1, 2, 3, 4];
        let data3 = vec![1, 2, 3, 5];
        let slice1 = FfiByteSlice::new(data1);
        let slice2 = FfiByteSlice::new(data2);
        let slice3 = FfiByteSlice::new(&data3);
        assert_eq!(slice1, slice2);
        assert_ne!(slice1, slice3);
        // This should be a compile failure because we drop the data source and
        // use the ApiByteSlice again.
        // drop(data3);
        assert!(slice1 < slice3);
    }
}

// ============================================================================
// End of File
// ============================================================================
