use std::{cell::RefCell, rc::Rc};
extern crate rust;
use rust::linked_lists::sllwt_rc_refcell::{LinkedList, Node};

// fn main() {
//     let data_vec: Vec<i64> = vec![2];
//     let manual_list = LinkedList::from(data_vec);
//
//     let (index, value): (usize, i64) = (8, 2);
//     let function_node: Option<Rc<RefCell<Node<i64>>>> = Some(Rc::new(RefCell::new(Node::from(index, value))));
//     let mut function_list: LinkedList<i64> = LinkedList {
//         head: function_node.clone(),
//         tail: function_node,
//         length: 1,
//         ordered: false,
//     };
//     function_list.update_indices();
//
//     assert_eq!(manual_list, function_list);
// }

fn main() {
    let (index, value): (usize, i64) = (8, 2);
    let (index_1, value_1): (usize, i64) = (2, 37);

    // let data_vec: Vec<i64> = vec![value, value_1];
    // let manual_list: LinkedList<i64> = LinkedList::from(data_vec);

    let node_1: Node<i64> = Node {
        index: Some(index_1),
        data: Some(value_1),
        next: None,
    };
    let rc_refcell_node_1: Rc<RefCell<Node<i64>>> = Rc::new(RefCell::new(node_1));
    let node: Node<i64> = Node {
        index: Some(index),
        data: Some(value),
        next: Some(rc_refcell_node_1.clone()),
    };
    let mut function_list: LinkedList<i64> = LinkedList {
        head: Some(Rc::new(RefCell::new(node))),
        tail: Some(rc_refcell_node_1.clone()),
        length: 2,
        ordered: false,
    };
    function_list.update_indices();

    // println!("list: {:#?}", manual_list);
    println!("list: {:#?}", function_list);
}
