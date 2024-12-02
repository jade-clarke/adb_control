// implements default
use std::default::Default;

#[derive(Debug, Default)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}