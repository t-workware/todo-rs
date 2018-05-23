use std::ffi::OsStr;

pub use todo::lang::*;

pub type Str = &'static str;

pub trait OsStrX {
    fn from_bytes(bytes: &[u8]) -> &Self;
    fn as_bytes(&self) -> &[u8];
    fn as_str(&self) -> &str;
    fn split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr);
}

impl OsStrX for OsStr {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> &Self {
        use std::mem;
        unsafe { mem::transmute(bytes) }
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()

    }

    #[inline]
    fn as_str(&self) -> &str {
        self.to_str().expect("unexpected invalid UTF-8 code point")
    }

    fn split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr) {
        let bytes = self.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            if b == &byte {
                return (
                    OsStr::from_bytes(&bytes[..i]),
                    OsStr::from_bytes(&bytes[i + 1..]),
                );
            }
        }
        (
            OsStr::new(""),
            &*self,
        )
    }
}