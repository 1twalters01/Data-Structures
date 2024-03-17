# Arrays

## Introduction
Arrays: A contiguous area of memory consisting of **equal-size** elements indexed by contiguous integers. It can be on either the stack or the heap.

Some examples of array declarations in various languages:
* long arr[] = new long[5];
* long arr[5];
* arr = [None * 5];

Arrays have constant-time access to read or write any element in the array.

We can do arithmetic to calculate the address of a particular array element:
array_addr + elem_size * (i - first_index)

## Example
Given an array address of 1000 and an element size of 8, if we wanted to find the memory address of the 6th element we would get 1048.

Most languages support multi-dimensional arrays, but if not then you can create your own.

## Example
Consider a 6 by 3 array (x by y). Say we wanted to calculate the memory address of position (3, 4).

We would first skip rows 1 and 2.
(3 - 1) * 6

Then we must skip the elements before (3, 4)
(3 - 1) * 6 + (4 - 1)

We then take this, multiply it by the element size and then add it to our array address
array_addr + elem_size * ((3 - 1) * 6 + (4 - 1))


We made a supposition: that all of the elements of the first row were layed out, then the second, and lastly the third. This is called Row-major ordering/indexing. Instead, some compilers and languages do it by column instead. This is called Column-major ordering/indexing.

## Times for Common Operations
* To read or write any element is order 1.
* Adding or removing the last element to the end of an array is of order *O(1)*.
* Removing the start/middle element requires us to move all elements after it down by 1 to prevent a hole being in the array. This is an order *o(n)* operation.
* The same is true for adding to the middle/start of an array.

||Add|Remove|
|Beginning|*O(n)*|*O(n)*|
|End|*O(1)*|*O(1)*|
|Middle|*O(n)*|*O(n)*|

## Summary
* An array consists of a contiguous chunk of memory - this lets us do the arithmetic mentioned above.
* The elements must be of equal size.
* It must be indexed by contiguous integers.
* Constant-time access to any element.
* Constant time to add/remove at the end of an array.
* Linear time to add/remove elsewhere.
