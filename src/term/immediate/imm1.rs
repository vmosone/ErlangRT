//!
//! Immediate1 values used to represent longer special term types.
//! Bit composition is - `.... .... .... aaPP`, where `PP` is primary tag, and
//! `aa` is imm1 tag
//!
//! Max value for imm1 is 64-4=60, or 32-4=28 bits.
//!
use term::primary;
use rt_defs;
use rt_defs::{Word, SWord};
//use term::immediate::primary::PRIM_TAG_LAST;

use bit_field::BitField;

/// Bit position for imm1 tag
pub const IMM1_TAG_FIRST: u8 = 2;
pub const IMM1_TAG_LAST: u8 = 4;

/// Bit position for the value after imm1 tag
pub const IMM1_VALUE_FIRST: u8 = IMM1_TAG_LAST;
pub const IMM1_VALUE_LAST: u8 = rt_defs::WORD_BITS as u8;

#[allow(dead_code)]
pub const SMALL_BITS: Word = (IMM1_VALUE_LAST - IMM1_VALUE_FIRST) as Word;

pub const TAG_IMM1_PID: Word = 0;
pub const TAG_IMM1_PORT: Word = 1;
pub const TAG_IMM1_IMM2: Word = 2;
pub const TAG_IMM1_SMALL: Word = 3;

/// Max value for the Immediate1 enum (for assertions).
pub const IMMEDIATE1_MAX: Word = 3;

/// Special tag {primary=Immediate} precomposed
pub const IMM1_PREFIX: Word = primary::TAG_IMMED;

/// Precomposed bits for pid imm1
pub const IMM1_PID_PREFIX: Word = IMM1_PREFIX
    | (TAG_IMM1_PID << IMM1_TAG_FIRST);

pub const IMM1_SMALL_PREFIX: Word = IMM1_PREFIX
    | (TAG_IMM1_SMALL << IMM1_TAG_FIRST);

#[inline]
pub fn is_immediate1(val: Word) -> bool {
  get_imm1_prefix(val) == IMM1_PREFIX
}

/// Get prefix bits BEFORE imm1 tag
#[inline]
pub fn get_imm1_prefix(val: Word) -> Word {
  val.get_bits(0..IMM1_TAG_FIRST)
}


/// Get prefix bits BEFORE imm1 tag plus the tag itself (for raw comparisons)
#[inline]
pub fn get_imm1_prefix_and_tag(val: Word) -> Word {
  val.get_bits(0..IMM1_VALUE_FIRST)
}


/// Trim the immediate1 bits and return them as an convenient enum.
#[inline]
pub fn get_imm1_tag(val: Word) -> Word {
  let t: Word = val.get_bits(IMM1_TAG_FIRST..IMM1_TAG_LAST);
  assert!(t <= IMMEDIATE1_MAX);
  t
}


/// Remove tag bits from imm1 value by shifting it right
#[inline]
pub fn get_imm1_value(val: Word) -> Word {
  assert!(is_immediate1(val), "raw lterm {:x} is not immediate1", val);
  val.get_bits(IMM1_VALUE_FIRST..IMM1_VALUE_LAST)
}


/// Remove tag bits from imm1 value by shifting it right. If the stored value
/// was negative (had its highest bit set) we preserve the sign.
#[inline]
pub fn get_imm1_value_s(val: Word) -> SWord {
  assert!(is_immediate1(val), "raw lterm {:x} is not immediate1", val);
  //val.get_bits(IMM1_VALUE_FIRST..IMM1_VALUE_LAST)
  (val as SWord) >> IMM1_VALUE_FIRST
}


/// Given a value raw preset bits, compose them together and form an imm1 `LTerm`
#[inline]
pub fn combine_imm1_prefix_and_val(val: Word, prefix0: Word) -> Word {
  let prefix = prefix0;
  assert!(prefix < (1 << IMM1_VALUE_FIRST));
  assert!(val < (1 << (IMM1_VALUE_LAST - IMM1_VALUE_FIRST)));
  //*prefix.set_bits(IMM1_VALUE_FIRST..IMM1_VALUE_LAST, val)
  prefix | (val << IMM1_VALUE_FIRST)
}


#[inline]
pub fn combine_imm1_prefix_and_val_signed(val: SWord, prefix0: Word) -> Word {
  let prefix = prefix0;
  assert!(prefix < (1 << IMM1_VALUE_FIRST));
  assert!(val >= rt_defs::MIN_NEG_SMALL,
          "val 0x{:x} must be >= MIN_SIG_SMALL 0x{:x}",
          val, rt_defs::MIN_NEG_SMALL);
  assert!(val <= rt_defs::MAX_POS_SMALL,
          "val 0x{:X} must be <= MAX_SIG_SMALL 0x{:x}",
          val, rt_defs::MAX_POS_SMALL);

  //*prefix.set_bits(IMM1_VALUE_FIRST..IMM1_VALUE_LAST, val as Word)
  prefix | (val << IMM1_VALUE_FIRST) as Word
}
