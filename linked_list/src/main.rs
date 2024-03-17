extern crate rust;
// use rust::linked_lists::singly_linked_list_with_tail::{SLList, SLListNode};
use rust::linked_lists::sllwt_rc::{LinkedList, Node};

// fn main() {
//     let key_vec: Vec<i64> = vec![1, 8, 27, 64, 125];
//     let mut pushed_list = SLList::from(key_vec);
//     pushed_list = pushed_list.push_back(5);
//     println!("next {:?}", &pushed_list.clone().tail.unwrap().next);
//     
//     let key_vec: Vec<i64> = vec![1, 8, 27, 64, 125, 5];
//     let mut list = SLList::from(key_vec);
//     list.updated = false;
//
//     assert_eq!(pushed_list, list);
// }

fn main() {
    let (index, value): (usize, i64) = (0, 8);
    let (index_1, value_1): (usize, i64) = (1, 56);
    let (index_2, value_2): (usize, i64) = (2, 19);
    let (index_3, value_3): (usize, i64) = (3, 80);

    let mut function_list: LinkedList<i64> = LinkedList::new();
    function_list.push_to_end(value);
    println!("tail: {:?}", function_list.tail);
    function_list.push_to_end(value_1);
    println!("tail: {:?}", function_list.tail);
    function_list.push_to_end(value_2);
    println!("tail: {:?}", function_list.tail);
    function_list.push_to_end(value_3);
    println!("tail: {:?}", function_list.tail);

    println!("\nlist: {:#?}", function_list);
}
