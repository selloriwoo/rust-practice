mod struct_f;
mod method_f;
mod enum_f;
mod match_f;
mod iflet_f;
mod vector_f;
mod string_f;

pub mod garden;

use std::vec;

use crate::garden::vegetables::Asparagus;
use crate::struct_f::stfile;
use crate::method_f::method_file;
use crate::enum_f::enum_file;
use crate::match_f::match_file;
use crate::iflet_f::iflet_file;
use crate::vector_f::vector_file;
use crate::string_f::string_file;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // stfile();
    // method_file();
    // enum_file();
    // match_file();
    // iflet_file();
    vector_file();
}