// Adapted from https://github.com/Alexhuszagh/rust-lexical.

use crate::lexical::rounding::*;

// MASKS

#[test]
fn lower_n_mask_test() {
    assert_eq!(lower_n_mask(0u64), 0b0);
    assert_eq!(lower_n_mask(1u64), 0b1);
    assert_eq!(lower_n_mask(2u64), 0b11);
    assert_eq!(lower_n_mask(10u64), 0b1111111111);
    assert_eq!(lower_n_mask(32u64), 0b11111111111111111111111111111111);
}

#[test]
fn lower_n_halfway_test() {
    assert_eq!(lower_n_halfway(0u64), 0b0);
    assert_eq!(lower_n_halfway(1u64), 0b1);
    assert_eq!(lower_n_halfway(2u64), 0b10);
    assert_eq!(lower_n_halfway(10u64), 0b1000000000);
    assert_eq!(lower_n_halfway(32u64), 0b10000000000000000000000000000000);
}

#[test]
fn nth_bit_test() {
    assert_eq!(nth_bit(0u64), 0b1);
    assert_eq!(nth_bit(1u64), 0b10);
    assert_eq!(nth_bit(2u64), 0b100);
    assert_eq!(nth_bit(10u64), 0b10000000000);
    assert_eq!(nth_bit(31u64), 0b10000000000000000000000000000000);
}

#[test]
fn internal_n_mask_test() {
    assert_eq!(internal_n_mask(1u64, 0u64), 0b0);
    assert_eq!(internal_n_mask(1u64, 1u64), 0b1);
    assert_eq!(internal_n_mask(2u64, 1u64), 0b10);
    assert_eq!(internal_n_mask(4u64, 2u64), 0b1100);
    assert_eq!(internal_n_mask(10u64, 2u64), 0b1100000000);
    assert_eq!(internal_n_mask(10u64, 4u64), 0b1111000000);
    assert_eq!(
        internal_n_mask(32u64, 4u64),
        0b11110000000000000000000000000000
    );
}

// DIRECTED ROUNDING
