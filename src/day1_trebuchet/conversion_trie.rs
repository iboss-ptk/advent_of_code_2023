use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Node {
    value: Option<u32>,
    subtrie: ConversionTrie,
}

impl Node {
    fn completed(value: u32) -> Self {
        Self {
            value: Some(value),
            subtrie: ConversionTrie {
                subtree: HashMap::new(),
            },
        }
    }

    fn intermediate(input: &str, value: u32) -> Self {
        Self {
            value: None,
            subtrie: ConversionTrie::new(input, value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConversionTrie {
    subtree: HashMap<char, Node>,
}

impl ConversionTrie {
    pub fn empty() -> Self {
        Self {
            subtree: HashMap::new(),
        }
    }

    pub fn new(input: &str, value: u32) -> Self {
        let mut tree = ConversionTrie {
            subtree: HashMap::new(),
        };
        tree.insert(input, value);
        tree
    }

    pub fn insert(&mut self, input: &str, value: u32) {
        let mut chars = input.chars();

        // end insertion if there are no more characters
        let Some(head) = chars.next() else {
            return;
        };

        let rest = chars.as_str();

        match self.get_child_mut(&head) {
            Some(child) => child.subtrie.insert(rest, value),
            None => self.add_child(
                head,
                // mark the node as completed if there are no more characters
                // else mark it as intermediate
                if rest.is_empty() {
                    Node::completed(value)
                } else {
                    Node::intermediate(rest, value)
                },
            ),
        }
    }

    pub fn convert_head<'a>(&self, input: &'a str) -> Option<(u32, &'a str)> {
        let mut chars = input.chars();

        // Return None if there are no more characters
        // Since if any matched value is found,
        // it must be returned on the previous iteration
        let Some(head) = chars.next() else {
            return None;
        };

        let rest = chars.as_str();

        // check if the current character is a child
        // if it is, continue searching with the rest
        // else return false
        match self.get_child(&head) {
            Some(child) => {
                // if the child has value, that means pattern is matched
                // return the value
                match child.value {
                    Some(value) => Some((value, rest)),
                    None => child.subtrie.convert_head(rest),
                }
            }
            None => None,
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
macro_rules! conversion_trie {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            let mut tree = $crate::day1_trebuchet::conversion_trie::ConversionTrie::empty();
            $(
                tree.insert($x, $y);
            )*
            tree
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_suffix_tree() {
        let tree = conversion_trie! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9
        };

        assert_eq!(tree.convert_head("one"), Some((1, "")));
        assert_eq!(tree.convert_head("1two"), None);
        assert_eq!(tree.convert_head("twenty"), None);
        assert_eq!(tree.convert_head("three1two"), Some((3, "1two")));
    }
}
