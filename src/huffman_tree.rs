use std::rc::Rc;
use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, BinaryHeap};

/// Denotes a Huffman node as either an internal node (if it has children)
/// or a leaf node (if it stores a character).
enum NodeData {
    Children(Rc<HuffmanNode>, Rc<HuffmanNode>),
    Character(char),
}

/// A node in the Huffman code tree.
/// All nodes are associated with a frequency, which is used by the algorithm
/// to construct the code tree by merging by lowest frequency.
struct HuffmanNode {
    freq: usize,
    data: NodeData,
}

impl HuffmanNode {
    fn leaf(c: char, freq: usize) -> Self {
        Self { freq, data: NodeData::Character(c) }
    }

    fn internal(left: Rc<HuffmanNode>, right: Rc<HuffmanNode>, freq: usize) -> Self {
        Self { freq, data: NodeData::Children(left, right) }
    }

    fn freq(&self) -> usize {
        self.freq
    }
}

impl Eq for HuffmanNode {}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Orders nodes in reverse order of frequency so that nodes with least
/// frequency have highest priority in the priority queue.
/// In other words, the BinaryHeap into which they are inserted becomes a 
/// MIN priority queue.
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq).reverse()
    }
}

/// Constructs a min priority queue of leaf nodes in the Huffamn code tree.
/// Prepares the portion of the algorithm which repeatedly merges the nodes
/// with the smallest frequencies.
fn get_frequencies(s: &str) -> BinaryHeap<HuffmanNode> {
    let mut freq = HashMap::new();
    for c in s.chars() {
        let entry = freq.entry(c).or_insert(0);
        *entry += 1;
    }

    let mut min_queue = BinaryHeap::new();
    for c in freq.keys() {
        min_queue.push(HuffmanNode::leaf(*c, freq[c]));
    }
    min_queue
}



/// Constructs a Huffman code tree and returns the root node.
fn build_huffman_tree(min_queue: &mut BinaryHeap<HuffmanNode>) 
    -> Result<HuffmanNode, &'static str> 
{
    if min_queue.is_empty() {
        return Err("cannot construct a Huffman code with no characters");
    }

    while min_queue.len() > 1 {
        let x = min_queue.pop().unwrap();
        let y = min_queue.pop().unwrap();
        let freq_sum = x.freq() + y.freq();
        let z = HuffmanNode::internal(Rc::new(x), Rc::new(y), freq_sum);
        min_queue.push(z);
    }
    Ok(min_queue.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_built_correctly() {
        let s = "aaaabbbccd";
        let mut freq = get_frequencies(&s);
        
        assert!(matches!(freq.pop().unwrap().data, NodeData::Character('d')));
        assert!(matches!(freq.pop().unwrap().data, NodeData::Character('c')));
        assert!(matches!(freq.pop().unwrap().data, NodeData::Character('b')));
        assert!(matches!(freq.pop().unwrap().data, NodeData::Character('a')));
        assert!(matches!(freq.pop(), None));

        freq = get_frequencies(&s);
        let root = build_huffman_tree(&mut freq).unwrap();
        assert_eq!(root.freq, 10);
        match root.data {
            NodeData::Children(left, right) => {
                assert_eq!(left.freq, 4);
                assert_eq!(right.freq, 6);

                assert!(matches!(&left.data, NodeData::Character('a')));
                match &right.data {
                    NodeData::Children(left, right) => {
                        assert_eq!(left.freq, 3);
                        assert_eq!(right.freq, 3);

                        assert!(matches!(&left.data, NodeData::Character('b')));
                        match &right.data {
                            NodeData::Children(left, right) => {
                                assert_eq!(left.freq, 1);
                                assert_eq!(right.freq, 2);

                                assert!(matches!(left.data, NodeData::Character('d')));
                                assert!(matches!(right.data, NodeData::Character('c')));
                            },
                            _ => panic!("node should have children")
                        }
                    },
                    _ => panic!("node should have children")
                }
            },
            _ => panic!("node should have children")
        }
    }
}
