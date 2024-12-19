#![allow(unused)]

use std::{path::Display, str::FromStr};

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn with_value(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct LinkedList<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut next_node_option = &self.root;

        while let Some(ref next_node) = next_node_option {
            let separator = if next_node.next.is_some() { " -> "} else { "" };
            write!(f, "{}{}", next_node, separator)?;
            next_node_option = &next_node.next;
        }

        write!(f, "]")
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            root: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self { Self::default() }

    pub fn push_back(&mut self, value: T) {
        let mut next_node_option = &mut self.root;
        while let Some(ref mut next_node) = next_node_option {
            next_node_option = &mut next_node.next;
        }
        *next_node_option = Some(Box::new(Node::with_value(value)));
    }

    pub fn length(&self) -> usize {
        let mut next_node_option = &self.root;
        let mut count = 0usize;

        while let Some(ref next_node) = next_node_option {
            next_node_option = &next_node.next;
            count += 1;
        }
        count
    }

    pub fn is_empty(&self) -> bool { self.length() == 0 }
}
    
#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::LinkedList;
    use super::Node;

    #[test]
    fn test_push_back() {
        let mut linked_list = LinkedList::<i32>::new();
        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);
        linked_list.push_back(4);
    }

    #[test]
    fn test_to_string() {
        let mut linked_list = LinkedList::<i32>::new();
        linked_list.push_back(1);
        linked_list.push_back(2);
        assert_eq!(linked_list.to_string(), "[1 -> 2]");
        linked_list.push_back(3);
        linked_list.push_back(4);
        assert_eq!(linked_list.to_string(), "[1 -> 2 -> 3 -> 4]");
    }

    #[test]
    fn test_length_empty() {
        let linked_list = LinkedList::<i32>::new();
        assert_eq!(linked_list.length(), 0);
        assert!(linked_list.is_empty());
    }

    #[test]
    fn test_length_not_empty() {
        let mut linked_list = LinkedList::<i32>::new();
        linked_list.push_back(2);
        assert_eq!(linked_list.length(), 1);
        assert!(!linked_list.is_empty());
    }

    #[test]
    fn test_length() {
        let mut linked_list = LinkedList::<i32>::new();
        linked_list.push_back(11);
        linked_list.push_back(12);
        linked_list.push_back(23);
        assert_eq!(linked_list.length(), 3);
    }
}  
