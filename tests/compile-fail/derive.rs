#![feature(custom_attribute)]

#![feature(plugin)]
#![plugin(apply_attr)]

#![apply_attr(to(structs), as_default(derive(PartialEq)))]

pub enum Foo {
    Bar
}

pub struct Bar;

#[apply_attr(to(enums), as_override(derive(PartialEq)))]
mod foo {
    pub struct Baz;
    pub enum Blee {
        FooBar
    }
}

fn main() {
    Foo::Bar == Foo::Bar;
    //~^ ERROR binary operation `==` cannot be applied to type `Foo`

    Bar == Bar;

    foo::Baz == foo::Baz;
    //~^ ERROR binary operation `==` cannot be applied to type `foo::Baz`

    foo::Blee::FooBar == foo::Blee::FooBar;
}
