#[cfg(not(feature = "external"))]
mod native {
    pub type Char = char;
    pub type CharList = Vec<char>;

    pub trait CharAt {
        fn at(&self, idx: usize) -> Char;
    }

    impl CharAt for CharList {
        fn at(&self, idx: usize) -> Char {
            self[idx]
        }
    }
}
#[cfg(not(feature = "external"))]
use native::{Char, CharAt, CharList};

#[cfg(feature = "external")]
mod sizes;
#[cfg(feature = "external")]
mod external {
    use crate::sizes::*;

    #[repr(C)]
    pub struct Char {
        bytes: [u8; CHAR_SIZE],
    }

    #[repr(C)]
    pub struct CharList {
        bytes: [u8; CHAR_LIST_SIZE],
    }

    extern "C" {
        fn char__new(c1: u8, c2: u8, c3: u8, c4: u8) -> Char;
        fn char__at(this: *const Char, idx: u8) -> u8;
        fn char__drop(this: *mut Char);

        fn char_list__new() -> CharList;
        fn char_list__push(this: *mut CharList, item: Char);
        fn char_list__len(this: *const CharList) -> usize;
        fn char_list__at(this: *const CharList, idx: usize) -> Char;
        fn char_list__drop(this: *mut CharList);
    }

    impl Char {
        fn byte_at(&self, idx: u8) -> u8 {
            debug_assert!(idx <= 3);
            unsafe { char__at(self, idx) }
        }
    }

    impl From<char> for Char {
        fn from(c: char) -> Self {
            let mut buf = [0; 4];
            c.encode_utf8(&mut buf);
            unsafe { char__new(buf[0], buf[1], buf[2], buf[3]) }
        }
    }

    impl From<&Char> for char {
        fn from(c: &Char) -> Self {
            let c1 = c.byte_at(0);
            let c2 = c.byte_at(1);
            let c3 = c.byte_at(2);
            let c4 = c.byte_at(3);
            let bytes = [c1, c2, c3, c4];
            let mut zero_idx = 4;
            for idx in 0..4 {
                if bytes[idx] == 0 {
                    zero_idx = idx;
                    break;
                }
            }
            let bytes = &bytes[0..zero_idx];
            let s = std::str::from_utf8(bytes).unwrap();
            let chars = s.chars().collect::<Vec<_>>();
            debug_assert!(chars.len() == 1);
            chars[0]
        }
    }

    impl PartialEq for Char {
        fn eq(&self, other: &Self) -> bool {
            (0..4).all(|idx| self.byte_at(idx) == other.byte_at(idx))
        }
    }

    impl std::fmt::Debug for Char {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", char::from(self))
        }
    }

    impl CharList {
        pub fn new() -> Self {
            unsafe { char_list__new() }
        }
        pub fn push(&mut self, item: Char) {
            unsafe { char_list__push(self, item) }
        }
        pub fn len(&self) -> usize {
            unsafe { char_list__len(self) }
        }
        pub fn at(&self, idx: usize) -> Char {
            unsafe { char_list__at(self, idx) }
        }
    }

    impl Drop for Char {
        fn drop(&mut self) {
            unsafe { char__drop(self) }
        }
    }

    impl Drop for CharList {
        fn drop(&mut self) {
            unsafe { char_list__drop(self) }
        }
    }
}
#[cfg(feature = "external")]
use external::{Char, CharList};

pub fn foo(s: &str) -> CharList {
    let mut char_list = CharList::new();
    for char in s.chars() {
        if !char.is_ascii() {
            char_list.push(Char::from(char));
        }
    }
    char_list
}

#[no_mangle]
pub extern "C" fn c_foo(s: *const i8) -> CharList {
    let s = unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap() };
    foo(s)
}

#[test]
fn test_foo() {
    let s = "abc😋中国";
    let chars = foo(s);
    assert_eq!(chars.len(), 3);
    assert_eq!(chars.at(0), Char::from('😋'));
    assert_eq!(chars.at(1), Char::from('中'));
    assert_eq!(chars.at(2), Char::from('国'));
}
