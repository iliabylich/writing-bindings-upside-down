#[cfg(feature = "external")]
mod sizes;

#[cfg(not(feature = "external"))]
mod native {
    pub struct PrefixTree {
        pub(crate) root: PrefixTreeNode,
    }

    pub(crate) struct PrefixTreeNode {
        pub(crate) character: char,
        pub(crate) is_word_end: bool,
        pub(crate) children: Vec<PrefixTreeNode>,
    }

    impl PrefixTree {
        const DUMMY_ROOT: char = '!';

        pub fn new() -> Self {
            Self {
                root: PrefixTreeNode {
                    character: Self::DUMMY_ROOT,
                    is_word_end: false,
                    children: vec![],
                },
            }
        }

        pub(crate) fn get_root(&self) -> &PrefixTreeNode {
            &self.root
        }

        pub(crate) fn get_root_mut(&mut self) -> &mut PrefixTreeNode {
            &mut self.root
        }
    }

    impl PrefixTreeNode {
        pub(crate) fn children_len(&self) -> usize {
            self.children.len()
        }

        pub(crate) fn child_at(&self, idx: usize) -> &PrefixTreeNode {
            &self.children[idx]
        }

        pub(crate) fn mut_child_at(&mut self, idx: usize) -> &mut PrefixTreeNode {
            &mut self.children[idx]
        }

        pub(crate) fn character(&self) -> char {
            self.character
        }

        pub(crate) fn push_child(&mut self, child: PrefixTreeNode) {
            self.children.push(child)
        }

        pub(crate) fn new(
            character: char,
            is_word_end: bool,
            children: Vec<PrefixTreeNode>,
        ) -> Self {
            Self {
                character,
                is_word_end,
                children,
            }
        }

        pub(crate) fn is_word_end(&self) -> bool {
            self.is_word_end
        }

        pub(crate) fn set_is_word_end(&mut self, value: bool) {
            self.is_word_end = value;
        }
    }
}
#[cfg(not(feature = "external"))]
type Char = char;
#[cfg(not(feature = "external"))]
type VecPrefixTreeNode = Vec<PrefixTreeNode>;
#[cfg(not(feature = "external"))]
pub use native::PrefixTree;
#[cfg(not(feature = "external"))]
use native::PrefixTreeNode;

#[cfg(feature = "external")]
mod external {
    use crate::sizes::*;

    #[repr(C)]
    pub(crate) struct Char {
        bytes: [u8; CHAR_SIZE],
    }

    #[repr(C)]
    pub struct PrefixTree {
        bytes: [u8; PREFIX_TREE_SIZE],
    }

    #[repr(C)]
    pub struct VecPrefixTreeNode {
        bytes: [u8; VEC_PREFIX_TREE_NODE_SIZE],
    }

    #[repr(C)]
    pub struct PrefixTreeNode {
        bytes: [u8; PREFIX_TREE_NODE_SIZE],
    }

    extern "C" {
        fn prefix_tree_node__children_len(this: *const PrefixTreeNode) -> usize;
        fn prefix_tree_node__child_at(
            this: *const PrefixTreeNode,
            idx: usize,
        ) -> *mut PrefixTreeNode;
        fn prefix_tree_node__push_child(this: *mut PrefixTreeNode, child: PrefixTreeNode);
        fn prefix_tree_node__character(this: *const PrefixTreeNode) -> Char;
        fn prefix_tree_node__new(
            character: Char,
            is_word_end: bool,
            children: VecPrefixTreeNode,
        ) -> PrefixTreeNode;
        fn prefix_tree_node__is_word_end(this: *const PrefixTreeNode) -> bool;
        fn prefix_tree_node__set_is_word_end(this: *mut PrefixTreeNode, value: bool);

        fn char__new(c1: u8, c2: u8, c3: u8, c4: u8) -> Char;
        fn char__cmp(this: *const Char, c1: u8, c2: u8, c3: u8, c4: u8) -> bool;

        fn vec_prefix_tree_node__new() -> VecPrefixTreeNode;

        fn prefix_tree__new() -> PrefixTree;
        fn prefix_tree__get_root(this: *const PrefixTree) -> *mut PrefixTreeNode;
    }

    impl PrefixTreeNode {
        pub(crate) fn children_len(&self) -> usize {
            unsafe { prefix_tree_node__children_len(self) }
        }

        pub(crate) fn child_at(&self, idx: usize) -> &PrefixTreeNode {
            unsafe { prefix_tree_node__child_at(self, idx).as_ref().unwrap() }
        }

        pub(crate) fn mut_child_at(&mut self, idx: usize) -> &mut PrefixTreeNode {
            unsafe { prefix_tree_node__child_at(self, idx).as_mut().unwrap() }
        }

        pub(crate) fn push_child(&mut self, child: PrefixTreeNode) {
            unsafe { prefix_tree_node__push_child(self, child) }
        }

        pub(crate) fn character(&self) -> Char {
            unsafe { prefix_tree_node__character(self) }
        }

        pub(crate) fn new(character: Char, is_word_end: bool, children: VecPrefixTreeNode) -> Self {
            unsafe { prefix_tree_node__new(character, is_word_end, children) }
        }

        pub(crate) fn is_word_end(&self) -> bool {
            unsafe { prefix_tree_node__is_word_end(self) }
        }

        pub(crate) fn set_is_word_end(&mut self, value: bool) {
            unsafe { prefix_tree_node__set_is_word_end(self, value) }
        }
    }

    impl PartialEq<char> for Char {
        fn eq(&self, c: &char) -> bool {
            let mut buf = [0; 4];
            c.encode_utf8(&mut buf);
            unsafe { char__cmp(self, buf[0], buf[1], buf[2], buf[3]) }
        }
    }

    impl From<char> for Char {
        fn from(c: char) -> Self {
            let mut buf = [0; 4];
            c.encode_utf8(&mut buf);
            unsafe { char__new(buf[0], buf[1], buf[2], buf[3]) }
        }
    }

    impl VecPrefixTreeNode {
        pub(crate) fn new() -> Self {
            unsafe { vec_prefix_tree_node__new() }
        }
    }

    impl PrefixTree {
        pub fn new() -> Self {
            unsafe { prefix_tree__new() }
        }

        pub(crate) fn get_root(&self) -> &PrefixTreeNode {
            unsafe { prefix_tree__get_root(self).as_ref().unwrap() }
        }

        pub(crate) fn get_root_mut(&mut self) -> &mut PrefixTreeNode {
            unsafe { prefix_tree__get_root(self).as_mut().unwrap() }
        }
    }
}
#[cfg(feature = "external")]
use external::Char;
#[cfg(feature = "external")]
pub use external::PrefixTree;
#[cfg(feature = "external")]
use external::PrefixTreeNode;
#[cfg(feature = "external")]
use external::VecPrefixTreeNode;

impl PrefixTreeNode {
    fn find_child<'a>(&'a self, c: char) -> Option<&'a PrefixTreeNode> {
        for idx in 0..self.children_len() {
            let child = self.child_at(idx);
            if child.character() == c {
                return Some(child);
            }
        }
        None
    }

    fn find_or_create_child<'a>(&'a mut self, c: char) -> &'a mut PrefixTreeNode {
        let mut child_idx = None;
        for idx in 0..self.children_len() {
            if self.child_at(idx).character() == c {
                child_idx = Some(idx)
            }
        }
        match child_idx {
            Some(idx) => self.mut_child_at(idx),
            None => {
                self.push_child(PrefixTreeNode::new(
                    Char::from(c),
                    false,
                    VecPrefixTreeNode::new(),
                ));
                self.mut_child_at(self.children_len() - 1)
            }
        }
    }
}

impl PrefixTree {
    pub fn insert<T: AsRef<str>>(&mut self, s: T) {
        let s = s.as_ref();
        let mut node = self.get_root_mut();
        for c in s.chars() {
            node = node.find_or_create_child(c);
        }
        node.set_is_word_end(true);
    }

    pub fn has<T: AsRef<str>>(&self, s: T) -> bool {
        let s = s.as_ref();
        let mut node = self.get_root();
        for c in s.chars() {
            match node.find_child(c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_word_end()
    }
}

#[test]
fn test_prefix_tree() {
    let mut tree = PrefixTree::new();
    tree.insert("foo");
    tree.insert("bar");
    tree.insert("baz");
    tree.insert("b");

    assert!(tree.has("foo"));
    assert!(tree.has("bar"));
    assert!(tree.has("baz"));
    assert!(tree.has("b"));

    assert!(!tree.has("unknown"));
    assert!(!tree.has("fo"));
    assert!(!tree.has("barr"));
    assert!(!tree.has("ba"));
}
