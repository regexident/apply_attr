#![feature(plugin)]
#![plugin(apply_attr)]

#![apply_attr_enums(derive(PartialEq))]
#![apply_attr_structs(derive(PartialEq))]

struct Foo;

struct Bar;

enum Baz {
    Blee
}

fn main() {
    Foo == Foo;
    Bar == Bar;
    Baz::Blee == Baz::Blee;
}
