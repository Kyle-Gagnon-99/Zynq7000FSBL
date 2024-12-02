#![no_std]

pub use bit_field::BitField;
pub use vcell::VolatileCell;
pub use volatile_register::{RO, RW, WO};

/// A readable register
pub trait RegisterR {
    /// Type-safe reader for the register value
    type R: Copy + Clone;

    /// Reads the value from the register
    fn read(&self) -> Self::R;
}

/// A writable register
pub trait RegisterW {
    /// Type-safe writer to the register value
    type W: Copy + Clone;

    /// Zeroes out the register and returns the writer
    fn zeroed() -> Self::W;

    /// Writes the value to the register
    fn write(&mut self, value: Self::W);
}

/// A readable and writable register
pub trait RegisterRW: RegisterR + RegisterW {
    fn modify<F: FnOnce(<Self as RegisterR>::R, <Self as RegisterW>::W) -> <Self as RegisterW>::W>(
        &mut self,
        f: F,
    );
}

#[macro_export]
macro_rules! register {
    () => {};
}

#[macro_export]
macro_rules! register_at {
    ($name: ident, $addr: expr, $ctor: ident) => {
        impl $name {
            #[allow(unused)]
            #[inline]
            pub fn $ctor() -> &'static mut Self {
                let addr = $addr as *mut Self;
                unsafe { &mut *addr }
            }
        }
    };
}
