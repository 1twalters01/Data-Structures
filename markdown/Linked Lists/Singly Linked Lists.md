# Singly Linked List
We have a head pointer which points to a node, which points to another node... and eventually to one that doesn't point to anything further.

A node contains a key and a next pointer.

## API
Type Method/Property - definition

Result<(), Err> PushFront(Key) - add an element to the front of the list
Result<(), Err> PopFront() - remove the front element from the list
Key TopFront() - return the front element from the list
Result<(), Err> PushBack(Key) or Append - add an element to the last of the list
Result<(), Err> PopBack() - remove the last element from the list
Key TopBack() - return the last element of the list
Boolean Find(Key) - is key in list?
Result<(), Err> Erase(Key) - remove key from list
Boolean Empty() - checks if the key is empty
Result<(), Err> AddBefore(Node, Key) - adds key before a certain node
Result<(), Err> AddAfter(Node, Key) - adds key after a certain node

## Pseudocode
### PushFront(key)
Allocate a new node
Set its key
set its next property to point to the old head
set the head to point to this new node
if the tail is none
    set it to be the head

### PopFront()
if head = none -> Error: Empty list
set the head to be the head's next
if the head is none
    set the tail to none

### PushBack(key)
Allocate a new node
Set its key
Set the next node to none
if the tail is nill
    update the head and tail to be this new node
else set the tail's next property to be the node and update the tail to be the new node

### PopBack
if head = none -> Error: Empty list
if head = tail then set head and tail to none
else
    start at the head
    while the next.next != none
        go to the next node
    set the current element's next to none
    set the tail to the current element

If we do a PopBack on a list containing the keys (in order) a, b, c, and d then the loop will occur twice.

### AddAfter(node, key)
allocate node2
set the key of node2
set the next property of node2 to node
set node.next to be node2
if tail = node
    set tail to node 2

### AddBack
This has the same problem as PopBack

## Cost
|Singly Linked List|no tail|with tail|
|PushFront(Key)|*O(1)*|*O(1)*|
|TopFront()|*O(1)*|*O(1)*|
|PopFront()|*O(1)*|*O(1)*|
|PushBack(Key)|*O(n)*|*O(1)*|
|TopBack()|*O(n)*|*O(1)*|
|PopBack|*O(n)*|*O(n)*|
|Find(Key)|*O(n)*|*O(n)*|
|Erase(Key)|*O(n)*|*O(n)*|
|Empty()|*O(1)*|*O(1)*|
|AddBefore(Node, Key)|*O(n)*|*O(n)*|
|AddAfter(Node, Key)|*O(1)*|*O(1)*|
