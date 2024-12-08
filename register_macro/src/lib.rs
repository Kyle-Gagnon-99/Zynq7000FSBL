#![no_std]

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro]
pub fn define_register(input: proc_macro::TokenStream) {
    let input = parse_macro_input!(input as ItemStruct);
}
