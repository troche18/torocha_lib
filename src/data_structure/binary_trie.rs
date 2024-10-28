struct Node {
    count: usize,
    children: [Option<Box<Node>>; 2],
}

impl Node {
    fn new() -> Self {
        Self {
            count: 0,
            children: Default::default(),
        }
    }
}

struct BinaryTrie {
    root: Node,
    bit_depth: usize,
    xor_mask: usize,
}

impl BinaryTrie {
    fn new(bit_depth: usize) -> Self {
        Self {
            root: Node::new(),
            bit_depth,
            xor_mask: 0,
        }
    }

    fn insert(&mut self, x: usize) {
        let mut node = &mut self.root;
        node.count += 1;
        for b in (0..self.bit_depth).rev() {
            let mask = (self.xor_mask >> b) & 1;
            let bit = ((x >> b) & 1) ^ mask;
            if node.children[bit].is_none() {
                node.children[bit] = Some(Box::new(Node::new()));
            }
            node = node.children[bit].as_deref_mut().unwrap();
            node.count += 1;
        }
    }

    fn find(&self, x: usize) -> usize {
        let mut node = &self.root;
        for b in (0..self.bit_depth).rev() {
            let mask = (self.xor_mask >> b) & 1;
            let bit = ((x >> b) & 1) ^ mask;
            if node.children[bit].is_none() {
                return 0;
            }
            node = node.children[bit].as_deref().unwrap();
        }
        node.count
    }

    fn discard(&mut self, x: usize) {
        if self.find(x) == 0 {
            return;
        }
        let mut node = &mut self.root;
        node.count -= 1;
        for b in (0..self.bit_depth).rev() {
            let mask = (self.xor_mask >> b) & 1;
            let bit = ((x >> b) & 1) ^ mask;
            if node.children[bit].is_none() {
                break;
            }
            if node.children[bit].as_deref_mut().unwrap().count > 1 {
                node = node.children[bit].as_deref_mut().unwrap();
                node.count -= 1;
            } else {
                node.children[bit] = None;
                break;
            }
        }
    }

    fn xor_mask_change(&mut self, x: usize) {
        self.xor_mask = x;
    }

    fn less_x(&mut self, x: usize) -> usize {
        let mut node = &mut self.root;
        let mut ret = 0;
        for b in (0..self.bit_depth).rev() {
            let mask = (self.xor_mask >> b) & 1;
            let bit = (x >> b) & 1;
            let bit_mask = bit ^ mask;
            if node.children[bit_mask].is_none() {
                if bit == 1 {
                    ret += node.children[bit_mask ^ 1].as_ref().unwrap().count;
                }
                return ret;
            }
            if bit == 1 {
                if node.children[bit_mask ^ 1].is_some()
                {
                    ret += node.children[bit_mask ^ 1].as_ref().unwrap().count;
                }
            }
            node = node.children[bit_mask].as_deref_mut().unwrap();
        }
        ret
    }

    fn lower_bound(&mut self, x: usize) -> Option<usize> {
        let less = self.less_x(x);
        let total = self.root.count;
        if less + 1 > total {
            None
        } else {
            self.get_kth_min(less + 1)
        }
    }

    fn get_min(&mut self) -> Option<usize> {
        self.get_kth_min(1)
    }

    fn get_max(&mut self) -> Option<usize> {
        self.get_kth_max(1)
    }

    fn get_kth_min(&mut self, mut k: usize) -> Option<usize> {
        let mut node = &self.root;
        let total = node.count;
        if total < k {
            return None;
        }
        let mut ret = 0;
        for b in (0..self.bit_depth).rev() {
            ret = ret << 1;
            let mask = (self.xor_mask >> b) & 1;
            if node.children[0 ^ mask].is_none() {
                node = node.children[1 ^ mask].as_deref().unwrap();
                ret += 1;
                continue;
            }
            if node.children[1 ^ mask].is_none() {
                node = node.children[0 ^ mask].as_deref().unwrap();
                continue;
            }
            if k <= node.children[0 ^ mask].as_deref().unwrap().count {
                node = node.children[0 ^ mask].as_deref().unwrap();
            } else {
                k -= node.children[0 ^ mask].as_deref().unwrap().count;
                node = node.children[1 ^ mask].as_deref().unwrap();
                ret += 1;
            }
        }
        Some(ret)
    }

    fn get_kth_max(&mut self, k: usize) -> Option<usize> {
        let total = self.root.count;
        if total < k {
            return None;
        }
        self.get_kth_min(total - k + 1)
    }
}

#[test]
fn binary_trie() {
    let v = vec![0, 3, 4, 5, 5, 6, 9];
    let mut binary_trie = BinaryTrie::new(30);
    assert_eq!(None, binary_trie.get_min());
    for i in v {
        binary_trie.insert(i);
    }

    // get_kth
    assert_eq!(4, binary_trie.get_kth_min(3).unwrap());
    assert_eq!(6, binary_trie.get_kth_max(2).unwrap());

    // find
    assert_eq!(1, binary_trie.find(0));
    assert_eq!(0, binary_trie.find(1));
    assert_eq!(2, binary_trie.find(5));

    // remove
    binary_trie.discard(6);
    assert_eq!(0, binary_trie.find(6));
    binary_trie.discard(5);
    assert_eq!(1, binary_trie.find(5));
    binary_trie.insert(6);
    binary_trie.insert(5);

    // less_x
    assert_eq!(3, binary_trie.less_x(5));
    assert_eq!(7, binary_trie.less_x(100));
    assert_eq!(0, binary_trie.less_x(0));

    // get_min get_max
    assert_eq!(0, binary_trie.get_min().unwrap());
    assert_eq!(9, binary_trie.get_max().unwrap());

    // lower_bound
    assert_eq!(3, binary_trie.lower_bound(2).unwrap());
    assert_eq!(5, binary_trie.lower_bound(5).unwrap());
    assert_eq!(None, binary_trie.lower_bound(10));

    // xor_mask
    binary_trie.xor_mask_change(3);
    // 0 3 5 6 6 7 10
    assert_eq!(6, binary_trie.get_kth_min(4).unwrap());
    assert_eq!(1, binary_trie.find(7));
    binary_trie.discard(7);
    assert_eq!(0, binary_trie.find(7));
    binary_trie.insert(7);
    assert_eq!(1, binary_trie.find(7));
    assert_eq!(5, binary_trie.less_x(7));
    assert_eq!(10, binary_trie.get_max().unwrap());
    assert_eq!(10, binary_trie.lower_bound(9).unwrap());
    binary_trie.xor_mask_change(1);
    // 1 2 5 4 4 7 8
    assert_eq!(2, binary_trie.find(4));
}