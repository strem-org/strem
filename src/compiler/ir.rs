//! Abstract Syntax Tree representations.
//!

pub mod ast;

/// Range operator kinds.
#[derive(Debug)]
pub enum RangeKind {
    Exactly(usize),
    AtLeast(usize),
    Between(usize, usize),
}

/// The set of Regular Expression operations allowed in a query.
#[derive(Debug)]
pub enum RegexOperatorKind {
    KleeneStar,
    Concatenation,
    Alternation,
    Range(RangeKind),
}

/// The set of spatial operations allowed against a frame.
///
/// These operators must be used within the `[]` enclosures. In addition, the
/// syntax for these operators may be the same as the syntax for some
/// non-spatial expressions (e.g., alternation and disjunction). Therefore,
/// these enumerations provide semantic meaning for symbolically
/// equivalent operators.
#[derive(Debug)]
pub enum SpatialOperatorKind {
    FolOperator(FolOperatorKind),
    SolOperator(SolOperatorKind),
    S4uOperator(S4uOperatorKind),
    S4Operator(S4OperatorKind),
}

/// First-Order Logic operators.
///
/// For more information on FOL, please see:
/// [Stanford Encyclopedia of Philosophy: Classical Logic](https://plato.stanford.edu/entries/logic-classical/)
#[derive(Debug)]
pub enum FolOperatorKind {
    Negation,
    Conjunction,
    Disjunction,
}

/// Second-Order Logic operators.
///
/// For more information on SOL, please see:
/// [Stanford Encyclopedia of Philosophy: Second-order and Higher-order logic](https://plato.stanford.edu/entries/logic-higher-order/)
#[derive(Debug)]
pub enum SolOperatorKind {
    Exists,
}

/// S4u operators.
///
/// For more information on S4, please see:
/// [Combining Spatial and Temporal Logics: Expressiveness vs. Complexity](https://arxiv.org/abs/1)
#[derive(Debug)]
pub enum S4uOperatorKind {
    NonEmpty,
}

/// S4 operators.
///
/// For more information on S4, please see:
/// [Combining Spatial and Temporal Logics: Expressiveness vs. Complexity](https://arxiv.org/abs/1110.2726)
#[derive(Debug)]
pub enum S4OperatorKind {
    Intersection,
    Union,
    Complement,
}

/// Operations kinds supported.
#[derive(Debug)]
pub enum Operator {
    RegexOperator(RegexOperatorKind),
    SpatialOperator(SpatialOperatorKind),
}

/// Generic representation of an AST.
///
/// This AST is used as an Intermediate Representation (IR) of expressions that
/// support unary and binary operator expressions.
#[derive(Debug)]
pub enum Node<T> {
    Operand(T),
    UnaryExpr {
        op: Operator,
        child: Box<Self>,
    },
    BinaryExpr {
        op: Operator,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Node::Operand(value)
    }
}

impl<T> Node<T> {
    pub fn unary<C>(op: Operator, child: C) -> Self
    where
        C: Into<Node<T>>,
    {
        Node::UnaryExpr {
            op,
            child: Box::new(child.into()),
        }
    }

    pub fn binary<L, R>(op: Operator, left: L, right: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Node::BinaryExpr {
            op,
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }
}
