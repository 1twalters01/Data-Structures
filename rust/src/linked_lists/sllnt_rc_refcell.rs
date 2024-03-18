#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, PartialEq, Debug)]
pub struct Node<T> {}

impl<T> Node<T> {}

#[derive(PartialEq, Debug)]
pub struct LinkedList<T> {}

impl<T> LinkedList<T> {
    pub fn new() -> Self {}

    pub fn from(data_vec: Vec<T>) -> Self {}

    pub fn push_to_front(&mut self, data: T) {}

    pub fn pop_from_front(&mut self) {}

    pub fn into_front(self) -> Option<T> {}

    pub fn push_to_end(&mut self, data: T) {}

    pub fn pop_from_end(&mut self) {}

    pub fn into_back(self) -> Option<T> {}

    pub fn find_by_index(self, index: usize) {}

    pub fn find_by_value(self, value: T) {}

    pub fn is_empty(self) -> bool {}

    pub fn add_before(index: usize) {}

    pub fn add_after(index: usize) {}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {}

    #[test]
    fn test_node_from() {}

    #[test]
    fn test_linked_list_new() {}

    #[test]
    fn test_empty_linked_list_from() {}

    #[test]
    fn test_one_linked_list_from() {}

    #[test]
    fn test_four_linked_list_from() {}

    #[test]
    pub fn test_empty_push_to_front() {}

    #[test]
    pub fn test_push_to_front() {}

    #[test]
    pub fn test_empty_pop_from_front() {}

    #[test]
    pub fn test_one_pop_from_front() {}

    #[test]
    pub fn test_pop_from_front() {}

    #[test]
    pub fn pop_from_front() {}


    #[test]
    fn test_empty_into_front() {}

    #[test]
    fn test_into_front() {}

    #[test]
    fn test_empty_push_to_end() {}

    #[test]
    fn test_push_to_end() {}
    
    #[test]
    fn test_empty_pop_from_end() {}

    #[test]
    fn test_one_pop_from_end() {}

    #[test]
    fn test_pop_from_end() {}

    #[test]
    fn test_empty_into_back() {}

    #[test]
    fn test_into_back() {}

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
    fn test_empty_is_empty() {}

    #[test]
    fn test_some_is_empty() {}

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
