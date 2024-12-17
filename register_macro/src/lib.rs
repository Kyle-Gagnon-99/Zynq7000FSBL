#![no_std]
#[doc = include_str!("../README.md")]
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, LitBool, Token};

use registers;
use volatile_register;

/// An enum used to represent if a bit field is a single bit or a range of bits
#[derive(Debug, Clone, Copy)]
enum BitRepresentation {
    Single(usize),
    Range(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

struct BitField {
    ident: syn::Ident,
    bits: BitRepresentation,
    ty: syn::Type,
    default: TokenStream,
    access: Access,
}

struct BitFieldParams {
    /// The bit representation of the field
    bits: BitRepresentation,

    /// The access specifier of the field
    access: Access,

    /// The default value of the field
    default: Option<syn::Expr>,

    /// If it is an enum, then the native type of the enum
    repr: Option<syn::Type>,
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

    let builder_name = format_ident!("{}Builder", name);

    // Parse the fields into a TokenStream
    let fields = field
        .named
        .iter()
        .map(BitField::new)
        .map(|bf| bf.map(|bf| bf.into_token_stream()).unwrap())
        .collect::<TokenStream>();

    Ok(quote! {
        pub struct #name {
            value: volatile_register::RW<#ty>,
        }

        pub struct #builder_name {
            pub bits: #ty
        }

        impl #builder_name {
            pub fn new() -> Self {
                Self { bits: #ty::default() }
            }

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

        // Convert the bit range to a BitRepresentation
        let bits = if let Some(upper_bound) = upper_bound {
            let lower = lower_bound.base10_parse::<usize>().unwrap();
            let upper = upper_bound.base10_parse::<usize>().unwrap();

            if lower > upper {
                return Err(s_err(
                    upper_bound.span(),
                    "Upper bound is less than the lower bound",
                ));
            }

            BitRepresentation::Range(lower, upper)
        } else {
            let bit = lower_bound.base10_parse::<usize>().unwrap();
            BitRepresentation::Single(bit)
        };

        // Set up defaults for the bit field information
        let mut return_type = Self {
            bits,
            access: Access::ReadWrite,
            default: None,
            repr: None,
        };

        // Loop through the rest of the attributes (which is separated by a comma)
        while <Token![,]>::parse(input).is_ok() {
            // Parse the attribute name
            let ident = input.parse::<syn::Ident>()?;

            // Check if the attribute is an access specifier
            if ident == "access" {
                input.parse::<syn::Token![=]>()?;
                return_type.access = input.parse::<Access>()?;
            } else if ident == "default" {
                input.parse::<syn::Token![=]>()?;
                return_type.default = Some(input.parse::<syn::Expr>()?);
            } else if ident == "repr" {
                input.parse::<syn::Token![=]>()?;
                return_type.repr = Some(input.parse::<syn::Type>()?);
            } else {
                return Err(s_err(ident.span(), "Invalid attribute"));
            }
        }

        Ok(return_type)
    }
}

impl core::default::Default for BitFieldParams {
    fn default() -> Self {
        BitFieldParams {
            bits: BitRepresentation::Single(0),
            access: Access::ReadWrite,
            default: Some(syn::parse_quote!(false)),
            repr: None,
        }
    }
}

impl BitField {
    pub fn new(syn_field: &syn::Field) -> syn::Result<Self> {
        // First, parse the attribute of the field to a BitFieldParams
        let bit_field_params = syn_field.attrs.iter().find_map(|attr| {
            if attr.path().is_ident("bits") {
                attr.parse_args::<BitFieldParams>().ok()
            } else {
                None
            }
        });

        // Check if the attribute is present
        let bit_field_params = match bit_field_params {
            Some(params) => params,
            None => return Err(s_err(syn_field.span(), "No bit field attribute found")),
        };

        // Get the type of the field and match it to a TypeClass
        let (ty_class, bits) = to_types(&syn_field.ty);

        // If the type is a boolean, then the number of bits should be 1
        if ty_class == TypeClass::Bool && bits != 1 {
            return Err(s_err(
                syn_field.ty.span(),
                "Boolean type should only be 1 bit",
            ));
        };

        // If the type is an enum (which would be reported as Other), check if the repr attribute is present
        if ty_class == TypeClass::Other && bit_field_params.repr.is_none() {
            return Err(s_err(
                syn_field.ty.span(),
                "Enum type should have a repr attribute",
            ));
        };

        // If the type is an enum, the number of bits should be at least the same size or smaller than the native type
        if ty_class == TypeClass::Other {
            let repr = bit_field_params.repr.as_ref().unwrap();
            let (_, repr_bits) = to_types(repr);

            if bits > repr_bits {
                return Err(s_err(
                    syn_field.ty.span(),
                    "Size of enum is smaller than the type",
                ));
            }
        };

        // Check if the default value is present
        let default = match bit_field_params.default {
            Some(default) => default.into_token_stream(),
            None => syn::parse_quote!(false),
        };

        Ok(BitField {
            ident: syn_field.ident.clone().unwrap(),
            bits: bit_field_params.bits,
            ty: syn_field.ty.clone(),
            default,
            access: bit_field_params.access,
        })
    }
}

impl ToTokens for BitField {
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        let BitField {
            ident,
            bits,
            ty,
            default,
            access,
        } = self;

        let getter_name = format_ident!("{}", ident);
        let setter_name = format_ident!("with_{}", ident);

        let start_bit = match bits {
            BitRepresentation::Single(bit) => bit,
            BitRepresentation::Range(start, _) => start,
        };

        let end_bit = match bits {
            BitRepresentation::Single(bit) => bit,
            BitRepresentation::Range(_, end) => end,
        };

        let get_bits = if start_bit == end_bit {
            quote! {
                self.bits.get_bit(#start_bit)
            }
        } else {
            quote! {
                self.bits.get_bits(#start_bit..=#end_bit)
            }
        };

        let set_bits = if start_bit == end_bit {
            quote! {
                self.bits.set_bit(#start_bit, (value as #ty).into())
            }
        } else {
            quote! {
                self.bits.set_bits(#start_bit..=#end_bit, (value as #ty).into())
            }
        };

        let getter_function = if matches!(access, Access::Read | Access::ReadWrite) {
            quote! {
                pub fn #getter_name(&self) -> #ty {
                    #get_bits
                }
            }
        } else {
            quote! {}
        };

        let setter_function = if matches!(access, Access::Write | Access::ReadWrite) {
            quote! {
                pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                    #set_bits;
                    self
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #getter_function
            #setter_function
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        self.into_token_stream()
    }

    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.into_token_stream());
    }
}
