use std::hash::Hash;

use bit_vec::BitVec;
use num_traits::ToPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<T>
where
    T: Clone + Eq + Hash,
{
    Top,
    Bot,
    Atom(T),
    Not(Box<Expr<T>>),
    Or(Box<Expr<T>>, Box<Expr<T>>),
    And(Box<Expr<T>>, Box<Expr<T>>),
    // Imp(Box<Expr<T>>, Box<Expr<T>>),
    // Iff(Box<Expr<T>>, Box<Expr<T>>),
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
            And(b1, b2) => b1.eval(valuation) & b2.eval(valuation),
            Or(b1, b2) => b1.eval(valuation) | b2.eval(valuation),
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
