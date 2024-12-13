#![no_std]
#[doc = include_str!("../README.md")]
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

use registers;

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
/// #[bitfield(0:1, access = RW, )] or #[bitfield(0)]
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
/// After the annotation will be the declaration of the field. The declaration should be a type (bool, u8, u16, u32, etc.) or even an Enum
/// If it is an Enum, then the Enum should have a #[repr(uSize)] attribute where uSize is the size of the register. Since this library
/// relies on the registers crate, the Enum should be implemented with the FromBits and IntoBits traits.
struct BitField {
    bits: usize,
    access: Access,
    ty: syn::Type,
}

struct BitFieldParams {
    bits: syn::LitInt,
    access: Access,
    ty: syn::Type,
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
        .map(|f| f.unwrap())
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

impl BitField {
    /// Create a new bit field member
    fn new(bits: usize, access: Access) -> Self {
        BitField {
            bits,
            access,
            ty: syn::Type::Verbatim(Default::default()),
        }
    }

    /// Create a new bit field member from a field
    fn new_from_field(field: &syn::Field) -> syn::Result<Self> {
        // Attempt to get the bits attribute from the field
        let BitFieldParams { bits, access, ty } = field
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("bitfield") {
                    Some(attr.parse_args::<BitFieldParams>())
                } else {
                    None
                }
            })
            .next()
            .unwrap_or_else(|| Ok(BitFieldParams::default()))?;

        Ok(BitField::new(0, Access::None))
    }
}

impl ToTokens for BitField {
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        let bits = self.bits;
        let access = match self.access {
            Access::Read => quote! { R },
            Access::Write => quote! { W },
            Access::ReadWrite => quote! { RW },
            Access::WriteToClear => quote! { WTC },
            Access::None => quote! { RW },
        };

        quote! {
            pub fn test() -> u32 {
                0
            }
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        self.into_token_stream()
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.into_token_stream());
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

        // Next parse a comma
        input.parse::<syn::Token![,]>()?;

        // Parse the access specifier
        let access = input.parse::<Access>()?;

        // Parse a comma
        input.parse::<syn::Token![,]>()?;

        // Parse the type
        let ty = input.parse::<syn::Type>()?;

        // Get the number of bits
        let (ty_class, bits) = to_types(&ty);

        // Return for now
        Ok(BitFieldParams {
            bits: lower_bound,
            access,
            ty,
        })
    }
}

impl core::default::Default for BitFieldParams {
    fn default() -> Self {
        BitFieldParams {
            bits: syn::LitInt::new("0", proc_macro2::Span::call_site()),
            access: Access::ReadWrite,
            ty: syn::Type::Verbatim(Default::default()),
        }
    }
}
