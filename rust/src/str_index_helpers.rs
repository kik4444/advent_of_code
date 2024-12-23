pub trait ToStrUnchecked {
    fn to_str_unchecked(&self) -> &str;
}

impl<T> ToStrUnchecked for T
where
    T: ?Sized + AsRef<[u8]>,
{
    // SAFETY: the caller must guarantee that the bytes `v` are valid UTF-8.
    fn to_str_unchecked(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_ref()) }
    }
}

pub trait StrCharIndex {
    fn char_index(&self, index: usize) -> char;
}

impl<T> StrCharIndex for T
where
    T: AsRef<[u8]>,
{
    fn char_index(&self, index: usize) -> char {
        self.as_ref()[index] as char
    }
}
