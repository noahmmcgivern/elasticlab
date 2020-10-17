use crate::Inf;
use crate::Opti;

pub fn opt(infrastructure: Inf, number: u8, option: Opti, value: String) {
    println!("{} : {} : {} : {}", infrastructure, number, option, value)
}
