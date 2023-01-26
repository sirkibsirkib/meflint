## Identifiers

I believe I really do need to be able to distinguish identifiers from types;
consider the case where I want to quantify over two facts of the same type.

## Combination at take time or product time

- We have to branch on the way down, because otherwise we cannot keep track
  of the correspondences between values in scope.

- continuation style:
  1. **Call** Callee builds a set of all values and returns them. Caller
	 decides what to do with the values in the set.
  2. **Callback**: caller delegates the destination of the values to the
     callee, who produces each value and uses the callback. 