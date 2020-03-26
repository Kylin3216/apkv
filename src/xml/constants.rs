use std::io::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub const TYPE_REFERENCE: i32 = 1;
pub const TYPE_ATTRIBUTE: i32 = 2;
pub const TYPE_STRING: i32 = 3;
pub const TYPE_FLOAT: i32 = 4;
pub const TYPE_DIMENSION: i32 = 5;
pub const TYPE_FRACTION: i32 = 6;
pub const TYPE_FIRST_INT: i32 = 16;
pub const TYPE_INT_HEX: i32 = 17;
pub const TYPE_INT_BOOLEAN: i32 = 18;
pub const TYPE_FIRST_COLOR_INT: i32 = 28;
pub const TYPE_LAST_COLOR_INT: i32 = 31;
pub const TYPE_LAST_INT: i32 = 31;
pub const COMPLEX_UNIT_MASK: i32 = 15;
pub const RADIX_MULTS: [f64; 4] = [0.00390625f64, 3.051758E-005f64, 1.192093E-007f64, 4.656613E-010f64];
pub const DIMENSION_UNITS: [&'static str; 8] = ["px", "dip", "sp", "pt", "in", "mm", "", ""];
pub const FRACTION_UNITS: [&'static str; 8] = ["%", "%p", "", "", "", "", "", ""];
pub const START_DOCUMENT: i32 = 0;
pub const END_DOCUMENT: i32 = 1;
pub const START_TAG: i32 = 2;
pub const END_TAG: i32 = 3;
pub const TEXT: i32 = 4;



