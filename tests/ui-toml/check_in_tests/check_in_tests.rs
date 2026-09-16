//@compile-flags: --test
//@no-rustfix
//! Lints listed in `check-in-tests` report in test code despite skipping it by default.
#![warn(
    clippy::arbitrary_source_item_ordering,
    clippy::eq_op,
    clippy::missing_assert_message,
    clippy::missing_const_for_fn
)]

#[allow(clippy::missing_const_for_fn)]
fn main() {}

#[test]
#[allow(clippy::missing_const_for_fn)]
fn in_test_fn() {
    // `eq_op` normally skips `#[test]` functions entirely.
    let _ = 1 + 1 == 1 + 1;
    //~^ eq_op

    // `missing_assert_message` also skips test code, but isn't listed, so it stays quiet.
    let x: u32 = "1".parse().unwrap();
    assert!(x > 0);
}

// `arbitrary_source_item_ordering` normally skips `#[cfg(test)]` items, so this module's
// placement after a function is not reported by default.
#[cfg(test)]
mod tests {
    //~^ arbitrary_source_item_ordering
    // `missing_const_for_fn` normally skips test code.
    fn helper(x: u32) -> u32 {
        //~^ missing_const_for_fn
        x
    }

    #[test]
    fn uses_helper() {
        assert_eq!(helper(1), 1);
    }
}
