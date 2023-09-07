#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> TreeNode<T> {
    fn new(element: T) -> TreeNode<T> {
        TreeNode{element, left: BinaryTree::Empty, right: BinaryTree::Empty}
    }
}

impl<T: Ord + std::fmt::Debug> BinaryTree<T> {
    fn add(&mut self, element: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode::new(element)));
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if element <= node.element {
                    node.left.add(element);
                } else {
                    node.right.add(element);
                }
            }
        }
    }

    fn describe(&self) -> String {
        match *self {
            BinaryTree::Empty => "Empty".to_string(),
            BinaryTree::NonEmpty(ref node) => {
                format!("NonEmpty: element={:?}, left=({}), right=({})", node.element, node.left.describe(), node.right.describe())
            }
        }
    }
}

fn main() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Jupiter");

    println!("{}", tree.describe());
}
