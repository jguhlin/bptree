#![feature(is_sorted)]

//! B+ Tree used for Index as well as SeqLocs?
//! As this is a write-once data structure, we don't need to reorganize anything after (so insertions, etc..).
//!
//! Todo: Set it up so we can append nodes to it (keys should already be in order, remember?). For very large indices.
//! But this is a TODO for now...
//!
//! TODO: Should have a values only store for ordinal keys
//! (don't need to store the keys)


struct BPlusTree<K, V> {
    /// Root node
    root: Box<Node<K, V>>,
    /// Order of the tree, i.e. the number of children each node can have
    order: usize,
    /// Depth of the tree
    depth: usize,
    /// Size of the leaf nodes, how many values can be stored in each leaf node
    leaf_node_size: usize,
}

impl<K, V> BPlusTree<K, V> 
where K: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug,
      V: std::fmt::Debug,
{
    /// Create a new B+ Tree
    fn new(order: usize) -> Self {
        // Create root node
        let root = Node::new(order);

        Self {
            root: Box::new(root),
            order,
            depth: 0,
            leaf_node_size: 1024,
        }
    }

    /// Set the size of the leaf nodes
    fn with_leaf_node_size(mut self, size: usize) -> Self {
        self.leaf_node_size = size;
        self
    }

    /// Set the order of the tree
    fn with_order(mut self, order: usize) -> Self {
        self.order = order;
        self
    }

    /// Create a B+ Tree from a list of items.
    fn from_items(&mut self, order: usize, keys: Vec<K>, values: Vec<V>) {
        // Keys should be sorted
        assert!(keys.is_sorted());
        assert!(keys.len() == values.len());

        // Calculate depth
        self.depth = self.depth(keys.len());

        // We can calculate the values that split the keys into the nodes
        let split_values: Vec<K> = self.split_keys(&keys[..]);

        // Create the root node
        let root: Node<K,V> = Node::new(order);
        self.root = Box::new(root);

        // Insert the keys into the root node
        for (i, key) in keys.iter().enumerate() {
            // Insert the key/value pair into the root node
            self.root.insert(*key, values[i]);

            // Insert the child node if we have one
            if i < split_values.len() {
                // Create the child node
                let mut child = Node::new(order);

                // Insert the key/value pair into the child node
                child.insert(split_values[i], values[i]);

                // Insert the child node into the root node
                self.root.insert_child(Box::new(child));
            }
        }

    }

    // Calculate the depth of the tree
    fn depth(&self, n: usize) -> usize {
        // Depth should allow stores of 1024 items in the leaf nodes (so if n = 96,000,000 and o=4, then d=9 to store all items in the leaf nodes,
        // with no more than 1,024 items in each leaf node)

        // Formula is
        // o^d * 1024 <= n

        // So we need to solve for d
        let mut d = 1;
        while (self.order.pow(d) * 1024) <= n {
            d += 1;
        }

        d as usize
    }

    fn split_keys(&self, keys: &[K]) -> Vec<K> {
        // Split values are the values that split the keys into the nodes
        // So if we have 96,000,000 keys, and o=4, then we need to find the 3 keys that split into even chunks

        let splits: Vec<K> = keys.chunks(self.order - 1).map(|chunk| chunk[0]).collect();
        splits
    }
}

/// B+ Tree Node
pub struct Node<K, V> {
    /// Keys
    keys: Vec<K>,
    /// Values
    values: Vec<V>,
    /// Children
    children: Vec<Box<Node<K, V>>>,
    /// Order of the tree
    order: usize,
}

impl<K, V> Node<K, V> 
where K: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug,
      V: std::fmt::Debug,
{
    /// Create a new node
    fn new(order: usize) -> Self {
        Self {
            keys: Vec::with_capacity(order - 1),
            values: Vec::with_capacity(order - 1),
            children: Vec::with_capacity(order),
            order,
        }
    }

    /// Insert a key/value pair into the node
    fn insert(&mut self, key: K, value: V) {
        // Find the index to insert the key/value pair
        let index = self.keys.binary_search(&key).unwrap_or_else(|x| x);

        // Insert the key/value pair
        self.keys.insert(index, key);
        self.values.insert(index, value);
    }

    /// Insert a key/value pair into the node
    fn insert_child(&mut self, child: Box<Node<K, V>>) {
        // Find the index to insert the key/value pair
        let index = self.keys.binary_search(&child.keys[0]).unwrap_or_else(|x| x);

        // Insert the key/value pair
        self.children.insert(index, child);
    }
}