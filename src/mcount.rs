#[path = "msplit.rs"]
mod msplit;

use msplit::*;

pub fn my_mcount(input: &str) -> usize {
    match input.len() {
        0 => 0,
        _ => chars(input).len(),
    }
}