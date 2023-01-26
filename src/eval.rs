
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct TypeIdx(u16);


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct TypeBytes(u16);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct StackIdx(u16);

enum EvalNode {
	Take(TypeIdx, TypeBytes, StackIdx),
}

struct Stack {

}