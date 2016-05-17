#![feature(plugin)]
#![plugin(apply_attr)]

// make compiletest accept dummy attribute `foo`:
#![feature(custom_attribute)]

#[apply_attr_crates(foo)]
#[apply_attr_uses(foo)]
#[apply_attr_statics(foo)]
#[apply_attr_consts(foo)]
#[apply_attr_fns(foo)]
#[apply_attr_mods(foo)]
#[apply_attr_fgn_mods(foo)]
#[apply_attr_types(foo)]
#[apply_attr_enums(foo)]
#[apply_attr_structs(foo)]
#[apply_attr_traits(foo)]
#[apply_attr_def_impls(foo)]
#[apply_attr_impls(foo)]
#[apply_attr_macros(foo)]
mod foo {

}

#[apply_attr_crates(foo)]    //~ ERROR Attribute not applicable to traits.
#[apply_attr_uses(foo)]      //~ ERROR Attribute not applicable to traits.
#[apply_attr_statics(foo)]   //~ ERROR Attribute not applicable to traits.
#[apply_attr_consts(foo)]
#[apply_attr_fns(foo)]
#[apply_attr_mods(foo)]      //~ ERROR Attribute not applicable to traits.
#[apply_attr_fgn_mods(foo)]  //~ ERROR Attribute not applicable to traits.
#[apply_attr_types(foo)]
#[apply_attr_enums(foo)]     //~ ERROR Attribute not applicable to traits.
#[apply_attr_structs(foo)]   //~ ERROR Attribute not applicable to traits.
#[apply_attr_traits(foo)]    //~ ERROR Attribute not applicable to traits.
#[apply_attr_def_impls(foo)] //~ ERROR Attribute not applicable to traits.
#[apply_attr_impls(foo)]     //~ ERROR Attribute not applicable to traits.
#[apply_attr_macros(foo)]    //~ ERROR Attribute not applicable to traits.
trait Bar {

}

struct Baz;

#[apply_attr_crates(foo)]    //~ ERROR Attribute not applicable to impls.
#[apply_attr_uses(foo)]      //~ ERROR Attribute not applicable to impls.
#[apply_attr_statics(foo)]   //~ ERROR Attribute not applicable to impls.
#[apply_attr_consts(foo)]
#[apply_attr_fns(foo)]
#[apply_attr_mods(foo)]      //~ ERROR Attribute not applicable to impls.
#[apply_attr_fgn_mods(foo)]  //~ ERROR Attribute not applicable to impls.
#[apply_attr_types(foo)]
#[apply_attr_enums(foo)]     //~ ERROR Attribute not applicable to impls.
#[apply_attr_structs(foo)]   //~ ERROR Attribute not applicable to impls.
#[apply_attr_traits(foo)]    //~ ERROR Attribute not applicable to impls.
#[apply_attr_def_impls(foo)] //~ ERROR Attribute not applicable to impls.
#[apply_attr_impls(foo)]     //~ ERROR Attribute not applicable to impls.
#[apply_attr_macros(foo)]
impl Baz {
    
}

fn main() {

}
