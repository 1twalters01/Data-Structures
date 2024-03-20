#![allow(unused_variables)]

use std::fmt::Error;
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
    pub ordered: bool,
}

impl<T> LinkedList<T> 
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug
{
    pub fn new() -> Self {
        LinkedList { head: None, tail: None, length: 0, ordered: true }
    }

    pub fn from(data_vec: Vec<T>) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();
        
        for data in data_vec {
            list.push_to_end(data);
        }

        return list;
    }

    pub fn update_indices(&mut self) {
        if self.ordered == true {
            return;
        }

        if self.length == 0 {
            self.ordered = true;
            return;
        }

        let mut current_node: Rc<RefCell<Node<T>>> = self.head.as_mut().unwrap().clone();
        let mut count = 0;
        // let continue_loop: bool = true;
        while count < self.length {
            current_node.borrow_mut().index = Some(count);
            let some_node: Option<Rc<RefCell<Node<T>>>> = current_node.clone().borrow().clone().next;
            if let Some(node) = some_node {
                current_node = node;
            } 
            else {
                self.tail = Some(current_node.clone());
            }
            count += 1;
        }

        self.ordered = true;
    }

    pub fn push_to_front(&mut self, data: T) {
        let mut node: Node<T> = Node::from(0, data);
        node.next = self.head.clone();
        self.head = Some(Rc::new(RefCell::from(node)));
        if self.tail == None {
            self.tail = self.head.clone();
        }

        self.length += 1;
        self.ordered = false;
        self.update_indices();
    }

    pub fn unordered_push_to_front(&mut self, data: T) {
        let mut node: Node<T> = Node::from(0, data);
        node.next = self.head.clone();
        self.head = Some(Rc::new(RefCell::from(node)));
        if self.tail == None {
            self.tail = self.head.clone();
        }

        self.length += 1;
        self.ordered = false;
    }

    pub fn pop_from_front(&mut self) -> Result<(), Error> {
        self.ordered = false;

        if self.head == None {
            return Err(Error);
        }

        self.head = self.head.clone().unwrap().borrow().clone().next;
        if self.head == None {
            self.ordered = true;
            self.tail = None;
        }

        self.length -= 1;
        self.update_indices();

        return Ok(());
    }

    pub fn unordered_pop_from_front(&mut self) -> Result<(), Error> {
        self.ordered = false;
        
        if self.head == None {
            return Err(Error);
        }

        self.head = self.head.clone().unwrap().borrow().clone().next;
        if self.head == None {
            self.ordered = true;
            self.tail = None;
        }

        self.length -= 1;

        return Ok(());
    }

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

    pub fn pop_from_end(&mut self) -> Result<(), Error> {
        if self.head == None {
            return Err(Error);
        }

        if self.head == self.tail {
            self.head = None;
            self.tail = None;
        }

        let mut current_node: Rc<RefCell<Node<T>>>  = self.head.clone().unwrap();
        while current_node.borrow().clone().next.unwrap().borrow().next != None {
            current_node = current_node.clone().borrow().clone().next.unwrap();
        }
        
        current_node.borrow_mut().next = None;
        self.tail = Some(current_node);
        self.length -= 1;

        return Ok(());
    }

    pub fn into_back(&self) -> Option<T> {
        match self.tail.clone() {
            Some(tail) => tail.borrow().data.as_ref().cloned(),
            None => None,
        }
    }

    pub fn find_node_by_index(&mut self, index: usize) -> Result<Rc<RefCell<Node<T>>>, Error> {
        if index >= self.length  {
            return Err(Error)
        }
        
        let mut count = 0;
        let mut current_node: Rc<RefCell<Node<T>>> = self.head.clone().unwrap();
        while current_node.borrow().clone().index.unwrap() < self.length {
            current_node.borrow_mut().index = Some(count);
            if count == index {
                break
            }
            match current_node.clone().borrow().clone().next {
                Some(node) => current_node = node,
                None => break,
            }
            count += 1;
        }

        self.ordered = true;
        return Ok(current_node);
    }

    pub fn find_node_by_value(self, data: T) -> Result<Rc<RefCell<Node<T>>>, Error> {
        let mut found_node: Option<Rc<RefCell<Node<T>>>> = None;
        let mut current_node: Rc<RefCell<Node<T>>>;

        match self.head.clone() {
            Some(node) => current_node = node,
            None => return Err(Error),
        }
        
        let found: bool = false;
        while found_node.is_none() {
            if current_node.borrow().clone().data.unwrap() == data {
                found_node = Some(current_node.clone());
                break
            }
            match current_node.clone().borrow().clone().next {
                Some(node) => current_node = node,
                None => break,
            }
            // current_node = current_node.clone().borrow().clone().next.unwrap()
        }

        if found_node.is_some() {
            return Ok(found_node.unwrap());
        }
        return Err(Error);
    }

    pub fn is_empty(&self) -> bool {
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
        let manual_list: LinkedList<i64> = LinkedList { head: None, tail: None, length: 0, ordered: true };
        let function_list: LinkedList<i64> = LinkedList::new();
        assert_eq!(manual_list, function_list);
    }

    #[test]
    fn test_empty_linked_list_from() {
        let manual_list: LinkedList<i64> = LinkedList { head: None, tail: None, length: 0, ordered: true };

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
            ordered: true,
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
            ordered: true,
        };

        let data_vec: Vec<i64> = vec![value, value_1, value_2, value_3];
        let function_list: LinkedList<i64> = LinkedList::from(data_vec);

        assert_eq!(manual_list, function_list);
 
    }

    #[test]
    pub fn test_empty_ordered_equals_true_update_indices() {
        let mut list: LinkedList<i64> = LinkedList::new();
        list.update_indices();
        assert_eq!(list, LinkedList::new());
    }

    #[test]
    pub fn test_empty_ordered_equals_false_update_indices() {
        let mut list: LinkedList<i64> = LinkedList { head: None, tail: None, length: 0, ordered: false };
        list.update_indices();
        assert_eq!(list, LinkedList::new());
    }

    #[test]
    pub fn test_one_ordered_equals_true_update_indices() {
        let data_vec: Vec<i64> = vec![2];
        let mut list: LinkedList<i64> = LinkedList::from(data_vec.clone());
        list.update_indices();
        assert_eq!(list, LinkedList::from(data_vec));
    }

    #[test]
    pub fn test_one_ordered_equals_false_update_indices() {
        let data_vec: Vec<i64> = vec![2];
        let manual_list = LinkedList::from(data_vec);

        let (index, value): (usize, i64) = (8, 2);
        let function_node: Option<Rc<RefCell<Node<i64>>>> = Some(Rc::new(RefCell::new(Node::from(index, value))));
        let mut function_list: LinkedList<i64> = LinkedList { 
            head: function_node.clone(),
            tail: function_node,
            length: 1,
            ordered: false,
        };
        function_list.update_indices();

        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_two_ordered_equals_false_update_indices() {
        let (index, value): (usize, i64) = (8, 2);
        let (index_1, value_1): (usize, i64) = (2, 37);

        let data_vec: Vec<i64> = vec![value, value_1];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let node_1: Node<i64> = Node {
            index: Some(index_1),
            data: Some(value_1), 
            next: None,
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1.clone()))),
        };
        let mut function_list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_1))),
            length: 2,
            ordered: false,
        };
        function_list.update_indices();

        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_five_ordered_equals_false_update_indices() {
        let (index, value): (usize, i64) = (8, 2);
        let (index_1, value_1): (usize, i64) = (2, 37);

        let data_vec: Vec<i64> = vec![value, value_1];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let node_1: Node<i64> = Node {
            index: Some(index_1),
            data: Some(value_1), 
            next: None,
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1.clone()))),
        };
        let mut function_list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_1))),
            length: 2,
            ordered: false,
        };
        function_list.update_indices();

        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_empty_push_to_front() {
        let value: i64 = 27;
        let data_vec: Vec<i64> = vec![value];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.push_to_front(value);
        
        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_one_push_to_front() {
        let (value, value_1): (i64, i64) = (27, 39);
        let data_vec: Vec<i64> = vec![value, value_1];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.push_to_front(value_1);
        function_list.push_to_front(value);

        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_push_to_front() {
        let (value, value_1, value_2, value_3) = (3, 54, 29, 12);
        let data_vec: Vec<i64> = vec![value, value_1, value_2, value_3];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.push_to_front(value_3);
        function_list.push_to_front(value_2);
        function_list.push_to_front(value_1);
        function_list.push_to_front(value);

        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_empty_unordered_push_to_front() {
        let value: i64 = 27;
        let data_vec: Vec<i64> = vec![value];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.unordered_push_to_front(value);
        assert_ne!(manual_list, function_list);

        function_list.update_indices();
        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_one_unordered_push_to_front() {
        let (value, value_1): (i64, i64) = (27, 39);
        let data_vec: Vec<i64> = vec![value, value_1];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.unordered_push_to_front(value_1);
        function_list.unordered_push_to_front(value);
        assert_ne!(manual_list, function_list);

        function_list.update_indices();
        assert_eq!(manual_list, function_list);

    }

    #[test]
    pub fn test_unordered_push_to_front() {
        let (value, value_1, value_2, value_3) = (3, 54, 29, 12);
        let data_vec: Vec<i64> = vec![value, value_1, value_2, value_3];
        let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.unordered_push_to_front(value_3);
        function_list.unordered_push_to_front(value_2);
        function_list.unordered_push_to_front(value_1);
        function_list.unordered_push_to_front(value);
        assert_ne!(manual_list, function_list);

        function_list.update_indices();
        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_empty_pop_from_front() {
        let mut list: LinkedList<i64> = LinkedList::new();
        let res = list.pop_from_front();
        assert_eq!(res.is_err(), true);
    }

    #[test]
    pub fn test_one_pop_from_front() {
        let manual_list: LinkedList<i64> = LinkedList::new();
        
        let data_vec: Vec<i64> = vec![2];
        let mut function_list: LinkedList<i64> = LinkedList::from(data_vec);
        let _ = function_list.pop_from_front();

        assert_eq!(manual_list, function_list)
    }

    #[test]
    pub fn test_pop_from_front() {
        let (index, value): (usize, i64) = (0, 8);
        let (index_1, value_1): (usize, i64) = (1, 56);
        let (index_2, value_2): (usize, i64) = (2, 19);
        let (index_3, value_3): (usize, i64) = (3, 80);
        let (index_4, value_4): (usize, i64) = (3, 22);

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
            ordered: true,
        };

        let data_vec: Vec<i64> = vec![value_4, value, value_1, value_2, value_3];
        let mut function_list: LinkedList<i64> = LinkedList::from(data_vec);
        let _ = function_list.pop_from_front();

        assert_eq!(manual_list, function_list);
    }

    #[test]
    pub fn test_empty_unordered_pop_from_front() {
        let mut list: LinkedList<i64> = LinkedList::new();
        let res = list.pop_from_front();
        assert_eq!(res.is_err(), true);
    }

    #[test]
    pub fn test_one_unordered_pop_from_front() {
        let manual_list: LinkedList<i64> = LinkedList::new();
        
        let data_vec: Vec<i64> = vec![2];
        let mut function_list: LinkedList<i64> = LinkedList::from(data_vec);
        let _ = function_list.unordered_pop_from_front();
        assert_eq!(manual_list, function_list)
    }

    #[test]
    pub fn test_unordered_pop_from_front() {
        let (index, value): (usize, i64) = (0, 8);
        let (index_1, value_1): (usize, i64) = (1, 56);
        let (index_2, value_2): (usize, i64) = (2, 19);
        let (index_3, value_3): (usize, i64) = (3, 80);
        let (index_4, value_4): (usize, i64) = (3, 22);

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
            ordered: true,
        };

        let data_vec: Vec<i64> = vec![value_4, value, value_1, value_2, value_3];
        let mut function_list: LinkedList<i64> = LinkedList::from(data_vec);
        let _ = function_list.unordered_pop_from_front();
        assert_ne!(manual_list, function_list);

        function_list.update_indices();
        assert_eq!(manual_list, function_list);
    }

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
            ordered: true,
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
            ordered: true,
        };

        let mut function_list: LinkedList<i64> = LinkedList::new();
        function_list.push_to_end(value);
        function_list.push_to_end(value_1);
        function_list.push_to_end(value_2);
        function_list.push_to_end(value_3);

        assert_eq!(manual_list, function_list);
    }
    
    #[test]
    fn test_empty_pop_from_end() {
        let mut list: LinkedList<i64> = LinkedList::new();
        assert_eq!(true, list.pop_from_end().is_err())
    }

    #[test]
    fn test_one_pop_from_end() {
        let value: i64 = 8;
        let mut function_list: LinkedList<i64> = LinkedList::new();
        let _ = function_list.pop_from_end();

        let manual_list: LinkedList<i64> = LinkedList::new();
        assert_eq!(manual_list, function_list)
    }

    #[test]
    fn test_pop_from_end() {
        let (index, value): (usize, i64) = (0, 8);
        let (index_1, value_1): (usize, i64) = (1, 56);
        let (index_2, value_2): (usize, i64) = (2, 19);
        let (index_3, value_3): (usize, i64) = (3, 80);
        let (index_4, value_4): (usize, i64) = (4, 20);

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
            ordered: true,
        };

        let data_vec: Vec<i64> = vec![value, value_1, value_2, value_3, value_4];
        let mut function_list: LinkedList<i64> = LinkedList::from(data_vec);
        let _ = function_list.pop_from_end();

        assert_eq!(manual_list, function_list);
    }

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
    fn test_empty_find_node_by_index() {
        let mut list: LinkedList<i64> = LinkedList::new();
        let res = list.find_node_by_index(1);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_zeroth_index_find_node_by_index() {
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
            next: Some(Rc::new(RefCell::new(node_2.clone()))),
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1))),
        };
        let mut list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node.clone()))),
            tail: Some(Rc::new(RefCell::new(node_3))),
            length: 4,
            ordered: true,
        };

        let function_node: Rc<RefCell<Node<i64>>> = list.find_node_by_index(0).unwrap();
        let manual_node: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node));

        assert_eq!(manual_node, function_node);
    }

    #[test]
    fn test_last_index_find_node_by_index() {
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
            next: Some(Rc::new(RefCell::new(node_2.clone()))),
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1))),
        };
        let mut list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_3.clone()))),
            length: 4,
            ordered: true,
        };

        let function_node: Rc<RefCell<Node<i64>>> = list.find_node_by_index(3).unwrap();
        let manual_node: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node_3));

        assert_eq!(manual_node, function_node);
    }

    #[test]
    fn test_inside_index_find_node_by_index() {
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
            next: Some(Rc::new(RefCell::new(node_2.clone()))),
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1))),
        };
        let mut list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_3))),
            length: 4,
            ordered: true,
        };

        let function_node: Rc<RefCell<Node<i64>>> = list.find_node_by_index(2).unwrap();
        let manual_node: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node_2));

        assert_eq!(manual_node, function_node);

    }

    #[test]
    fn test_outside_index_find_node_by_index() {
        let value: i64 = 43;
        let value_1: i64 = 17;
        let value_2: i64 = 99;
        let data_vec: Vec<i64> = vec![value, value_1, value_2];
        let mut list: LinkedList<i64> = LinkedList::from(data_vec);
        let res = list.find_node_by_index(3);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_empty_invalid_find_node_by_value() {
        let list: LinkedList<i64> = LinkedList::new();
        let res = list.find_node_by_value(105);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_invalid_find_node_by_value() {
        let value: i64 = 43;
        let value_1: i64 = 17;
        let value_2: i64 = 99;
        let data_vec: Vec<i64> = vec![value, value_1, value_2];
        let list: LinkedList<i64> = LinkedList::from(data_vec);
        let res = list.find_node_by_value(105);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_valid_first_find_node_by_value() {
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
        let list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node.clone()))),
            tail: Some(Rc::new(RefCell::new(node_3.clone()))),
            length: 4,
            ordered: true,
        };

        let function_node: Rc<RefCell<Node<i64>>> = list.find_node_by_value(8).unwrap();
        let manual_node: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node));

        assert_eq!(manual_node, function_node);
    }

    #[test]
    fn test_valid_last_find_node_by_value() {
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
        let list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node.clone()))),
            tail: Some(Rc::new(RefCell::new(node_3.clone()))),
            length: 4,
            ordered: true,
        };

        let function_node: Rc<RefCell<Node<i64>>> = list.find_node_by_value(80).unwrap();
        let manual_node: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node_3));

        assert_eq!(manual_node, function_node);
    }

    #[test]
    fn test_valid_find_node_by_value() {
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
            next: Some(Rc::new(RefCell::new(node_2.clone()))),
        };
        let node: Node<i64> = Node {
            index: Some(index),
            data: Some(value), 
            next: Some(Rc::new(RefCell::new(node_1))),
        };
        let list: LinkedList<i64> = LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            tail: Some(Rc::new(RefCell::new(node_3))),
            length: 4,
            ordered: true,
        };

        let function_node: Rc<RefCell<Node<i64>>> = list.find_node_by_value(19).unwrap();
        let manual_node: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node_2));

        assert_eq!(manual_node, function_node);
    }

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
