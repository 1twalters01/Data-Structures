#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, PartialEq, Debug)]
pub struct Node<T> {
    pub index: Option<usize>,
    pub data: Option<T>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new() -> Node<T> {
        Node { data: None, next: None, index: None }
    }

    pub fn from(index: usize, data: T) -> Node<T> {
        Node { data: Some(data), next: None, index: Some(index) }
    }
}

#[derive(PartialEq, Debug)]
pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Rc<RefCell<Node<T>>>>,
    pub length: usize,
}

impl<T> LinkedList<T> 
where
    T: Clone
{
    pub fn new() -> Self {
        LinkedList { head: None, tail: None, length: 0 }
    }

    pub fn from(data_vec: Vec<T>) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();
        
        for data in data_vec {
            list.push_to_end(data);
        }

        return list;
    }

    pub fn push_to_front(&mut self, data: T) {}

    pub fn pop_from_front(&mut self) {}

    pub fn into_front(self) -> Option<T> {
        match self.head {
            Some(head) => head.borrow().data.as_ref().cloned(),
            None => None,
        }
    }

    pub fn push_to_end(&mut self, data: T) {
        let index = self.length;
        let new_node = Rc::new(RefCell::new(Node::from(index, data)));

        match self.tail.take() {
            Some(tail) => tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn pop_from_end(&mut self) {}

    pub fn into_back(self) -> Option<T> {
        match self.tail {
            Some(tail) => tail.borrow().data.as_ref().cloned(),
            None => None,
        }
    }

    pub fn find_by_index(self, index: usize) {}

    pub fn find_by_value(self, value: T) {}

    pub fn is_empty(self) -> bool {
        if self.length > 0 {
            return false;
        }

        return true;
    }

    pub fn add_before(index: usize) {}

    pub fn add_after(index: usize) {}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let manual_node: Node<i64> = Node { data: None, next: None, index: None };
        let function_node: Node<i64> = Node::new();
        assert_eq!(manual_node, function_node);
    }

    #[test]
    fn test_node_from() {
        let (index, value): (usize, i64) = (0, 8);
        let manual_node: Node<i64> = Node { data: Some(value), next: None, index: Some(index) };
        let function_node: Node<i64> = Node::from(index, value);
        assert_eq!(manual_node, function_node);
    }

    #[test]
    fn test_linked_list_new() {
        let manual_list: LinkedList<i64> = LinkedList { head: None, tail: None, length: 0 };
        let function_list: LinkedList<i64> = LinkedList::new();
        assert_eq!(manual_list, function_list);
    }

    #[test]
    fn test_empty_linked_list_from() {
        let manual_list: LinkedList<i64> = LinkedList { head: None, tail: None, length: 0 };

        let data_vec: Vec<i64> = vec![];
        let function_list: LinkedList<i64> = LinkedList::from(data_vec);
        assert_eq!(manual_list, function_list);
    }

    #[test]
    fn test_one_linked_list_from() {
        let (index, value): (usize, i64) = (0, 8);
        let manual_list = LinkedList {
            head: Some(Rc::new(RefCell::new(Node::from(index, value)))),
            tail: Some(Rc::new(RefCell::new(Node::from(index, value)))),
            length: 1,
        };

        let data_vec: Vec<i64> = vec![value];
        let function_list: LinkedList<i64> = LinkedList::from(data_vec);
        assert_eq!(manual_list, function_list);

    }

    #[test]
    fn test_four_linked_list_from() {
        let (index, value): (usize, i64) = (0, 8);
        let (index_1, value_1): (usize, i64) = (1, 56);
        let (index_2, value_2): (usize, i64) = (2, 19);
        let (index_3, value_3): (usize, i64) = (3, 80);

        let node_3: Node<i64> = Node {
            index: Some(index_3),
            data: Some(value_3), 
            next: None,
        };
        let node_2: Node<i64> = Node {
            index: Some(index_2),
            data: Some(value_2), 
            next: Some(Rc::new(RefCell::new(node_3.clone()))),
        };
        let node_1: Node<i64> = Node {
            index: Some(index_1),
            data: Some(value_1), 
            next: Some(Rc::new(RefCell::new(node_2))),
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1))),
        };
        let manual_list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_3))),
            length: 4,
        };

        let data_vec: Vec<i64> = vec![value, value_1, value_2, value_3];
        let function_list: LinkedList<i64> = LinkedList::from(data_vec);

        assert_eq!(manual_list, function_list);
 
    }

    #[test]
    pub fn push_to_front() {}

    #[test]
    pub fn pop_from_front() {}


    #[test]
    fn test_empty_into_front() {
        let list: LinkedList<i64> = LinkedList::new();
        assert_eq!(None, list.into_front());
    }

    #[test]
    fn test_into_front() {
        let value: i64 = 19;
        let value_1: i64 = 3;
        let value_2:i64 = 2;
        let data_vec: Vec<i64> = vec![value, value_1, value_2];
        let list: LinkedList<i64> = LinkedList::from(data_vec);

        assert_eq!(value, list.into_front().unwrap());
    }

    #[test]
    fn test_empty_push_to_end() {
        let (index, value): (usize, i64) = (0, 8);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.push_to_end(value);

        let manual_list = LinkedList {
            head: Some(Rc::new(RefCell::new(Node::from(index, value)))),
            tail: Some(Rc::new(RefCell::new(Node::from(index, value)))),
            length: 1,
        };

        assert_eq!(manual_list, function_list);
    }

    #[test]
    fn test_push_to_end() {
        let (index, value): (usize, i64) = (0, 8);
        let (index_1, value_1): (usize, i64) = (1, 56);
        let (index_2, value_2): (usize, i64) = (2, 19);
        let (index_3, value_3): (usize, i64) = (3, 80);

        let node_3: Node<i64> = Node {
            index: Some(index_3),
            data: Some(value_3), 
            next: None,
        };
        let node_2: Node<i64> = Node {
            index: Some(index_2),
            data: Some(value_2), 
            next: Some(Rc::new(RefCell::new(node_3.clone()))),
        };
        let node_1: Node<i64> = Node {
            index: Some(index_1),
            data: Some(value_1), 
            next: Some(Rc::new(RefCell::new(node_2))),
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1))),
        };
        let manual_list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_3))),
            length: 4,
        };

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.push_to_end(value);
        function_list.push_to_end(value_1);
        function_list.push_to_end(value_2);
        function_list.push_to_end(value_3);

        assert_eq!(manual_list, function_list);
    }
    
    #[test]
    fn test_empty_pop_from_end() {}

    #[test]
    fn test_one_pop_from_end() {}

    #[test]
    fn test_pop_from_end() {}

    #[test]
    fn test_empty_into_back() {
        let list: LinkedList<i64> = LinkedList::new();
        assert_eq!(None, list.into_back());
    }

    #[test]
    fn test_into_back() {
        let value: i64 = 19;
        let value_1: i64 = 3;
        let value_2:i64 = 2;
        let data_vec: Vec<i64> = vec![value, value_1, value_2];
        let list: LinkedList<i64> = LinkedList::from(data_vec);

        assert_eq!(value_2, list.into_back().unwrap());
    }

    #[test]
    fn test_empty_find_by_index() {}

    #[test]
    fn test_zeroth_index_find_by_index() {}

    #[test]
    fn test_last_index_find_by_index() {}

    #[test]
    fn test_inside_index_find_by_index() {}

    #[test]
    fn test_outside_index_find_by_index() {}

    #[test]
    fn find_empty_invalid_by_value() {}

    #[test]
    fn find_none_invalid_by_value() {}

    #[test]
    fn find_invalid_by_value() {}

    #[test]
    fn find_valid_first_by_value() {}

    #[test]
    fn find_valid_last_by_value() {}

    #[test]
    fn find_valid_by_value() {}

    #[test]
    fn test_empty_is_empty() {
        let list: LinkedList<i64> = LinkedList::new();
        assert_eq!(true, list.is_empty());
    }

    #[test]
    fn test_some_is_empty() {
        let data_vec: Vec<i64> = vec![1, 445,2];
        let list: LinkedList<i64> = LinkedList::from(data_vec);
        assert_eq!(false, list.is_empty());
    }

    #[test]
    pub fn test_empty_add_before() {}

    #[test]
    pub fn test_one_add_before() {}

    #[test]
    pub fn test_first_add_before() {}

    #[test]
    pub fn test_last_add_before() {}

    #[test]
    pub fn test_add_before() {}

    #[test]
    pub fn test_empty_add_after() {}

    #[test]
    pub fn test_one_add_after() {}

    #[test]
    pub fn test_first_add_after() {}

    #[test]
    pub fn test_last_add_after() {}

    #[test]
    pub fn test_add_after() {}
}
