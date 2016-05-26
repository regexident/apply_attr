#![feature(plugin)]
#![plugin(apply_attr)]

#![apply_attr(to(enums, structs, mods(structs)), default(derive(PartialEq)))]

struct Foo;

enum Bar {
    Baz
}

mod blee {
    pub struct Blee;
}

fn main() {
    Foo == Foo;
    Bar::Baz == Bar::Baz;
    blee::Blee == blee::Blee;
}
