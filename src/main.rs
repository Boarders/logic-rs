#[macro_use]
extern crate enum_primitive_derive;
extern crate strum_macros;


pub mod propositional;

fn main() {
    propositional::syntax::yep();
}

