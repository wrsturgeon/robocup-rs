#![crate_type = "bin"]
#![deny(warnings)]
#![allow(incomplete_features)] // turn this off if anything is behaving inexplicably
#![feature( // https://github.com/rust-lang/rust/blob/master/compiler/rustc_feature/src/active.rs -> CtrlF 'declare_features! '
    auto_traits,
    box_patterns,
    cfg_sanitize,
    cfg_version,
    const_closures,
    const_convert,
    const_fn_floating_point_arithmetic,
    const_for,
    const_likely,
    const_mut_refs,
    const_precise_live_drops,
    const_trait_impl,
    const_try,
    custom_inner_attributes,
    custom_test_frameworks,
    decl_macro,
    default_type_parameter_fallback,
    exclusive_range_pattern,
    exhaustive_patterns,
    fn_align,
    generators,
    generic_arg_infer,
    // generic_associated_types_extended,
    // generic_assert,
    // generic_assert_internals,
    generic_const_exprs,
    half_open_range_patterns_in_slices,
    if_let_guard,
    inline_const,
    ip_in_core,
    lang_items,
    let_chains,
    min_specialization,
    negative_impls,
    never_type,
    never_type_fallback,
    optimize_attribute,
    precise_pointer_size_matching,
    repr_simd,
    simd_ffi,
    specialization,
    stmt_expr_attributes,
    strict_provenance,
    thread_local,
    trait_alias,
)]

#[macro_use]
extern crate debug_print;

mod comm;
mod spl;
mod state;

fn main() {
    debug_println!("We're team #{:#?}", spl::TEAM_NUMBER);

    let mut comm = crate::comm::udp::GCLiaison::init_blocking();
    loop {
        comm.get();
    }
    // println!("Robocup executable finished");
}
