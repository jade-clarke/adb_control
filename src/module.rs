// implements default
use std::default::Default;

#[derive(Debug, Default)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}