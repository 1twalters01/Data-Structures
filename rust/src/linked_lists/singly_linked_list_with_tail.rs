#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{Error, ErrorKind};

/// An iterable node that forms a non-contiguous linked list.
///
/// The node has the following fields:
///     key: This is the object being stored in the linked list node
///     next: The next node. This can be a node or none.
///           A box is used to put it on the stack.
///     index: The index of the linked list node.
///
/// # Examples
/// ```
/// use rust::linked_lists::singly_linked_list_with_tail::SLListNode;
///
/// let new_node: SLListNode<i64> = SLListNode::new();
/// let node: SLListNode<i64> = SLListNode { key: None, next: None, index: 0 };
/// assert_eq!(new_node, node);
///
/// let (index, value): (usize, i64) = (0, 8);
/// let new_node: SLListNode<i64> = SLListNode::from(value, index);
/// let node: SLListNode<i64> = SLListNode { key: Some(value), next: None, index: 0 };
/// assert_eq!(new_node, node);
///
/// let mut node_1 = SLListNode::from(8, 0);
/// let mut node_2 = SLListNode::from(20, 1);
/// let node_3 = SLListNode::from(12, 3);
/// node_2.next = Some(Box::new(node_3.clone()));
/// node_1.next = Some(Box::new(node_2.clone()));
///
/// assert_eq!(node_1.next().unwrap().next(), Some(node_3));
/// assert_eq!(node_2.next().unwrap().next(), None);
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct SLListNode<T>
where
    T: Clone,
    SLListNode<T>: Iterator,
{
    pub key: Option<T>,
    pub next: Option<Box<SLListNode<T>>>,
    pub index: usize,
}

impl<T> Iterator for SLListNode<T>
where
    T: Clone,
{
    type Item = SLListNode<T>;
    fn next(&mut self) -> Option<Self> {
        match self.clone().next {
            Some(value) => return Some(*value),
            None => return None,
        }
    }
}

impl<T> SLListNode<T>
where
    T: Clone,
    SLListNode<T>: Iterator,
{
    /// Creates a new, empty instance of a linked list node that lives on the stack.
    ///
    /// # Example
    /// ```
    /// use rust::linked_lists::singly_linked_list_with_tail::SLListNode;
    ///
    /// let new_node: SLListNode<i64> = SLListNode::new();
    /// let node: SLListNode<i64> = SLListNode { key: None, next: None, index: 0 };
    /// assert_eq!(new_node, node);
    /// ```
    pub fn new() -> SLListNode<T> {
        SLListNode {
            key: None,
            next: None,
            index: 0,
        }
    }

    /// Creates a new instance of a linked list node that lives on the stack.
    /// The key must have the Clone trait.
    ///
    /// # Example
    /// ```
    /// use rust::linked_lists::singly_linked_list_with_tail::SLListNode;
    /// let mut node_1 = SLListNode::from(8, 0);
    /// let mut node_2 = SLListNode::from(20, 1);
    /// let node_3 = SLListNode::from(12, 3);
    /// node_2.next = Some(Box::new(node_3.clone()));
    /// node_1.next = Some(Box::new(node_2.clone()));
    ///
    /// assert_eq!(node_1.next().unwrap().next(), Some(node_3));
    /// assert_eq!(node_2.next().unwrap().next(), None);
    /// ```
    pub fn from(key: T, index: usize) -> SLListNode<T> {
        SLListNode {
            key: Some(key),
            next: None,
            index,
        }
    }
}

/// Todo upon completion 
#[derive(Clone, PartialEq, Debug)]
pub struct SLList<T>
where
    T: Clone,
{
    pub updated: bool,
    pub head: Option<Box<SLListNode<T>>>, // Wrap in option
    pub tail: Option<Box<SLListNode<T>>>, // Wrap in option
    pub length: usize,
}

impl<T> SLList<T>
where
    T: Clone + PartialEq,
{
    /// Creates a new, empty instance of a linked list that lives on the stack.
    ///
    /// # Example
    /// ```
    /// use rust::linked_lists::singly_linked_list_with_tail::{ SLList, SLListNode };
    ///
    /// let new_list: SLList<i64> = SLList::new();
    /// let list: SLList<i64> = SLList {
    ///     updated: true,
    ///     head: None,
    ///     tail: None,
    ///     length: 0,
    /// };
    ///
    /// assert_eq!(new_list, list);
    /// ```
    pub fn new() -> SLList<T> {
        SLList {
            updated: true,
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Creates a new instance of a linked list that lives on the stack.
    /// The key must have the Clone trait.
    ///
    /// # Example
    /// ```
    /// use rust::linked_lists::singly_linked_list_with_tail::{ SLList, SLListNode };
    ///
    /// let (index_1, value_1): (usize, i64) = (0, 1);
    /// let (index_2, value_2): (usize, i64) = (1, 8);
    /// let (index_3, value_3): (usize, i64) = (2, 27);
    /// let (index_4, value_4): (usize, i64) = (3, 64);
    ///
    /// let key_vec: Vec<i64> = Vec::from([value_1, value_2, value_3, value_4]);
    /// let new_list: SLList<i64> = SLList::from(key_vec);
    ///
    /// let mut node_1 = SLListNode::from(value_1, index_1);
    /// let mut node_2 = SLListNode::from(value_2, index_2);
    /// let mut node_3 = SLListNode::from(value_3, index_3);
    /// let node_4 = SLListNode::from(value_4, index_4);
    ///
    /// node_3.next = Some(Box::new(node_4.clone()));
    /// node_2.next = Some(Box::new(node_3.clone()));
    /// node_1.next = Some(Box::new(node_2));
    ///
    /// let list: SLList<i64> = SLList {
    ///     updated: true,
    ///     head: Some(Box::new(node_1)),
    ///     tail: Some(Box::new(node_4)),
    ///     length: 4,
    /// };
    ///
    /// assert_eq!(new_list, list);
    /// ```
    pub fn from(key_vec: Vec<T>) -> SLList<T> {
        let mut sllist = SLList::new();
        if key_vec.len() == 0 {
            return sllist;
        }

        let node_vec: Vec<SLListNode<T>> = key_vec
            .into_iter()
            .enumerate()
            .map(|(index, key)| SLListNode::from(key, index))
            .collect();

        sllist.tail = Some(Box::new(node_vec.last().unwrap().clone()));
        sllist.length = node_vec.len();

        let mut node_holder: Option<SLListNode<T>> = None;
        for mut node in node_vec.into_iter().rev() {
            if node_holder == None {
                node.next = None;
            } else {
                node.next = Some(Box::new(node_holder.unwrap()));
            }
            node_holder = Some(node);
        }

        sllist.head = Some(Box::new(node_holder.unwrap()));

        return sllist.clone();
    }

    pub fn update(&mut self) -> Result<(), Error> {
        if self.head.is_none() {
            return Ok(())
        }

        let mut count = 0;
        let mut new_self: SLList<T> = SLList::new();
        let mut node_current: SLListNode<T> = *self.head.unwrap();
        while node_current.next().is_some() {
            node_current.next = None;
            node_current.index = count;
            count += 1;
            new_self.push_back(node_current.key);
        }

        self = &mut new_self;
        return Ok(());
    }

    /// Creates a node and push it to the front of the linked list.
    ///
    /// # Example
    /// ```
    /// use rust::linked_lists::singly_linked_list_with_tail::{ SLList, SLListNode };
    ///
    /// let mut pushed_list = SLList::new();
    /// let _ = pushed_list.push_front(5);
    /// let list = SLList::from(Vec::from([5]));
    ///
    /// assert_eq!(pushed_list, list);
    /// ```
    pub fn push_front(&mut self, key: T) -> Result<(), Error> {
        let mut node: SLListNode<T> = SLListNode::from(key, 0);
        node.next = self.clone().head;
        self.head = Some(Box::new(node));

        if self.tail.is_none() {
            self.tail = self.clone().head;
        }

        self.length += 1;
        self.updated = false;

        return Ok(())
    }

    /// Removes the front node of the linked list.
    ///
    /// # Example
    /// ```
    /// ```
    fn pop_front(&mut self) -> Result<(), Error> {
        if self.head.is_none() {
            let error: Error = Error::new(
                ErrorKind::NotFound,
                String::from("Cannot delete element from empty list"),
            );

            return Err(error);
        }

        self.head = match self.clone().head.unwrap().next() {
            None => None,
            Some(value) => Some(Box::new(value)),
        };

        if self.head.is_none() {
            self.tail = None;
        }

        self.length -= 1;
        self.updated = false;

        return Ok(());
    }

    fn top_front(self) -> Option<T> {
        if self.head.is_none() {
            return None
        }

        return self.head.unwrap().key;
    }
    // fn PushBack(self, key: T) -> Result<(), Error> {}
    // fn PopBack(self) -> Result<(), Error> {}
    // fn TopBack(self) -> T {}
    // fn Find(self, Key: T) -> bool {}
    // fn Erase(self, Key: T) -> Result<(), Error> {}
    // fn Empty(self) -> bool {}
    // fn AddBefore(self, Node, key: T) -> Result<(), Error> {}
    // fn AddAfter(self, node: SLListNode<T>, key: T) -> Result<(), Error> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let new_node: SLListNode<i64> = SLListNode::new();
        let node: SLListNode<i64> = SLListNode {
            key: None,
            next: None,
            index: 0,
        };

        assert_eq!(new_node, node);
    }

    #[test]
    fn test_node_from() {
        let (index, value): (usize, i64) = (0, 8);
        let new_node: SLListNode<i64> = SLListNode::from(value, index);
        let node: SLListNode<i64> = SLListNode {
            key: Some(value),
            next: None,
            index: 0,
        };

        assert_eq!(new_node, node);
    }

    #[test]
    fn test_node_next() {
        let mut node_1 = SLListNode::from(8, 0);
        let mut node_2 = SLListNode::from(20, 1);
        let node_3 = SLListNode::from(12, 3);

        node_2.next = Some(Box::new(node_3.clone()));
        node_1.next = Some(Box::new(node_2.clone()));

        assert_eq!(node_1.next().unwrap().next(), Some(node_3));
        assert_eq!(node_2.next().unwrap().next(), None);
    }

    #[test]
    fn test_list_new() {
        let new_list: SLList<i64> = SLList::new();
        let list: SLList<i64> = SLList {
            updated: true,
            head: None,
            tail: None,
            length: 0,
        };

        assert_eq!(new_list, list);
    }

    #[test]
    fn test_empty_list_from() {
        let key_vec: Vec<i64> = Vec::new();
        let new_list: SLList<i64> = SLList::from(key_vec);
        let list: SLList<i64> = SLList {
            updated: true,
            head: None,
            tail: None,
            length: 0,
        };

        assert_eq!(new_list, list);
    }

    #[test]
    fn test_one_list_from() {
        let (index, value): (usize, i64) = (0, 1);
        let key_vec: Vec<i64> = Vec::from([value]);
        let new_list: SLList<i64> = SLList::from(key_vec);

        let node = SLListNode::from(value, index);
        let list: SLList<i64> = SLList {
            updated: true,
            head: Some(Box::new(node.clone())),
            tail: Some(Box::new(node)),
            length: 1,
        };

        assert_eq!(new_list, list);
    }

    #[test]
    fn test_four_list_from() {
        let (index_1, value_1): (usize, i64) = (0, 1);
        let (index_2, value_2): (usize, i64) = (1, 8);
        let (index_3, value_3): (usize, i64) = (2, 27);
        let (index_4, value_4): (usize, i64) = (3, 64);

        let key_vec: Vec<i64> = Vec::from([value_1, value_2, value_3, value_4]);
        let new_list: SLList<i64> = SLList::from(key_vec);

        let mut node_1 = SLListNode::from(value_1, index_1);
        let mut node_2 = SLListNode::from(value_2, index_2);
        let mut node_3 = SLListNode::from(value_3, index_3);
        let node_4 = SLListNode::from(value_4, index_4);

        node_3.next = Some(Box::new(node_4.clone()));
        node_2.next = Some(Box::new(node_3.clone()));
        node_1.next = Some(Box::new(node_2));

        let list: SLList<i64> = SLList {
            updated: true,
            head: Some(Box::new(node_1)),
            tail: Some(Box::new(node_4)),
            length: 4,
        };

        assert_eq!(new_list, list);
    }

    #[test]
    fn test_empty_push_front() {
        let mut pushed_list = SLList::new();
        let _ = pushed_list.push_front(5);

        let mut list = SLList::from(Vec::from([5]));
        list.updated = false;

        assert_eq!(pushed_list, list);
    }

    #[test]
    fn test_push_front() {
        let key_vec: Vec<i64> = vec![1, 8, 27, 64, 125];
        let mut pushed_list = SLList::from(key_vec);
        let _ = pushed_list.push_front(5);
        let _ = pushed_list.update();

        let key_vec: Vec<i64> = vec![5, 1, 8, 27, 64, 125];
        let list = SLList::from(key_vec);

        assert_eq!(pushed_list, list);
    }

    fn test_empty_pop_front() {
        let mut list: SLList<i64> = SLList::new();
        let pop_result: Result<(), Error> = list.pop_front();

        assert_eq!(pop_result.is_err(), true);
    }

    fn test_empty_top_front() {
        let list: SLList<i64> = SLList::new();
        assert_eq!(list.top_front(), None);
    }

    fn test_top_front() {
        let key_vec: Vec<i64> = vec![1, 8, 27, 64];
        let list: SLList<i64> = SLList::from(key_vec);

        assert_eq!(list.top_front().unwrap(), 1);
    }
}
