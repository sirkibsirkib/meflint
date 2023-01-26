## Implementation Strategy

Each instance is a Data, a buffer of bytes, maybe smallvec.
Each type has a fixed size, the size of its Data elements.
Each projection from one type to its field is a (byte) offset
During evaluation, the local variables are identified by stack indices, thus, the stack is a list of Data

