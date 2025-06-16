use std::hash::Hash;

use bit_vec::BitVec;
use num_traits::ToPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Or,
    And,
    Imp,
    Iff,
}

impl BinOp {
    pub fn to_bool_op(&self) -> fn(bool, bool) -> bool {
        match self {
            BinOp::Or => |b1, b2| b1 || b2,
            BinOp::And => |b1, b2| b1 && b2,
            BinOp::Imp => |b1, b2| b1 <= b2,
            BinOp::Iff => |b1, b2| b1 == b2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op<T> {
    code: BinOp,
    left: Expr<T>,
    right: Expr<T>,
}

impl<T> Op<T> {
    pub fn eval_on<S, F>(&self, ev: F, op: fn(S, S) -> S) -> S
    where
        F: Fn(&Expr<T>) -> S,
    {
        op(ev(&self.left), ev(&self.right))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<T> {
    Top,
    Bot,
    Atom(T),
    Not(Box<Expr<T>>),
    BinOp(Box<Op<T>>),
}

use Expr::*;

impl<T> Expr<T>
where
    T: Clone + Eq + Hash + ToPrimitive,
{
    pub fn eval(&self, valuation: &BitVec<u32>) -> bool {
        match self {
            Top => true,
            Bot => false,
            Atom(atom) => match valuation.get(T::to_usize(atom).unwrap()) {
                Some(b) => b,
                None => panic!("variable not defined by valuation",),
            },
            Not(b) => !b.eval(valuation),
            BinOp(op) => op.eval_on(|e| e.eval(valuation), op.code.to_bool_op()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr;
    use bit_vec::BitVec;
    use strum::{EnumCount, IntoEnumIterator};
    use strum_macros::{EnumCount, EnumIter};

    #[derive(Debug, Primitive, Clone, PartialEq, Eq, Hash, EnumCount, EnumIter)]
    enum Foo {
        Red = 0,
        Blue = 1,
        Green = 2,
    }

    #[test]
    fn eval_1() {
        let foo_size = Foo::COUNT;
        let bv = BitVec::from_elem(foo_size, true);
        let ex = Expr::And(
            Box::new(Expr::Atom(Foo::Red)),
            Box::new(Expr::Atom(Foo::Blue)),
        );
        let not_ex = Expr::Not(Box::new(ex.clone()));
        assert_eq!(true, ex.eval(&bv));
        assert_eq!(false, not_ex.eval(&bv));
    }
}
