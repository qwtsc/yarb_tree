## Red black tree
This is a red black tree demo in Rust only for learning smart pointer. Deleting element in the rbtree is not implemented yet.

## benchmark result
I compared this toy rbtree with the official BtreeSet in terms of insertation efficiency. No surprise, the BtreeSet is way faster than mine.
The detailed result can be seen as follows:
+ insert 0.1 million elements in the self-made rbtree
  time:   [79.794 ms 80.333 ms 80.906 ms]
+ insert 0.1 million elements in the official btreeset
  time:   [17.092 ms 17.291 ms 17.484 ms]

