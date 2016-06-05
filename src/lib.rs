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


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("apply_attr"), MultiModifier(Box::new(expand)));
}

bitflags! {
    flags ItemMask: u16 {
        const ITEM_NONE         = 0b0,
        const ITEM_EXT_CRATE    = 0b1 <<  0,
        const ITEM_USE          = 0b1 <<  1,
        const ITEM_STATIC       = 0b1 <<  2,
        const ITEM_CONST        = 0b1 <<  3,
        const ITEM_FN           = 0b1 <<  4,
        const ITEM_MOD          = 0b1 <<  5,
        const ITEM_FGN_MOD      = 0b1 <<  6,
        const ITEM_TY           = 0b1 <<  7,
        const ITEM_ENUM         = 0b1 <<  8,
        const ITEM_STRUCT       = 0b1 <<  9,
        const ITEM_TRAIT        = 0b1 << 10,
        const ITEM_DEF_IMPL     = 0b1 << 11,
        const ITEM_IMPL         = 0b1 << 12,
        const ITEM_MAC          = 0b1 << 13,
    }
}

type Selector = P<ast::MetaItem>;
type Selectors<'a> = &'a [Selector];

trait SelectorValidation {
    fn validate(&self, ctx: &mut ExtCtxt) -> bool;
}

impl SelectorValidation for Selector {
    fn validate(&self, ctx: &mut ExtCtxt) -> bool {
        match self.node {
            ast::MetaItemKind::List(ref name, ref sub_selectors) => {
                let valid_item = match &**name {
                    "mods" | "traits" | "impls" => true,
                    _ => {
                        let error_msg = format!("Unrecognized selector `{}(...)`.", name);
                        ctx.span_err(self.span, &error_msg);
                        false
                    }
                };
                let valid_sub_selectors = sub_selectors.iter()
                    .fold(true,
                          |valid, sub_selector| valid & sub_selector.validate(ctx));
                valid_item & valid_sub_selectors
            }
            ast::MetaItemKind::Word(ref name) => {
                match &**name {
                    "crates" | "uses" | "statics" | "consts" | "fns" | "mods" | "fgn_mods" |
                    "types" | "enums" | "structs" | "traits" | "def_impls" | "impls" | "macros" => {
                        true
                    }
                    _ => {
                        let error_msg = format!("Unrecognized selector `{}`.", name);
                        ctx.span_err(self.span, &error_msg);
                        false
                    }
                }
            }
            ast::MetaItemKind::NameValue(ref name, ref _value) => {
                let valid_item = false;
                let error_msg = format!("Unexpected name value pair `{} = ...`.", name);
                ctx.span_err(self.span, &error_msg);
                valid_item
            }
        }
    }
}

type Attribute = syntax::codemap::Spanned<syntax::ast::Attribute_>;
enum Attributes {
    Default(Vec<Attribute>),
    Override(Vec<Attribute>),
}

impl Attributes {
    fn augment(&self, existing: &[Attribute]) -> Vec<Attribute> {
        let mut expanded_attrs = vec![];
        match *self {
            Attributes::Default(ref attrs) => {
                expanded_attrs.extend(attrs.iter().cloned());
                expanded_attrs.extend(existing.iter().cloned());
            }
            Attributes::Override(ref attrs) => {
                expanded_attrs.extend(existing.iter().cloned());
                expanded_attrs.extend(attrs.iter().cloned());
            }
        }
        expanded_attrs
    }
}

fn expand(ctx: &mut ExtCtxt, span: Span, meta: &ast::MetaItem, ann: Annotatable) -> Annotatable {
    if let Some((selectors, attributes)) = extract_meta(ctx, meta) {
        fn not_applicable(ctx: &mut ExtCtxt, span: Span) {
            ctx.span_err(span, "Only applicable to `mod`, `trait` or `impl` items.");
        }
        match ann {
            Annotatable::Item(item_ptr) => {
                let ptr = item_ptr.map(|item| {
                    match item.node {
                        ast::ItemKind::Mod(..) |
                        ast::ItemKind::Trait(..) |
                        ast::ItemKind::Impl(..) => {}
                        _ => {
                            not_applicable(ctx, span);
                        }
                    }
                    expand_item(ctx, item, selectors, &attributes, true)
                });
                Annotatable::Item(ptr)
            }
            Annotatable::TraitItem(item_ptr) => {
                not_applicable(ctx, span);
                let ptr =
                    item_ptr.map(|item| expand_trait_item(ctx, item, selectors, &attributes, true));
                Annotatable::TraitItem(ptr)
            }
            Annotatable::ImplItem(item_ptr) => {
                not_applicable(ctx, span);
                let ptr =
                    item_ptr.map(|item| expand_impl_item(ctx, item, selectors, &attributes, true));
                Annotatable::ImplItem(ptr)
            }
        }
    } else {
        ann
    }
}

fn expand_item(ctx: &mut ExtCtxt,
               item: ast::Item,
               selectors: Selectors,
               attributes: &Attributes,
               is_root: bool)
               -> ast::Item {
    let item_mask = map_item_to_mask(&item);
    let selector_mask = extract_mask_from_selectors(selectors);
    let sub_selectors = if is_root {
        selectors.to_owned() // simply forward selectors for root item
    } else {
        extract_sub_selectors(selectors, item_mask)
    };
    let augmented_attributes = fold_attributes(item_mask, selector_mask, &item.attrs, attributes);
    let node = match item.node {
        ast::ItemKind::Mod(m) => {
            let expanded_items = m.items
                .into_iter()
                .map(|item_ptr| {
                    item_ptr.map(|item| expand_item(ctx, item, &sub_selectors, attributes, false))
                });
            ast::ItemKind::Mod(ast::Mod {
                inner: m.inner,
                items: expanded_items.collect(),
            })
        }
        ast::ItemKind::Trait(unsafety, generics, bounds, items) => {
            let expanded_items = items.into_iter()
                .map(|item| expand_trait_item(ctx, item, &sub_selectors, attributes, false));
            ast::ItemKind::Trait(unsafety, generics, bounds, expanded_items.collect())
        }
        ast::ItemKind::Impl(unsafety, polarity, generics, trt, typ, items) => {
            let expanded_items = items.into_iter()
                .map(|item| expand_impl_item(ctx, item, &sub_selectors, attributes, false));
            ast::ItemKind::Impl(unsafety,
                                polarity,
                                generics,
                                trt,
                                typ,
                                expanded_items.collect())
        }
        _ => item.node,
    };
    ast::Item {
        node: node,
        attrs: augmented_attributes,
        ..item
    }
}

fn expand_trait_item(_ctx: &mut ExtCtxt,
                     item: ast::TraitItem,
                     selectors: Selectors,
                     attributes: &Attributes,
                     _is_root: bool)
                     -> ast::TraitItem {
    let item_mask = map_trait_item_to_mask(&item);
    let selector_mask = extract_mask_from_selectors(selectors);
    let augmented_attributes = fold_attributes(item_mask, selector_mask, &item.attrs, attributes);
    ast::TraitItem { attrs: augmented_attributes, ..item }
}

fn expand_impl_item(_ctx: &mut ExtCtxt,
                    item: ast::ImplItem,
                    selectors: Selectors,
                    attributes: &Attributes,
                    _is_root: bool)
                    -> ast::ImplItem {
    let item_mask = map_impl_item_to_mask(&item);
    let selector_mask = extract_mask_from_selectors(selectors);
    let augmented_attributes = fold_attributes(item_mask, selector_mask, &item.attrs, attributes);
    ast::ImplItem { attrs: augmented_attributes, ..item }
}

fn extract_mask_from_selectors(selectors: Selectors) -> ItemMask {
    selectors.iter().fold(ITEM_NONE, |mask, selector| {
        mask |
        match (*selector).node {
            ast::MetaItemKind::Word(ref name) => map_selector_to_mask(&**name),
            _ => ITEM_NONE,
        }
    })
}

fn extract_sub_selectors(selectors: Selectors, mask: ItemMask) -> Vec<Selector> {
    for selector in selectors {
        if let ast::MetaItemKind::List(ref name, ref vec) = (*selector).node {
            if mask & map_selector_to_mask(&**name) != ITEM_NONE {
                return vec.clone();
            }
        }
    }
    vec![]
}

fn extract_meta<'a>(ctx: &mut ExtCtxt,
                    meta: &'a ast::MetaItem)
                    -> Option<(Selectors<'a>, Attributes)> {
    if let ast::MetaItemKind::List(_, ref vec) = meta.node {
        if vec.len() == 2 {
            let selectors = extract_selectors(ctx, &*vec[0]);
            let attributes = extract_attributes(ctx, &*vec[1]);
            if let (Some(selectors), Some(attributes)) = (selectors, attributes) {
                return Some((selectors, attributes));
            }
        }
    }
    ctx.span_err(meta.span,
                 "Expected 'apply_attr(to(...), as_default|as_override(...))'.");
    None
}

fn fold_attributes(item_mask: ItemMask,
                   selector_mask: ItemMask,
                   item_attributes: &[Attribute],
                   attributes: &Attributes)
                   -> Vec<Attribute> {
    if (selector_mask & item_mask) != ITEM_NONE {
        attributes.augment(item_attributes)
    } else {
        item_attributes.to_owned()
    }
}

fn extract_selectors<'a>(ctx: &mut ExtCtxt, meta: &'a ast::MetaItem) -> Option<Selectors<'a>> {
    if let ast::MetaItemKind::List(ref name, ref selectors) = meta.node {
        if name == "to" {
            let valid_selectors = selectors.iter()
                .fold(true, |valid, selector| valid & selector.validate(ctx));
            if !valid_selectors {
                return None;
            }
            return Some(selectors);
        }
    }
    ctx.span_err(meta.span, "Expected `to(...)`.");
    None
}

fn extract_attributes(ctx: &mut ExtCtxt, meta: &ast::MetaItem) -> Option<Attributes> {
    if let ast::MetaItemKind::List(ref name, ref vec) = meta.node {
        let attributes = vec.iter().map(|meta| ctx.attribute(meta.span, meta.clone()));
        if name == "as_default" {
            return Some(Attributes::Default(attributes.collect()));
        } else if name == "as_override" {
            return Some(Attributes::Override(attributes.collect()));
        }
    }
    ctx.span_err(meta.span, "Expected `as_default(...)` or `as_override(...)`.");
    None
}

fn map_selector_to_mask(selector: &str) -> ItemMask {
    match selector {
        "crates" => ITEM_EXT_CRATE,
        "uses" => ITEM_USE,
        "statics" => ITEM_STATIC,
        "consts" => ITEM_CONST,
        "fns" => ITEM_FN,
        "mods" => ITEM_MOD,
        "fgn_mods" => ITEM_FGN_MOD,
        "types" => ITEM_TY,
        "enums" => ITEM_ENUM,
        "structs" => ITEM_STRUCT,
        "traits" => ITEM_TRAIT,
        "def_impls" => ITEM_DEF_IMPL,
        "impls" => ITEM_IMPL,
        "macros" => ITEM_MAC,
        _ => ITEM_NONE,
    }
}

fn map_item_to_mask(item: &ast::Item) -> ItemMask {
    match item.node {
        ast::ItemKind::ExternCrate(..) => ITEM_EXT_CRATE,
        ast::ItemKind::Use(..) => ITEM_USE,
        ast::ItemKind::Static(..) => ITEM_STATIC,
        ast::ItemKind::Const(..) => ITEM_CONST,
        ast::ItemKind::Fn(..) => ITEM_FN,
        ast::ItemKind::Mod(..) => ITEM_MOD,
        ast::ItemKind::ForeignMod(..) => ITEM_FGN_MOD,
        ast::ItemKind::Ty(..) => ITEM_TY,
        ast::ItemKind::Enum(..) => ITEM_ENUM,
        ast::ItemKind::Struct(..) => ITEM_STRUCT,
        ast::ItemKind::Trait(..) => ITEM_TRAIT,
        ast::ItemKind::DefaultImpl(..) => ITEM_DEF_IMPL,
        ast::ItemKind::Impl(..) => ITEM_IMPL,
        ast::ItemKind::Mac(..) => ITEM_MAC,
    }
}

fn map_trait_item_to_mask(item: &ast::TraitItem) -> ItemMask {
    match item.node {
        ast::TraitItemKind::Const(..) => ITEM_CONST,
        ast::TraitItemKind::Method(..) => ITEM_FN,
        ast::TraitItemKind::Type(..) => ITEM_TY,
    }
}

fn map_impl_item_to_mask(item: &ast::ImplItem) -> ItemMask {
    match item.node {
        ast::ImplItemKind::Const(..) => ITEM_CONST,
        ast::ImplItemKind::Method(..) => ITEM_FN,
        ast::ImplItemKind::Type(..) => ITEM_TY,
        ast::ImplItemKind::Macro(..) => ITEM_MAC,
    }
}
