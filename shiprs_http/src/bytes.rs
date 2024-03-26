#[derive(Debug)]
pub(super) struct Bytes<'buf> {
    str: *const u8,
    end: *const u8,
    ptr: *const u8,
    _marker: std::marker::PhantomData<&'buf [u8]>,
}

impl<'buf> Bytes<'buf> {
    pub fn new(buf: &'buf [u8]) -> Self {
        let str = buf.as_ptr();
        let end = unsafe { str.add(buf.len()) };
        let ptr = str;

        Bytes {
            str,
            end,
            ptr,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        if self.ptr < self.end {
            Some(unsafe { *self.ptr })
        } else {
            None
        }
    }

    pub unsafe fn advance(&mut self, n: usize) {
        debug_assert!(self.ptr < self.end, "out of bounds");
        self.ptr = self.ptr.add(n);
    }

    pub unsafe fn bump(&mut self) {
        self.advance(1);
    }

    pub fn peek_n<U>(&'buf self) -> Option<U>
    where
        U: TryFrom<&'buf [u8]>,
    {
        let n = std::mem::size_of::<U>();
        self.as_ref().get(..n)?.try_into().ok()
    }

    #[inline]
    pub fn slice(&mut self) -> &'buf [u8] {
        // SAFETY: not moving position at all, so it's safe
        let slice = unsafe { slice_from_raw_parts(self.str, self.ptr) };
        self.commit();
        slice
    }

    #[inline]
    pub unsafe fn slice_skip(&mut self, n: usize) -> &'buf [u8] {
        debug_assert!(self.ptr.sub(n) >= self.str, "out of bounds");
        let slice = slice_from_raw_parts(self.str, self.ptr.sub(n));
        self.commit();
        slice
    }

    #[inline]
    pub fn commit(&mut self) {
        self.str = self.ptr
    }
}

impl<'buf> AsRef<[u8]> for Bytes<'buf> {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice_from_raw_parts(self.ptr, self.end) }
    }
}

unsafe fn slice_from_raw_parts<'a>(start: *const u8, end: *const u8) -> &'a [u8] {
    debug_assert!(start <= end, "out of bound");
    std::slice::from_raw_parts(start, end as usize - start as usize)
}

impl<'buf> Iterator for Bytes<'buf> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.ptr < self.end {
            unsafe {
                let byte = *self.ptr;
                self.bump();
                Some(byte)
            }
        } else {
            None
        }
    }
}
