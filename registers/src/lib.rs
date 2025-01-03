#![no_std]

pub use bit_field::BitField;
pub use vcell::VolatileCell;
pub use volatile_register::{RO, RW, WO};

/// Converts a builder struct into raw bits and vice versa
///
/// RawType: The raw bits type (a primitive integer type)
pub trait FromBits<RawType: Sized> {
    /// Converts the raw bits into the builder
    fn from_bits(bits: RawType) -> Self;
    /// Converts the builder into the raw bits
    fn into_bits(&self) -> RawType;
}
/// A readable register
pub trait RegisterRO<Builder, RawType>
where
    RawType: Sized,
    Builder: Copy + Clone + FromBits<RawType>,
{
    /// Reads the value from the register
    fn read(&self) -> Builder;
}
/// A writable register
pub trait RegisterWO<Builder, RawType>
where
    RawType: Sized,
    Builder: Copy + Clone + FromBits<RawType> + Default,
{
    /// Zeroes out the register and returns the builder
    fn zeroed() -> Builder;
    /// Writes the value to the register
    fn write(&mut self, value: Builder);
}
/// A readable and writable register
pub trait RegisterRW<Builder, RawType>:
    RegisterRO<Builder, RawType> + RegisterWO<Builder, RawType>
where
    RawType: Sized,
    Builder: Copy + Clone + FromBits<RawType> + Default,
{
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Builder) -> Builder;
}

/// A trait to read an internal register
pub trait RegisterR {
    type R;

    /// Reads the register
    fn read(&self) -> Self::R;
}

pub trait RegisterW {
    type W;

    /// Writes to the register
    fn write(&self, value: Self::W);
}
