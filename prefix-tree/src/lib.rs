#[derive(Debug)]
pub struct PrefixTree {
    root: PrefixTreeNode,
}

#[derive(Debug)]
struct PrefixTreeNode {
    character: char,
    is_word_end: bool,
    children: Vec<PrefixTreeNode>,
}

impl PrefixTreeNode {
    fn find_child<'a>(&'a self, c: char) -> Option<&'a PrefixTreeNode> {
        self.children.iter().find(|child| child.character == c)
    }

    fn find_or_create_child<'a>(&'a mut self, c: char) -> &'a mut PrefixTreeNode {
        let child_idx = self
            .children
            .iter_mut()
            .enumerate()
            .find(|(_idx, child)| child.character == c)
            .map(|(idx, _)| idx);

        match child_idx {
            Some(idx) => &mut self.children[idx],
            None => {
                self.children.push(PrefixTreeNode {
                    character: c,
                    is_word_end: false,
                    children: vec![],
                });
                self.children.last_mut().unwrap()
            }
        }
    }
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

    pub fn insert<T: AsRef<str>>(&mut self, s: T) {
        let s = s.as_ref();
        let mut node = &mut self.root;
        for c in s.chars() {
            node = node.find_or_create_child(c);
        }
        node.is_word_end = true;
    }

    pub fn has<T: AsRef<str>>(&self, s: T) -> bool {
        let s = s.as_ref();
        let mut node = &self.root;
        for c in s.chars() {
            match node.find_child(c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_word_end
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
