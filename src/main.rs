#[macro_use]
extern crate enum_primitive_derive;
extern crate strum_macros;
use bit_vec::BitVec;

use crate::propositional::syntax::Expr;

pub mod propositional;

fn main() {
    use strum::{EnumCount, IntoEnumIterator};
    use strum_macros::{EnumCount, EnumIter};
    #[derive(Debug, Primitive, Clone, PartialEq, Eq, Hash, EnumCount, EnumIter)]
    enum Foo {
        Red = 0,
        Blue = 1,
        Green = 2,
    }
    let foo_size = Foo::COUNT;
    let bv = BitVec::from_elem(foo_size, true);
    // let ex = Expr::And(
    //     Box::new(Expr::Atom(Foo::Red)),
    //     Box::new(Expr::Atom(Foo::Blue)),
    // );
    // let not_ex = Expr::Not(Box::new(ex.clone()));
    // println!("{}", ex.eval(&bv));
}
