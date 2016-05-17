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

Add the following to your dependencies in your project's `Cargo.toml`:

```toml
apply_attr = "0.1.0"
```

… or whatever [other version](./releases) is more up-to-date.

Then add …

```rust
#![feature(plugin)]
#![plugin(apply_attr)]
```
… to your crate's root file (e.g. `lib.rs`, `main.rs`).

This gives you access to the following attributes:

```rust
#[apply_attr_crates(...)]    // available for: mods
#[apply_attr_uses(...)]      // available for: mods
#[apply_attr_statics(...)]   // available for: mods
#[apply_attr_consts(...)]    // available for: mods/impls/traits
#[apply_attr_fns(...)]       // available for: mods/impls/traits
#[apply_attr_mods(...)]      // available for: mods
#[apply_attr_fgn_mods(...)]  // available for: mods
#[apply_attr_types(...)]     // available for: mods/impls/traits
#[apply_attr_enums(...)]     // available for: mods
#[apply_attr_structs(...)]   // available for: mods
#[apply_attr_traits(...)]    // available for: mods
#[apply_attr_def_impls(...)] // available for: mods
#[apply_attr_impls(...)]     // available for: mods
#[apply_attr_macros(...)]    // available for: mods/impls
```

## Example
```rust
#![feature(plugin)]
#![plugin(apply_attr)]

#![apply_attr_structs(derive(PartialEq))]

pub struct Foo;

#[apply_attr_structs(derive(PartialEq))]
mod Bar {
    pub struct Baz;
    // ...
}

#![apply_attr_fns(inline)]
impl Blee {
    fn foo(&self) { /* ... */ }
    fn bar(&self) { /* ... */ }
    fn baz(&self) { /* ... */ }
    fn blee(&self) { /* ... */ }
}

fn main() {
    Foo == Foo;
    Bar::Baz == Bar::Baz;
}
```

## API Reference

[Documentation](https://regexident.github.io/apply_attr)

## Debugging

To see how the attributes were applied compile your crate using this (requires `nightly`):

```bash
cargo rustc -- -Z unstable-options --pretty=expanded
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html), and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/apply_attr/tags).

## Authors

* **Vincent Esche** - *Initial work* - [Regexident](https://github.com/Regexident)

See also the list of [contributors](https://github.com/regexident/apply_attr/contributors) who participated in this project.

## License

This project is licensed under the **BSD License** - see the [LICENSE.md](LICENSE.md) file for details.
