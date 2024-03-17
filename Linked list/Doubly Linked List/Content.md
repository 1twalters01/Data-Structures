# Doubly Linked List

A doubly linked list makes adding before and popping the back cheap.
The problem was that although we had a way to go from one element to the next, we had no way to get back. A doubly linked list has a way to go back.

We implement by having a node with a value, a next pointer, and a previous pointer.

## Pseudocode

### pushBack(key)
node = new node
node.key = key
node.next = none
if tail = none
    head and tail = node
    node.prev = none
else
    tail.next = node
    node.prev = tail
    tail = node

### PopBack()
if head = none
    return Error: empty list

if head = tail
    head = tail = none
else
    tail = tail.prev
    tail.next = none

### AddAfter(node, key)
node2 = new node
node2.key = key
node2.next = node.next
node2.prev = node
node.next = node2
if node2.next != none
    node2.next.prev = node2
if tail = node
    tail = node2

### AddBefore
node2 = new node
node2.key = key
node2.next = node
node2.prev = node.prev
node.prev = node2
if node2.prev != none
    node2.prev.next = node2
if head = node
    head = node2

## Cost
|Doubly Linked List|no tail|with tail|
|PushFront(Key)|*O(1)*|*O(1)*|
|TopFront()|*O(1)*|*O(1)*|
|PopFront()|*O(1)*|*O(1)*|
|PushBack(Key)|*O(1)*|*O(1)*|
|TopBack()|*O(1)*|*O(1)*|
|PopBack()|*O(1)*|*O(1)*|
|Find(Key)|*O(n)*|*O(n)*|
|Erase(key)|*O(1)*|*O(1)*|
|Empty()|*O(n)*|*O(n)*|
|AddBefore(Node, Key)|*O(1)*|*O(1)*|
|AddAfter(Node, Key)|*O(1)*|*O(1)*|

## Summary
Linked lists are *O(1)* constant time to do things insert at or remove from the front unlike arrays which are order n.
Linked lists are *O(n)* to find an arbitrary element as elements are not contiguous
Arrays are *O(1)* to find an arbitrary element
Linked lists are *O(1)* to insert between nodes or remove a node
Arrays are *O(n)* to insert between nodes or remove a node

Things like binary search are easy with an array but hard with a linked list because of this.
