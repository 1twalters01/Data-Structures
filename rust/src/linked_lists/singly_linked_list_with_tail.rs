#![allow(dead_code)]

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

#[derive(Clone, PartialEq, Debug)]
pub struct SLList<T>
where
    T: Clone,
{
    pub head: Box<SLListNode<T>>,
    pub tail: Box<SLListNode<T>>,
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
    ///     head: Box::new(SLListNode::new()),
    ///     tail: Box::new(SLListNode::new()),
    ///     length: 0,
    /// };
    ///
    /// assert_eq!(new_list, list);
    /// ```
    pub fn new() -> SLList<T> {
        SLList {
            head: Box::new(SLListNode::new()),
            tail: Box::new(SLListNode::new()),
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
    ///     head: Box::new(node_1),
    ///     tail: Box::new(node_4),
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

        sllist.tail = Box::new(node_vec.last().unwrap().clone());
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

        sllist.head = Box::new(node_holder.unwrap());

        return sllist.clone();
    }

    // fn PushFront(self, key: T) -> Result<(), Error> {
    //     let node = SLListNode::from(key);
    //     node.next = Some(Box::new(self));
    //
    // }
    //
    // fn PopFront(self) -> Result<(), Error> {}
    // fn TopFront(self) -> Result<(), Error> {}
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
            head: Box::new(SLListNode::new()),
            tail: Box::new(SLListNode::new()),
            length: 0,
        };

        assert_eq!(new_list, list);
    }

    #[test]
    fn test_empty_list_from() {
        let key_vec: Vec<i64> = Vec::new();
        let new_list: SLList<i64> = SLList::from(key_vec);
        let list: SLList<i64> = SLList {
            head: Box::new(SLListNode::new()),
            tail: Box::new(SLListNode::new()),
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
            head: Box::new(node.clone()),
            tail: Box::new(node),
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
            head: Box::new(node_1),
            tail: Box::new(node_4),
            length: 4,
        };

        assert_eq!(new_list, list);
    }
}