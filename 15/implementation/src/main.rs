fn main() {
    {
        struct I32Range {
            start: i32,
            end: i32
        }

        impl Iterator for I32Range {
            type Item = i32;

            fn next(&mut self) -> Option<i32> {
                if self.start >= self.end {
                    return None;
                }

                let result = Some(self.start);
                self.start += 1;

                result
            }
        }

        let mut pi = 0.0;
        let mut numerator = 1.0;

        for k in (I32Range { start: 0, end: 14}) {
            pi += numerator / (2 * k + 1) as f64;

            numerator /= -3.0;
        }

        pi *= f64::sqrt(12.0);
        assert_eq!(pi as f32, std::f32::consts::PI);
    }
    {
        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>)
        }

        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>
        }

        impl<T: Ord> BinaryTree<T> {
            fn add(&mut self, value: T) {
                match *self {
                    BinaryTree::Empty => {
                        *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                            element: value,
                            left: BinaryTree::Empty,
                            right: BinaryTree::Empty
                        }))
                    },
                    BinaryTree::NonEmpty(ref mut node) => {
                        if value <= node.element {
                            node.left.add(value);
                        } else {
                            node.right.add(value);
                        }
                    }
                }
            }
        }

        struct TreeIter<'a, T: 'a> {
            unvisited: Vec<&'a TreeNode<T>>
        }

        impl<'a, T: 'a> TreeIter<'a, T> {
            fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
                while let BinaryTree::NonEmpty(ref node) = *tree {
                    self.unvisited.push(node);
                    tree = &node.left;
                }
            }
        }

        impl<T> BinaryTree<T> {
            fn iter(&self) -> TreeIter<T> {
                let mut iter = TreeIter { unvisited: Vec::new() };
                iter.push_left_edge(self);
                iter
            }
        }

        impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
            type Item = &'a T;
            type IntoIter = TreeIter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a, T> Iterator for TreeIter<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<&'a T> {
                let node = match self.unvisited.pop() {
                    None => return None,
                    Some(n) => n
                };

                self.push_left_edge(&node.right);

                Some(&node.element)
            }
        }

        let mut tree = BinaryTree::<i32>::Empty;
        tree.add(10);
        tree.add(5);
        tree.add(15);
        tree.add(3);
        tree.add(7);
        tree.add(13);
        tree.add(17);

        for i in tree.iter() {
            println!("{}", i);
        }
    }
}
