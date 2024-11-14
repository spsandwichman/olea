#![warn(missing_docs)]

//! The Olea abstract syntax tree module.
//!
//! The types in this module represent the syntactic forms that comprise the Olea grammar. We use a convention for each category of item of an enum `FooKind` representing the element itself, and a `Foo` which contains a `FooKind` as well as the source span of the element.

use crate::compiler_types::Str;

/// A source span, a single subsection of a source file corresponding to an item.
pub type Span = core::ops::Range<usize>;

/// A string that carries its own span.
pub type Name = Spanned<Str>;

/// A wrapper type for associating an item with a source span. This type is aliased by an item type `Foo` wrapping around a `FooKind`.
#[derive(Clone, Debug)]
pub struct Spanned<T> {
    /// The inner value.
    pub kind: T,
    /// The span corresponding to the inner value.
    pub span: Span,
}

/// A full source program, made of a list of declarations.
#[derive(Clone, Debug)]
pub struct Program {
    /// The list of declarations.
    pub decls: Vec<Decl>,
}

/// See [DeclKind].
pub type Decl = Spanned<DeclKind>;

/// A declaration.
#[derive(Clone, Debug)]
pub enum DeclKind {
    /// See [Function].
    Function(Function),
}

/// A named body of code that can be called with a list of arguments and yield a return value.
#[derive(Clone, Debug)]
pub struct Function {
    /// The name of the function.
    pub name: Name,
    /// The list of parameters and their types that the function accepts.
    pub parameters: Vec<(Name, Ty)>,
    /// The type of value the function returns, if any.
    pub returns: Option<Ty>,
    /// The body of code that is executed when the function is called.
    pub body: Block,
}

/// See [TyKind].
pub type Ty = Spanned<TyKind>;

/// A data type.
#[derive(Clone, Debug)]
pub enum TyKind {
    /// An integer.
    Int,
    /// A pointer to a value of a given type.
    Pointer(Box<Ty>),
}

/// See [StmtKind].
pub type Stmt = Spanned<StmtKind>;

/// A statement, a piece of code executed within a block.
#[derive(Clone, Debug)]
pub enum StmtKind {
    /// A local variable declaration. This introduces a name which is bound to a value and can be accessed by all following statements in the block.
    Let(Name, Ty, Expr),
    /// An expression whose yielded value, if any, is ignored (unless it is the final statement of a block). See [ExprKind].
    Expr(Expr),
}

/// See [ExprKind].
pub type Expr = Spanned<ExprKind>;

/// An expression, a piece of code which may yield a value when executed.
#[derive(Clone, Debug)]
pub enum ExprKind {
    /// An integer constant.
    Int(u64),
    /// A calculation taking the values of two expressions to yield another.
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    /// An "if" block composed of a condition and two branch bodies. The boolean condition is evaluated. If true, the first branch is evaluated. Otherise, the second branch is evaluated.
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /// A "while" loop. The condition and body expressions are evaluated repeatedly until the condition yields false.
    While(Box<Expr>, Box<Expr>),
    /// Store a value in a memory location.
    Assign(Place, Box<Expr>),
    /// See [Block].
    Block(Block),
    /// A function call, composed of a function and a list of arguments to pass to it.
    Call(Box<Expr>, Vec<Expr>),
    /// See [PlaceKind].
    Place(PlaceKind),
}

/// See [PlaceKind].
pub type Place = Spanned<PlaceKind>;

/// A "place expression", a category of expressions which are associated with a pointer to memory in which its value is stored. This distinction is relevant in so-called "place expression contexts", such as the left side of an assignment.
#[derive(Clone, Debug)]
pub enum PlaceKind {
    /// A variable.
    Var(Name),
    /// An access of a value from an expression which yields a pointer. The second field contains the source span of the deref operator `^` itself.
    Deref(Box<Expr>, Span),
}

/// See [BinOpKind].
pub type BinOp = Spanned<BinOpKind>;

/// The kind of binary operation.
#[derive(Clone, Debug)]
pub enum BinOpKind {
    /// Two's complement addition.
    Add,
    /// Two's complement subtration.
    Sub,
    /// Two's complement multiplication.
    Mul,
}

/// A block, a list of statements that are executed in order. The block may yield the value of its last statement.
#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);
