#![feature(plugin)]
#![plugin(apply_attr)]

#![apply_attr_enums(derive(PartialEq))]
#![apply_attr_structs(derive(PartialEq))]

pub struct Foo;

#[apply_attr_enums(derive(PartialEq))]
mod Bar {
    pub struct Baz;
    pub enum Blee {
        FooBar
    }
}

fn main() {
    Foo == Foo;

    Bar::Baz == Bar::Baz;
    //~^ ERROR binary operation `==` cannot be applied to type `Bar::Baz`

    Bar::Blee::FooBar == Bar::Blee::FooBar;
}
