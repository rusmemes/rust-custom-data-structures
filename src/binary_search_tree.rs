use crate::queue::Queue;

pub struct BinarySearchTree<T: Ord> {
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

pub struct BinarySearchTreeIterator<T> {
    inner: Queue<T>
}

impl <T> Iterator for BinarySearchTreeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.dequeue()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, item: T) {
        match &self.value {
            None => self.value = Some(item),
            Some(key) => {
                let target_node = if item < *key {
                    &mut self.left
                } else if item > *key {
                    &mut self.right
                } else {
                    return;
                };
                match target_node {
                    None => {
                        let mut sub_tree = BinarySearchTree::new();
                        sub_tree.insert(item);
                        *target_node = Some(Box::new(sub_tree))
                    }
                    Some(ref mut b) => b.insert(item),
                }
            }
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        match &self.value {
            None => false,
            Some(v) => {
                if *v == *item {
                    true
                } else {
                    let target_node = if *item < *v { &self.left } else { &self.right };
                    target_node
                        .as_ref()
                        .map(|ref sub_tree| sub_tree.contains(item))
                        .unwrap_or(false)
                }
            }
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.value {
            None => None,
            Some(v) => self
                .left
                .as_ref()
                .and_then(|ref sub_tree| sub_tree.min())
                .or(Some(v)),
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.value {
            None => None,
            Some(v) => self
                .right
                .as_ref()
                .and_then(|ref sub_tree| sub_tree.max())
                .or(Some(v)),
        }
    }

    pub fn floor(&self, item: &T) -> Option<&T> {
        match &self.value {
            None => None,
            Some(v) => {
                if *item > *v {
                    match &self.right {
                        None => Some(v),
                        Some(r) => match r.floor(item) {
                            None => Some(v),
                            Some(f) => Some(f),
                        },
                    }
                } else {
                    self.left.as_ref().and_then(|sub_tree| sub_tree.floor(item))
                }
            }
        }
    }

    pub fn ceil(&self, item: &T) -> Option<&T> {
        match &self.value {
            None => None,
            Some(v) => {
                if *item < *v {
                    match &self.left {
                        None => Some(v),
                        Some(r) => match r.ceil(item) {
                            None => Some(v),
                            Some(f) => Some(f),
                        },
                    }
                } else {
                    self.right.as_ref().and_then(|sub_tree| sub_tree.ceil(item))
                }
            }
        }
    }

    pub fn iter(&self) -> BinarySearchTreeIterator<&T> {
        let mut inner = Queue::new();
        self.iter_inner(&mut inner);
        BinarySearchTreeIterator { inner }
    }

    fn iter_inner<'a>(&'a self, refs: &mut Queue<&'a T>) {
        if let Some(l) = &self.left {
            l.iter_inner(refs);
        }
        if let Some(v) = &self.value {
            refs.enqueue(v)
        }
        if let Some(r) = &self.right {
            r.iter_inner(refs);
        }
    }

    pub fn into_iter(self) -> BinarySearchTreeIterator<T> {
        let mut inner = Queue::new();
        self.into_iter_inner(&mut inner);
        BinarySearchTreeIterator { inner }
    }

    fn into_iter_inner(self, refs: &mut Queue<T>) {
        if let Some(l) = self.left {
            l.into_iter_inner(refs);
        }
        if let Some(v) = self.value {
            refs.enqueue(v)
        }
        if let Some(r) = self.right {
            r.into_iter_inner(refs);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::binary_search_tree::BinarySearchTree;

    #[test]
    fn iter() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);

        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&13));
        assert_eq!(iter.next(), Some(&45));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);

        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(45));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn floor() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);

        assert_eq!(tree.floor(&1), Some(&0));
        assert_eq!(tree.floor(&5), Some(&1));
        assert_eq!(tree.floor(&100), Some(&45));
        assert_eq!(tree.floor(&20), Some(&13));
        assert_eq!(tree.floor(&0), None);

        let mut tree = BinarySearchTree::new();

        tree.insert(13);
        tree.insert(45);
        tree.insert(50);
        tree.insert(36);
        tree.insert(5);
        tree.insert(1);
        tree.insert(0);
        tree.insert(8);

        assert_eq!(tree.floor(&13), Some(&8));
        assert_eq!(tree.floor(&100), Some(&50));
        assert_eq!(tree.floor(&46), Some(&45));
        assert_eq!(tree.floor(&50), Some(&45));
        assert_eq!(tree.floor(&45), Some(&36));
        assert_eq!(tree.floor(&40), Some(&36));
        assert_eq!(tree.floor(&6), Some(&5));
        assert_eq!(tree.floor(&5), Some(&1));
        assert_eq!(tree.floor(&3), Some(&1));
        assert_eq!(tree.floor(&1), Some(&0));
        assert_eq!(tree.floor(&10), Some(&8));
    }

    #[test]
    fn ceil() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);

        assert_eq!(tree.ceil(&1), Some(&13));
        assert_eq!(tree.ceil(&5), Some(&13));
        assert_eq!(tree.ceil(&100), None);
        assert_eq!(tree.ceil(&20), Some(&45));
        assert_eq!(tree.ceil(&0), Some(&1));

        let mut tree = BinarySearchTree::new();

        tree.insert(13);
        tree.insert(45);
        tree.insert(50);
        tree.insert(36);
        tree.insert(5);
        tree.insert(1);
        tree.insert(0);
        tree.insert(8);

        assert_eq!(tree.ceil(&13), Some(&36));
        assert_eq!(tree.ceil(&100), None);
        assert_eq!(tree.ceil(&46), Some(&50));
        assert_eq!(tree.ceil(&50), None);
        assert_eq!(tree.ceil(&45), Some(&50));
        assert_eq!(tree.ceil(&40), Some(&45));
        assert_eq!(tree.ceil(&6), Some(&8));
        assert_eq!(tree.ceil(&5), Some(&8));
        assert_eq!(tree.ceil(&3), Some(&5));
        assert_eq!(tree.ceil(&1), Some(&5));
        assert_eq!(tree.ceil(&10), Some(&13));
        assert_eq!(tree.ceil(&0), Some(&1));
        assert_eq!(tree.ceil(&-1), Some(&0));
    }

    #[test]
    fn minimum() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);

        assert_eq!(tree.min(), Some(&0))
    }

    #[test]
    fn minimum_on_empty() {
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(tree.min(), None)
    }

    #[test]
    fn maximum() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);

        assert_eq!(tree.max(), Some(&45))
    }

    #[test]
    fn maximum_on_empty() {
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(tree.max(), None)
    }

    #[test]
    fn contains_number() {
        let mut tree = BinarySearchTree::new();

        tree.insert(1);
        tree.insert(45);
        tree.insert(13);
        tree.insert(0);
        tree.insert(10);
        tree.insert(7);
        tree.insert(12);
        tree.insert(31);
        tree.insert(37);
        tree.insert(24);
        tree.insert(2);
        tree.insert(27);
        tree.insert(17);
        tree.insert(7);

        assert!(tree.contains(&1));
        assert!(tree.contains(&12));
        assert!(tree.contains(&0));
        assert!(tree.contains(&7));
        assert!(tree.contains(&31));
        assert!(!tree.contains(&100));
        assert!(!tree.contains(&5));
        assert!(!tree.contains(&9));
        assert!(!tree.contains(&4));
        assert!(!tree.contains(&50));
    }

    #[test]
    fn contains_str() {
        let mut tree = BinarySearchTree::new();

        tree.insert("1");
        tree.insert("45");
        tree.insert("13");
        tree.insert("0");
        tree.insert("10");
        tree.insert("7");
        tree.insert("12");
        tree.insert("31");
        tree.insert("37");
        tree.insert("24");
        tree.insert("2");
        tree.insert("27");
        tree.insert("17");
        tree.insert("7");

        assert!(tree.contains(&"1"));
        assert!(tree.contains(&"12"));
        assert!(tree.contains(&"0"));
        assert!(tree.contains(&"7"));
        assert!(tree.contains(&"31"));
        assert!(!tree.contains(&"100"));
        assert!(!tree.contains(&"5"));
        assert!(!tree.contains(&"9"));
        assert!(!tree.contains(&"4"));
        assert!(!tree.contains(&"50"));
    }

    #[test]
    fn contains_string() {
        let mut tree = BinarySearchTree::new();

        tree.insert("1".to_string());
        tree.insert("45".to_string());
        tree.insert("13".to_string());
        tree.insert("0".to_string());
        tree.insert("10".to_string());
        tree.insert("7".to_string());
        tree.insert("12".to_string());
        tree.insert("31".to_string());
        tree.insert("37".to_string());
        tree.insert("24".to_string());
        tree.insert("2".to_string());
        tree.insert("27".to_string());
        tree.insert("17".to_string());
        tree.insert("7".to_string());

        assert!(tree.contains(&"1".to_string()));
        assert!(tree.contains(&"12".to_string()));
        assert!(tree.contains(&"0".to_string()));
        assert!(tree.contains(&"7".to_string()));
        assert!(tree.contains(&"31".to_string()));
        assert!(!tree.contains(&"100".to_string()));
        assert!(!tree.contains(&"5".to_string()));
        assert!(!tree.contains(&"9".to_string()));
        assert!(!tree.contains(&"4".to_string()));
        assert!(!tree.contains(&"50".to_string()));
    }
}
