#![feature(plugin)]
#![plugin(apply_attr)]

// make compiletest accept dummy attribute `foo`:
#![feature(custom_attribute)]

#[apply_attr(to(), default())]
pub mod foo {}

#[apply_attr(to(), default())]
pub trait Bar {}

pub struct Baz;
#[apply_attr(to(), default())]
impl Baz {}

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
extern crate apply_attr;

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
use foo;

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
static FOO: usize = 42;

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
const BAR: usize = 42;

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
fn foo_bar() {}

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
type Foo = Bar;

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
enum FooBar {}

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
struct BazBlee {}

// fgn_mod
// def_impls

#[apply_attr(to(), default())] //~ ERROR Only applicable to `mod`, `trait` or `impl` items.
macro_rules! foo_bar_baz {
    () => ()
}

fn main() {}
