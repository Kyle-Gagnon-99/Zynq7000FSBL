#![no_std]
#[doc = include_str!("../README.md")]
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, LitBool, Token};

use registers;

/// An enum used to represent if a bit field is a single bit or a range of bits
#[derive(Debug, Clone, Copy)]
enum BitRepresentation {
    Single(usize),
    Range(usize, usize),
}

#[derive(Debug, Clone, Copy)]
enum TypeClass {
    /// A boolean type
    Bool,
    /// An unsigned integer type
    UInt,
    /// A signed integer type
    SInt,
    /// Anything else
    Other,
}

/// The access type of a bit field
enum Access {
    Read,
    Write,
    ReadWrite,
    WriteToClear,
    None,
}

/// A structure that represents
///
/// A field will have the following annotations over the field name:
/// #[bitfield(0:1, access = RW, default = 0)] or #[bitfield(0)]
/// field_name: (bool, u8, u16, enum)
///
/// The first number is the bit range. So a single number means a single bit and two numbers in the format of "a:b" means a range of bits.
/// "a" should be less than "b" and both should be less than the size of the register. This will translate to a range of bits from "a" to "b" inclusive.
///
/// The access specifier is optional and can be one of the following:
/// - RW: Read/Write
/// - R: Read
/// - W: Write
/// - WTC: Write to clear
///
/// If no access specifier is provided, it defaults to RW.
///
/// The default specifier is optional as well and will set the default value of the field. If no default value is provided, it will use the type's default value.
/// This means if the field is a bool, it will default to false. If it is an integer, it will default to 0. If it is an enum, it needs to implement the Default trait.
///
/// After the annotation will be the declaration of the field. The declaration should be a type (bool, u8, u16, u32, etc.) or even an Enum
/// If it is an Enum, then the Enum should have a #[repr(uSize)] attribute where uSize is the size of the register. Since this library
/// relies on the registers crate, the Enum should be implemented with the FromBits and IntoBits traits.
struct BitField {
    bits: BitRepresentation,
    access: Access,
    ty: syn::Type,
    field_name: syn::Ident,
    field_type: syn::Type,
}
impl BitField {
    fn new_from_field(field: &syn::Field) -> Self {
        let bit_field = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("bitfield"));

        match bit_field {
            Some(bit_field) => {
                let BitFieldParams {
                    bits,
                    access,
                    default,
                } = syn::parse2(bit_field.into_token_stream()).unwrap();

                BitField {
                    bits,
                    access,
                    ty: field.ty.clone(),
                    field_name: field.ident.clone().unwrap(),
                    field_type: field.ty.clone(),
                }
            }
            None => {
                let bits = BitRepresentation::Single(0);
                let access = Access::ReadWrite;
                let ty = field.ty.clone();
                let field_name = field.ident.clone().unwrap();
                let field_type = field.ty.clone();

                BitField {
                    bits,
                    access,
                    ty,
                    field_name,
                    field_type,
                }
            }
        }
    }
}

struct BitFieldParams {
    bits: BitRepresentation,
    access: Access,
    default: syn::Expr,
}

/// The parameters of the register attribute
/// These get parsed from the attribute arguments
struct RegisterParams {
    size: syn::LitInt,
    ty: syn::Type,
}

fn s_err(span: proc_macro2::Span, msg: impl core::fmt::Display) -> syn::Error {
    syn::Error::new(span, msg)
}

#[proc_macro_attribute]
pub fn register(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    match register_inner(args.into(), input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn register_inner(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    // Parse the intput as a struct
    let input = match syn::parse2::<syn::ItemStruct>(input) {
        Ok(input) => input,
        Err(e) => return Err(e),
    };

    // Parse the attributes
    let RegisterParams { size, ty } = syn::parse2(args)?;

    // For testing purposes, get the name of the struct
    let name = &input.ident;

    // Check for only named fields
    let syn::Fields::Named(field) = &input.fields else {
        return Err(s_err(
            input.fields.span(),
            "Only named fields are supported",
        ));
    };

    // Get the fields
    let fields = field
        .named
        .iter()
        .map(BitField::new_from_field)
        //.map(|f| f.unwrap())
        .map(|f| f.into_token_stream())
        .collect::<TokenStream>();

    Ok(quote! {
        #input

        impl #name {
            #fields
        }
    })
}

impl Parse for RegisterParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let size = input.parse::<syn::LitInt>()?;
        input.parse::<syn::Token![,]>()?;
        let ty = input.parse::<syn::Type>()?;

        // Validate that the size of the register is at least the same size or smaller than the type
        // of the register
        // (A.k.a 32 bits should not be a u8 type but a u32 type)
        let (_, bits) = to_types(&ty);

        if size.base10_parse::<usize>().unwrap() > bits {
            return Err(s_err(
                size.span(),
                "Size of register is larger than the type",
            ));
        }

        Ok(RegisterParams { size, ty })
    }
}

impl Parse for Access {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;

        if ident == "RW" {
            Ok(Access::ReadWrite)
        } else if ident == "R" {
            Ok(Access::Read)
        } else if ident == "W" {
            Ok(Access::Write)
        } else if ident == "WTC" {
            Ok(Access::WriteToClear)
        } else {
            Err(s_err(ident.span(), "Invalid access specifier"))
        }
    }
}

/// Convert a syn::Type to a TypeClass and usize (number of bits)
///
/// # Arguments
/// ty: &syn::Type - The type to convert
///
/// # Returns
/// (TypeClass, usize) - The type class and number of bits
fn to_types(ty: &syn::Type) -> (TypeClass, usize) {
    let syn::Type::Path(syn::TypePath { path, .. }) = ty else {
        return (TypeClass::Other, 0);
    };

    let Some(ident) = path.get_ident() else {
        return (TypeClass::Other, 0);
    };

    // Check if the type is a boolean
    if ident == "bool" {
        return (TypeClass::Bool, 1);
    }

    // Check if the type is isize or usize
    if ident == "isize" || ident == "usize" {
        return (TypeClass::UInt, 0); // they have architecture dependend sizes
    }

    // Signed integers are not supported
    if ident == "i8" || ident == "i16" || ident == "i32" || ident == "i64" || ident == "i128" {
        return (TypeClass::SInt, 0);
    }

    // Check if the type is an unsigned integer
    macro_rules! integer {
        ($ident:ident => $($uint:ident),*) => {
            match ident {
                $(_ if ident == stringify!($uint) => (TypeClass::UInt, $uint::BITS as _),)*
                _ => (TypeClass::Other, 0)
            }
        };
    }
    integer!(ident => u8, u16, u32, u64, u128)
}

impl Parse for BitFieldParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // The first argument should be the bit range
        // It can be a single bit or a range of bits
        // A single bit will be a single number
        // A range of bits will be two numbers separated by a colon (a:b)
        let lower_bound = input.parse::<syn::LitInt>()?;

        // Check if the next token is a colon
        let has_colon = input.peek(syn::Token![:]);

        // If there is a colon, then parse the upper bound
        let upper_bound = if has_colon {
            input.parse::<syn::Token![:]>()?;
            Some(input.parse::<syn::LitInt>()?)
        } else {
            None
        };

        // Set up defaults for the bit field information
        let mut access = Access::ReadWrite;
        let mut default = syn::Expr::Verbatim(Default::default());

        // Loop through the rest of the attributes (which is separated by a comma)
        while <Token![,]>::parse(input).is_ok() {
            // Parse the attribute name
            let ident = input.parse::<syn::Ident>()?;

            // Check if the attribute is an access specifier
            if ident == "access" {
                input.parse::<syn::Token![=]>()?;
                access = input.parse::<Access>()?;
            } else if ident == "default" {
                input.parse::<syn::Token![=]>()?;
                default = input.parse::<syn::Expr>()?;
            } else {
                return Err(s_err(ident.span(), "Invalid attribute"));
            }
        }

        // Check if the upper bound is present
        let bits = if let Some(upper_bound) = upper_bound {
            // Parse the lower and upper bounds
            let lower = lower_bound.base10_parse::<usize>().unwrap();
            let upper = upper_bound.base10_parse::<usize>().unwrap();

            // Check if the bounds are valid
            if lower > upper {
                return Err(s_err(
                    upper_bound.span(),
                    "Upper bound is less than the lower bound",
                ));
            }

            // Return the range of bits
            BitRepresentation::Range(lower, upper)
        } else {
            // Parse the single bit
            let bit = lower_bound.base10_parse::<usize>().unwrap();

            // Return the single bit
            BitRepresentation::Single(bit)
        };

        Ok(BitFieldParams {
            bits,
            access,
            default,
        })
    }
}

impl core::default::Default for BitFieldParams {
    fn default() -> Self {
        BitFieldParams {
            bits: BitRepresentation::Single(0),
            access: Access::ReadWrite,
            default: syn::Expr::Verbatim(Default::default()),
        }
    }
}

impl ToTokens for BitField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = &self.field_name;
        let field_type = &self.field_type;

        let test_func_with_name = format_ident!("test_{}", field_name);

        tokens.extend(quote! {
            fn #test_func_with_name() -> #field_type {
                0
            }
        });
    }
}

fn parse_field(field: &syn::Field) -> syn::Result<BitField> {
    Ok(BitField::new_from_field(field))
}
