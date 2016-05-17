#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
extern crate syntax;
extern crate rustc;

extern crate rustc_plugin;

#[macro_use]
extern crate bitflags;

use rustc_plugin::Registry;

use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiModifier;
use syntax::ext::build::AstBuilder;
use syntax::parse::token::intern;
use syntax::ptr::P;

/// A syntax extension providing higher-order attributes to Rust.
///
/// The following higher-order attributes are available:
///
/// ```rust
/// #[apply_attr_crates(...)]    // available for: mods
/// #[apply_attr_uses(...)]      // available for: mods
/// #[apply_attr_statics(...)]   // available for: mods
/// #[apply_attr_consts(...)]    // available for: mods/impls/traits
/// #[apply_attr_fns(...)]       // available for: mods/impls/traits
/// #[apply_attr_mods(...)]      // available for: mods
/// #[apply_attr_fgn_mods(...)]  // available for: mods
/// #[apply_attr_types(...)]     // available for: mods/impls/traits
/// #[apply_attr_enums(...)]     // available for: mods
/// #[apply_attr_structs(...)]   // available for: mods
/// #[apply_attr_traits(...)]    // available for: mods
/// #[apply_attr_def_impls(...)] // available for: mods
/// #[apply_attr_impls(...)]     // available for: mods
/// #[apply_attr_macros(...)]    // available for: mods/impls
/// ```
///
/// # Example
///
/// ```rust
/// #![feature(plugin)]
/// #![plugin(apply_attr)]
///
/// #![apply_attr_structs(derive(PartialEq))]
///
/// pub struct Foo;
///
/// #[apply_attr_structs(derive(PartialEq))]
/// mod Bar {
///     pub struct Baz;
///     // ...
/// }
///
/// #![apply_attr_fns(inline)]
/// impl Blee {
///   fn foo(&self) { /* ... */ }
///   fn bar(&self) { /* ... */ }
///   fn baz(&self) { /* ... */ }
///   fn blee(&self) { /* ... */ }
/// }
///
/// fn main() {
///     Foo == Foo;
///     Bar::Baz == Bar::Baz;
/// }
/// ```

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let extensions = vec![
        ("apply_attr_crates", MultiModifier(Box::new(expand_crates))),
        ("apply_attr_uses", MultiModifier(Box::new(expand_uses))),
        ("apply_attr_statics", MultiModifier(Box::new(expand_statics))),
        ("apply_attr_consts", MultiModifier(Box::new(expand_consts))),
        ("apply_attr_fns", MultiModifier(Box::new(expand_fns))),
        ("apply_attr_mods", MultiModifier(Box::new(expand_mods))),
        ("apply_attr_fgn_mods", MultiModifier(Box::new(expand_fgn_mods))),
        ("apply_attr_types", MultiModifier(Box::new(expand_types))),
        ("apply_attr_enums", MultiModifier(Box::new(expand_enums))),
        ("apply_attr_structs", MultiModifier(Box::new(expand_structs))),
        ("apply_attr_traits", MultiModifier(Box::new(expand_traits))),
        ("apply_attr_def_impls", MultiModifier(Box::new(expand_def_impls))),
        ("apply_attr_impls", MultiModifier(Box::new(expand_impls))),
        ("apply_attr_macros", MultiModifier(Box::new(expand_macros))),
    ];
    for (attribute, extension) in extensions.into_iter() {
        reg.register_syntax_extension(intern(attribute), extension);
    }
}

macro_rules! expand_function {
    ($function:ident, $pattern:path, $src:expr, $trg:expr) => (
        fn $function(ctx: &mut ExtCtxt,
                     span: Span,
                     meta: &ast::MetaItem,
                     ann: Annotatable)
                     -> Annotatable {
            match ann {
                Annotatable::Item(item) => {
                    Annotatable::Item(expand(ctx, span, meta, item, $src, $trg))
                }
                Annotatable::TraitItem(item_ptr) => {
                    Annotatable::TraitItem(item_ptr) // noop
                }
                Annotatable::ImplItem(item_ptr) => {
                    Annotatable::ImplItem(item_ptr) // noop
                }
            }
        }
    )
}

expand_function!(expand_crates, ast::ItemKind::ExternCrate, SRC_MOD, TRG_EXT_CRATE);
expand_function!(expand_uses, ast::ItemKind::Use, SRC_MOD, TRG_USE);
expand_function!(expand_statics, ast::ItemKind::Static, SRC_MOD, TRG_STATIC);
expand_function!(expand_consts, ast::ItemKind::Const, SRC_MOD | SRC_IMPL | SRC_TRAIT, TRG_CONST);
expand_function!(expand_fns, ast::ItemKind::Fn, SRC_MOD | SRC_IMPL | SRC_TRAIT, TRG_FN);
expand_function!(expand_mods, ast::ItemKind::Mod, SRC_MOD, TRG_MOD);
expand_function!(expand_fgn_mods, ast::ItemKind::ForeignMod, SRC_MOD, TRG_FOREIGN_MOD);
expand_function!(expand_types, ast::ItemKind::Ty, SRC_MOD | SRC_IMPL | SRC_TRAIT, TRG_TY);
expand_function!(expand_enums, ast::ItemKind::Enum, SRC_MOD, TRG_ENUM);
expand_function!(expand_structs, ast::ItemKind::Struct, SRC_MOD, TRG_STRUCT);
expand_function!(expand_traits, ast::ItemKind::Trait, SRC_MOD, TRG_TRAIT);
expand_function!(expand_def_impls, ast::ItemKind::DefaultImpl, SRC_MOD, TRG_DEF_IMPL);
expand_function!(expand_impls, ast::ItemKind::Impl, SRC_MOD, TRG_IMPL);
expand_function!(expand_macros, ast::ItemKind::Mac, SRC_MOD | SRC_IMPL, TRG_MAC);

type Attr = syntax::codemap::Spanned<syntax::ast::Attribute_>;

bitflags! {
    flags SourceMask: u8 {
        const SRC_NONE  = 0b0,
        const SRC_MOD   = 0b1 << 0,
        const SRC_IMPL  = 0b1 << 1,
        const SRC_TRAIT = 0b1 << 2,
    }
}

bitflags! {
    flags TargetMask: u16 {
        const TRG_NONE         = 0b0,
        const TRG_EXT_CRATE    = 0b1 <<  0,
        const TRG_USE          = 0b1 <<  1,
        const TRG_STATIC       = 0b1 <<  2,
        const TRG_CONST        = 0b1 <<  3,
        const TRG_FN           = 0b1 <<  4,
        const TRG_MOD          = 0b1 <<  5,
        const TRG_FOREIGN_MOD  = 0b1 <<  6,
        const TRG_TY           = 0b1 <<  7,
        const TRG_ENUM         = 0b1 <<  8,
        const TRG_STRUCT       = 0b1 <<  9,
        const TRG_TRAIT        = 0b1 << 10,
        const TRG_DEF_IMPL     = 0b1 << 11,
        const TRG_IMPL         = 0b1 << 12,
        const TRG_MAC          = 0b1 << 13,
    }
}

fn expand(ctx: &mut ExtCtxt,
          span: Span,
          meta: &ast::MetaItem,
          item: P<ast::Item>,
          source: SourceMask,
          target: TargetMask)
          -> P<ast::Item> {
    let node_clone = item.node.clone();
    let node = match node_clone {
        ast::ItemKind::Mod(m) => {
            if (source & SRC_MOD) != SRC_NONE {
                let attrs = get_attributes(ctx, span, meta);
                let items = expand_mod_items(m.items, &attrs, target);
                ast::ItemKind::Mod(ast::Mod {
                    inner: m.inner,
                    items: items,
                })
            } else {
                ctx.span_err(span, "Attribute not applicable to mods.");
                ast::ItemKind::Mod(m)
            }
        }
        ast::ItemKind::Trait(unsafety, generics, bounds, items) => {
            if (source & SRC_TRAIT) != SRC_NONE {
                let attrs = get_attributes(ctx, span, meta);
                let items = expand_trait_items(items, &attrs, target);
                ast::ItemKind::Trait(unsafety, generics, bounds, items)
            } else {
                ctx.span_err(span, "Attribute not applicable to traits.");
                ast::ItemKind::Trait(unsafety, generics, bounds, items)
            }
        }
        ast::ItemKind::Impl(unsafety, polarity, generics, trt, typ, items) => {
            if (source & SRC_IMPL) != SRC_NONE {
                let attrs = get_attributes(ctx, span, meta);
                let items = expand_impl_items(items, &attrs, target);
                ast::ItemKind::Impl(unsafety, polarity, generics, trt, typ, items)
            } else {
                ctx.span_err(span, "Attribute not applicable to impls.");
                ast::ItemKind::Impl(unsafety, polarity, generics, trt, typ, items)
            }
        }
        _ => {
            ctx.span_err(span, "Attribute not applicable here.");
            node_clone
        }
    };
    P(ast::Item { node: node, ..((*item).clone()) })
}

fn get_attributes(ctx: &mut ExtCtxt, span: Span, meta: &ast::MetaItem) -> Vec<Attr> {
    if let ast::MetaItemKind::List(_, ref vec) = meta.node {
        vec.iter().map(|meta_item| ctx.attribute(span, meta_item.clone())).collect()
    } else {
        ctx.span_err(span, "Expects a list of applicable attributes.");
        vec![]
    }
}

fn expand_mod_items(items: Vec<P<ast::Item>>,
                    attrs: &[Attr],
                    target: TargetMask)
                    -> Vec<P<ast::Item>> {
    items.iter()
        .map(|item| {
            if item_matches_target(item, target) {
                item.clone().map(|item| {
                    let mut expanded_attrs = attrs.to_owned();
                    expanded_attrs.extend(item.attrs);
                    ast::Item { attrs: expanded_attrs, ..item }
                })
            } else {
                item.clone()
            }
        })
        .collect()
}

fn expand_impl_items(items: Vec<ast::ImplItem>,
                     attrs: &[Attr],
                     target: TargetMask)
                     -> Vec<ast::ImplItem> {
    items.iter()
        .map(|item| {
            if impl_item_matches_target(item, target) {
                let mut expanded_attrs = attrs.to_owned();
                expanded_attrs.extend(item.attrs.clone());
                ast::ImplItem { attrs: expanded_attrs, ..item.clone() }
            } else {
                item.clone()
            }
        })
        .collect()
}

fn expand_trait_items(items: Vec<ast::TraitItem>,
                      attrs: &[Attr],
                      target: TargetMask)
                      -> Vec<ast::TraitItem> {
    items.iter()
        .map(|item| {
            if trait_item_matches_target(item, target) {
                let mut expanded_attrs = attrs.to_owned();
                expanded_attrs.extend(item.attrs.clone());
                ast::TraitItem { attrs: expanded_attrs, ..item.clone() }
            } else {
                item.clone()
            }
        })
        .collect()
}

fn item_matches_target(item: &ast::Item, target: TargetMask) -> bool {
    match item.node {
        ast::ItemKind::ExternCrate(..) => (target & TRG_EXT_CRATE) != TRG_NONE,
        ast::ItemKind::Use(..) => (target & TRG_USE) != TRG_NONE,
        ast::ItemKind::Static(..) => (target & TRG_STATIC) != TRG_NONE,
        ast::ItemKind::Const(..) => (target & TRG_CONST) != TRG_NONE,
        ast::ItemKind::Fn(..) => (target & TRG_FN) != TRG_NONE,
        ast::ItemKind::Mod(..) => (target & TRG_MOD) != TRG_NONE,
        ast::ItemKind::ForeignMod(..) => (target & TRG_FOREIGN_MOD) != TRG_NONE,
        ast::ItemKind::Ty(..) => (target & TRG_TY) != TRG_NONE,
        ast::ItemKind::Enum(..) => (target & TRG_ENUM) != TRG_NONE,
        ast::ItemKind::Struct(..) => (target & TRG_STRUCT) != TRG_NONE,
        ast::ItemKind::Trait(..) => (target & TRG_TRAIT) != TRG_NONE,
        ast::ItemKind::DefaultImpl(..) => (target & TRG_DEF_IMPL) != TRG_NONE,
        ast::ItemKind::Impl(..) => (target & TRG_IMPL) != TRG_NONE,
        ast::ItemKind::Mac(..) => (target & TRG_MAC) != TRG_NONE,
    }
}

fn trait_item_matches_target(item: &ast::TraitItem, target: TargetMask) -> bool {
    match item.node {
        ast::TraitItemKind::Const(..) => (target & TRG_CONST) != TRG_NONE,
        ast::TraitItemKind::Method(..) => (target & TRG_FN) != TRG_NONE,
        ast::TraitItemKind::Type(..) => (target & TRG_TY) != TRG_NONE,
    }
}

fn impl_item_matches_target(item: &ast::ImplItem, target: TargetMask) -> bool {
    match item.node {
        ast::ImplItemKind::Const(..) => (target & TRG_CONST) != TRG_NONE,
        ast::ImplItemKind::Method(..) => (target & TRG_FN) != TRG_NONE,
        ast::ImplItemKind::Type(..) => (target & TRG_TY) != TRG_NONE,
        ast::ImplItemKind::Macro(..) => (target & TRG_MAC) != TRG_NONE,
    }
}
