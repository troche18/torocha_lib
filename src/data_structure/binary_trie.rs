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
}

impl BinaryTrie {
    fn new(bit_depth: usize) -> Self {
        Self {
            root: Node::new(),
            bit_depth,
        }
    }

    fn insert(&mut self, x: usize) {
        let mut node = &mut self.root;
        for b in (0..self.bit_depth).rev() {
            let bit = (x >> b) & 1;
            if node.children[bit].is_none() {
                node.children[bit] = Some(Box::new(Node::new()));
            }
            node = node.children[bit].as_deref_mut().unwrap();
            node.count += 1;
        }
    }

    fn find() {
        todo!()
    }

    fn remove() {
        todo!()
    }

    fn xor_mask() {
        todo!()
    }

    fn less() {
        todo!()
    }

    fn merge() {
        todo!()
    }

    fn lower_bound() {
        todo!()
    }

    fn upper_bound() {
        todo!()
    }

    fn get_min() {
        todo!()
    }

    fn get_max() {
        todo!()
    }

    fn get_kth_min(&mut self, mut k: usize) -> usize {
        let mut node = &self.root;
        let mut ret = 0;
        for _ in 0..self.bit_depth {
            ret = ret << 1;
            if node.children[0].is_none() {
                node = node.children[1].as_deref().unwrap();
                ret += 1;
                continue;
            }
            if node.children[1].is_none() {
                node = node.children[0].as_deref().unwrap();
                continue;
            }
            if k <= node.children[0].as_deref().unwrap().count {
                node = node.children[0].as_deref().unwrap();
                continue;
            } else {
                k -= node.children[0].as_deref().unwrap().count;
                node = node.children[1].as_deref().unwrap();
                ret += 1;
                continue;
            }
        }
        ret
    }

    fn get_kth_max(&mut self, k: usize) -> usize {
        let mut total = 0;
        if let Some(x) = &self.root.children[0].as_deref() {
            total += x.count;
        }
        if let Some(x) = &self.root.children[1].as_deref() {
            total += x.count;
        }
        self.get_kth_min(total - k + 1)
    }
}
