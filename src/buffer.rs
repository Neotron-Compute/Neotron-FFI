//! Types representing mutable borrowed byte slices.

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

/// A Rust u8 mutable slice, but compatible with FFI. Assume the lifetime is
/// only valid until the callee returns to the caller.
#[repr(C)]
#[derive(Clone)]
pub struct FfiBuffer<'a> {
    /// A pointer to where the data can be put
    pub data: *mut u8,
    /// The maximum number of bytes we can store in this buffer
    pub data_len: usize,
    /// A phantom object to hold the lifetime
    _phantom: core::marker::PhantomData<&'a [u8]>,
}

impl<'a> FfiBuffer<'a> {
    /// Create a new buffer we can send over the FFI.
    ///
    /// This buffer is a mutable borrow of some storage space allocated
    /// elsewhere. If you are given this type in an API, assume it is only
    /// valid for as long as the function call you were given in it.
    pub fn new(s: &'a mut [u8]) -> FfiBuffer<'a> {
        FfiBuffer {
            data: s.as_mut_ptr(),
            data_len: s.len(),
            _phantom: core::marker::PhantomData,
        }
    }

    /// Make an empty slice.
    pub fn empty() -> FfiBuffer<'static> {
        FfiBuffer {
            data: core::ptr::null_mut(),
            data_len: 0,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Turn this buffer into a Rust byte slice.
    pub fn as_slice(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.data_len) }
        }
    }

    /// Turn this buffer into a Rust mutable byte slice.
    ///
    /// You will get `None` if the buffer is empty (i.e. has zero length).
    pub fn as_mut_slice(&mut self) -> core::option::Option<&mut [u8]> {
        if self.data.is_null() {
            None
        } else {
            Some(unsafe { core::slice::from_raw_parts_mut(self.data, self.data_len) })
        }
    }
}

impl core::fmt::Debug for FfiBuffer<'_> {
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

impl<'a> From<&'a mut [u8]> for FfiBuffer<'a> {
    /// Convert from a Rust byte slice into an FFI compatible byte slice
    fn from(input: &'a mut [u8]) -> FfiBuffer<'a> {
        FfiBuffer::new(input)
    }
}

impl<'a> core::cmp::PartialEq for FfiBuffer<'a> {
    /// Check if two ApiBuffers are equal.
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

impl<'a> core::cmp::Eq for FfiBuffer<'a> {}

impl<'a> core::cmp::Ord for FfiBuffer<'a> {
    /// Compare two ApiBuffers.
    ///
    /// We just make some actual slices and compare then.
    fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
        let this_slice = self.as_slice();
        let that_slice = rhs.as_slice();
        this_slice.cmp(that_slice)
    }
}

impl<'a> core::cmp::PartialOrd for FfiBuffer<'a> {
    /// Compare two ApiBuffers.
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
    fn make_buffers() {
        let data1 = &mut [1, 2, 3, 4];
        let data2 = &mut [1, 2, 3, 4];
        let mut data3 = vec![1, 2, 3, 5];
        let mut buffer1 = FfiBuffer::new(data1);
        let buffer2 = FfiBuffer::new(data2);
        let buffer3 = FfiBuffer::new(&mut data3);
        assert_eq!(buffer1, buffer2);
        assert_ne!(buffer1, buffer3);
        // This should be a compile failure because we drop the data source and
        // use the ApiBuffer again.
        // drop(data3);
        assert!(buffer1 < buffer3);

        let output1 = buffer1.as_mut_slice().unwrap();
        assert_eq!(output1, &[1, 2, 3, 4]);
    }
}

// ============================================================================
// End of File
// ============================================================================
