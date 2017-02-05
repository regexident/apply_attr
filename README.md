# apply_attr

[![Build Status](http://img.shields.io/travis/regexident/apply_attr.svg?style=flat-square)](https://travis-ci.org/regexident/apply_attr)
[![Downloads](https://img.shields.io/crates/d/apply_attr.svg?style=flat-square)](https://crates.io/crates/apply_attr/)
[![Version](https://img.shields.io/crates/v/apply_attr.svg?style=flat-square)](https://crates.io/crates/apply_attr/)
[![License](https://img.shields.io/crates/l/apply_attr.svg?style=flat-square)](https://crates.io/crates/apply_attr/)

## Synopsis

A syntax extension providing higher-order attributes to Rust.

## Motivation

Sometimes it would be desirable to be able to apply certain attributes to all items in a scope (`mod`, `trait` or `impl`). The **`apply_attr`** crate aims to provide a versatile API for this.

Possible use-cases would be:

- Make all structs in mod `xyz` use `#[derive(PartialEq)]`.
- Mark all methods in a certain `impl` with `#[inline(never)]` (for profiling, e.g.).
- …

## Getting Started

Add the most recent [version](https://crates.io/crates/apply_attr) of `apply_attr`
to your dependencies in your project's `Cargo.toml`.

Then add …

```rust
#![feature(custom_attribute)]

#![feature(plugin)]
#![plugin(apply_attr)]
```

… to your crate's root file (e.g. `lib.rs`, `main.rs`).

Once that's done you're ready to play!

# Example

```rust
#![feature(custom_attribute)]

#![feature(plugin)]
#![plugin(apply_attr)]

// Make all top-level structs as well as those
// within top-level mods implement `PartialEq`:
#![apply_attr(to(structs, mods(structs)), as_default(derive(PartialEq)))]

pub struct Foo;

mod Bar {
  pub struct Baz;
  // ...
}

// Disable inlining when `no_inline` feature is present:
#[cfg_attr(feature = "no_inline", apply_attr(to(fns), as_override(inline(never))))]
impl Blee {
  fn foo(&self) { ... }
  fn bar(&self) { ... }
  fn baz(&self) { ... }
  fn blee(&self) { ... }
}

fn main() {
  Foo == Foo;
  Bar::Baz == Bar::Baz;
}
```

## API Reference

The `apply_attr` syntax extension provides a single higher-order attribute,
conveniently named `apply_attr` expecting two arguments:

1. `to(...)` (with `...` being a list of zero or more selectors).
2. `as_default(...)` or `as_override(...)` (with `...` being a list of zero or more attributes).

Resulting either of:

```rust
#[apply_attr(to(...), as_default(...))]
#[apply_attr(to(...), as_override(...))]
```

The first argument (`to(...)`) accepts a nested list of item selectors.

## Selectors

Selectors behave similar to CSS selectors:

As such a CSS selector like `div > span, p` would translate to `to(div(span), p)`.

### Flat Selectors

The following selectors are supported:

```rust
consts
crates
def_impls
enums
fgn_mods
fns
impls
macros
mods
statics
structs
traits
types
uses
```

### Nested Selectors

With the following ones allowing for nesting:

```rust
mods(...)
impls(...)
traits(...)
```

Nested selectors denote direct ancestry equivalent to CSS's `outer > inner` path operator.

## Attributes

### Default

Attributes can either be applied as using `as_default(...)`, in which case …

```rust
#[apply_attr(to(fns), as_default(inline(never)))]
impl Foo {
  #[inline(always)]
  fn foo() { ... }
}
```

… will be turned into …

```rust
impl Foo {
  #[inline(always)]
  fn foo() { ... }
}
```

… upon completion.

### Overriding

Or using `as_override(...)`, in which case …

```rust
#[apply_attr(to(fns), as_override(inline(never)))]
impl Foo {
  #[inline(always)]
  fn foo() { ... }
}
```

… will be turned into …

```rust
impl Foo {
  #[inline(never)]
  fn foo() { ... }
}
```

… upon completion.

## Debugging

To see how the attributes were applied compile your crate using this (requires `nightly`):

```bash
cargo rustc -- -Z unstable-options --pretty=expanded
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/apply_attr/tags).

## Authors

* **Vincent Esche** – *Initial work* – [Regexident](https://github.com/Regexident)

See also the list of [contributors](https://github.com/regexident/apply_attr/contributors) who participated in this project.

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
