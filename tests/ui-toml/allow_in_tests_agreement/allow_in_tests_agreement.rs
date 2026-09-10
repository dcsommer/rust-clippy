//@compile-flags: --test
//@no-rustfix
//! Late passes resolve test-ness by walking the HIR parent chain; early passes match against
//! spans collected before the HIR exists. The two are separate implementations that must agree.
//!
//! `needless_raw_strings` is emitted from an early pass and `dbg_macro` from a late one. Pairing
//! them at every position `is_in_test` distinguishes means any disagreement between the two
//! shows up as an extra diagnostic below.
#![warn(clippy::dbg_macro, clippy::needless_raw_strings)]

fn main() {}

// Not test code: both fire.
fn plain() {
    let _ = r"no escapes";
    //~^ needless_raw_strings
    let _ = dbg!(0);
    //~^ dbg_macro
}

#[test]
fn in_test_fn() {
    // `is_in_test_function`
    let _ = r"no escapes";
    let _ = dbg!(0);
}

#[cfg(test)]
mod in_cfg_test {
    // Not a `#[test]` function, so this is `is_in_cfg_test` rather than `is_in_test_function`.
    fn helper() {
        let _ = r"no escapes";
        let _ = dbg!(0);
    }

    #[test]
    fn uses_helper() {
        helper();
    }
}
