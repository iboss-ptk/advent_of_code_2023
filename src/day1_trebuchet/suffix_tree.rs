use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Node {
    completed: bool,
    subtree: SuffixTree,
}

impl Node {
    fn completed() -> Self {
        Self {
            completed: true,
            subtree: SuffixTree {
                subtree: HashMap::new(),
            },
        }
    }

    fn intermediate(input: &str) -> Self {
        Self {
            completed: false,
            subtree: SuffixTree::new(input),
        }
    }
}

#[derive(Debug, PartialEq)]
struct SuffixTree {
    subtree: HashMap<char, Node>,
}

impl SuffixTree {
    fn empty() -> Self {
        Self {
            subtree: HashMap::new(),
        }
    }

    fn new(input: &str) -> Self {
        let mut tree = SuffixTree {
            subtree: HashMap::new(),
        };
        tree.insert(input);
        tree
    }

    fn insert(&mut self, input: &str) {
        let mut chars = input.chars();

        // end insertion if there are no more characters
        let Some(head) = chars.next() else {
            return;
        };

        let rest = chars.as_str();

        match self.get_child_mut(&head) {
            Some(child) => child.subtree.insert(rest),
            None => self.add_child(
                head,
                // mark the node as completed if there are no more characters
                // else mark it as intermediate
                if rest.is_empty() {
                    Node::completed()
                } else {
                    Node::intermediate(rest)
                },
            ),
        }
    }

    fn contains(&self, input: &str) -> bool {
        let mut chars = input.chars();

        // end search if there are no more characters
        // return true since getting here requires
        // all previous characters to be found
        let Some(head) = chars.next() else {
            return true;
        };
        let rest = chars.as_str();

        // check if the current character is a child
        // if it is, continue searching with the rest
        // else return false
        match self.get_child(&head) {
            Some(child) => {
                // if there are no more characters to search
                // return the completed status of the child
                if rest.is_empty() {
                    child.completed
                } else {
                    child.subtree.contains(rest)
                }
            }
            None => false,
        }
    }

    fn add_child(&mut self, suffix: char, child: Node) {
        self.subtree.insert(suffix, child);
    }

    fn get_child_mut(&mut self, suffix: &char) -> Option<&mut Node> {
        self.subtree.get_mut(suffix)
    }

    fn get_child(&self, suffix: &char) -> Option<&Node> {
        self.subtree.get(suffix)
    }
}

#[macro_export]
macro_rules! suffix_tree {
    ( $( $x:expr ),* ) => {
        {
            let mut tree = SuffixTree::empty();
            $(
                tree.insert($x);
            )*
            tree
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_tree() {
        let tree = suffix_tree!["a", "b", "any", "anus"];

        // checks if the tree contains the given string
        assert!(tree.contains("a"));
        assert!(tree.contains("b"));
        assert!(tree.contains("any"));
        assert!(tree.contains("anus"));
        assert!(!tree.contains("an"));
    }
}
