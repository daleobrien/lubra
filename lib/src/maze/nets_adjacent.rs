/* This file is generated. */
use ::phf::{Map, phf_map};

pub static ADJ_CHECKS: Map<u32, &'static [&'static [(usize, bool, u32)]]> = phf_map! {
  8u32 =>  &[&[(3usize, true, 9u32)], &[(3usize, true, 10u32)]], // 001000
  9u32 =>  &[&[(3usize, true, 8u32)], &[(3usize, true, 8u32)], &[(3usize, true, 10u32)], &[(3usize, true, 12u32)], &[(3usize, true, 24u32)], &[(3usize, true, 40u32)]], // 001001
  10u32 =>  &[&[(3usize, true, 8u32)], &[(3usize, true, 9u32)], &[(3usize, true, 12u32)], &[(3usize, true, 24u32)], &[(3usize, true, 40u32)]], // 001010
  16u32 =>  &[&[(4usize, true, 17u32)], &[(4usize, true, 20u32)]], // 010000
  17u32 =>  &[&[(4usize, true, 16u32)], &[(4usize, true, 18u32)], &[(4usize, true, 20u32)], &[(4usize, true, 48u32)]], // 010001
  20u32 =>  &[&[(4usize, true, 16u32)], &[(4usize, true, 17u32)], &[(4usize, true, 18u32)], &[(4usize, true, 48u32)], &[(4usize, true, 24u32)]], // 010100
  24u32 =>  &[&[(3usize, true, 9u32), (4usize, true, 20u32)], &[(3usize, true, 10u32), (4usize, true, 17u32)], &[(3usize, false, 9u32), (4usize, true, 20u32)], &[(4usize, false, 17u32), (3usize, true, 10u32)], &[(3usize, true, 9u32)]], // 011000
  32u32 =>  &[&[(5usize, true, 34u32)], &[(5usize, true, 36u32)]], // 100000
  34u32 =>  &[&[(5usize, true, 32u32)], &[(5usize, true, 33u32)], &[(5usize, true, 36u32)], &[(5usize, true, 40u32)], &[(5usize, true, 48u32)]], // 100010
  36u32 =>  &[&[(5usize, true, 32u32)], &[(5usize, true, 33u32)], &[(5usize, true, 34u32)], &[(5usize, true, 40u32)], &[(5usize, true, 48u32)]], // 100100
  40u32 =>  &[&[(3usize, true, 10u32), (5usize, true, 36u32)], &[(3usize, true, 9u32), (5usize, true, 34u32)], &[(3usize, true, 9u32)], &[(5usize, true, 36u32)]], // 101000
  48u32 =>  &[&[(4usize, true, 17u32), (5usize, true, 36u32)], &[(4usize, true, 20u32), (5usize, true, 34u32)], &[(4usize, true, 17u32)], &[(5usize, true, 34u32)]], // 110000
};

