#![cfg_attr(not(feature = "std"), no_std)]
#![allow(async_fn_in_trait)]
#![allow(clippy::unused_unit)] // enumset
#![warn(clippy::large_futures)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(target_arch = "xtensa", feature(asm_experimental_arch))]

#[cfg(all(not(feature = "riscv-ulp-hal"), not(feature = "esp-idf-sys")))]
compile_error!("Neither feature `esp-idf-sys`, nor feature `riscv-ulp-hal` is enabled");

#[cfg(all(
    not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))),
    not(esp_idf_comp_driver_enabled)
))]
compile_error!("esp-idf-hal requires the `driver` ESP-IDF component to be enabled");

#[cfg(all(
    any(
        feature = "std",
        feature = "alloc",
        feature = "critical-section-interrupt",
        feature = "critical-section-mutex"
    ),
    all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))
))]
compile_error!("Enabling feature `riscv-ulp-hal` implies no other feature is enabled");

#[cfg(all(
    all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys")),
    not(esp32s2)
))]
compile_error!("Feature `riscv-ulp-hal` is currently only supported on esp32s2");

#[macro_use]
pub mod riscv_ulp_hal;

pub mod adc;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod can;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod cpu;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod delay;
pub mod gpio;
#[cfg(all(esp32, esp_idf_version_major = "4"))]
pub mod hall;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod i2c;
#[cfg(all(
    not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))),
    esp_idf_comp_driver_enabled
))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(esp_idf_soc_i2s_supported, esp_idf_comp_driver_enabled)))
)]
pub mod i2s;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod interrupt;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod io;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod ledc;
#[cfg(all(
    any(all(esp32, esp_idf_eth_use_esp32_emac), esp_idf_eth_use_openeth),
    not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys")))
))]
pub mod mac;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod modem;
#[cfg(all(
    not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))),
    any(esp32, esp32s2, esp32s3)
))]
pub mod pcnt;
pub mod peripheral;
pub mod peripherals;
pub mod prelude;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod reset;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod rmt;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod rom;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod spi;
pub mod sys;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod task;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod timer;
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
pub mod uart;
#[cfg(all(
    any(esp32, esp32s2, esp32s3, esp32c6, esp32p4),
    not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys")))
))]
pub mod ulp;
pub mod units;

#[cfg(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys")))]
pub use crate::riscv_ulp_hal::delay;

// This is used to create `embedded_hal` compatible error structs
// that preserve original `EspError`.
//
// Example:
// embedded_hal_error!(I2cError, embedded_hal::i2c::Error, embedded_hal::i2c::ErrorKind)
#[cfg(not(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys"))))]
#[allow(unused_macros)]
macro_rules! embedded_hal_error {
    ($error:ident, $errortrait:ty, $kind:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $error {
            kind: $kind,
            cause: esp_idf_sys::EspError,
        }

        impl $error {
            pub fn new(kind: $kind, cause: esp_idf_sys::EspError) -> Self {
                Self { kind, cause }
            }
            pub fn other(cause: esp_idf_sys::EspError) -> Self {
                Self::new(<$kind>::Other, cause)
            }
            pub fn cause(&self) -> esp_idf_sys::EspError {
                self.cause
            }
        }
        impl From<esp_idf_sys::EspError> for $error {
            fn from(e: esp_idf_sys::EspError) -> Self {
                Self::other(e)
            }
        }

        impl $errortrait for $error {
            fn kind(&self) -> $kind {
                self.kind
            }
        }

        impl core::fmt::Display for $error {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{} {{ kind: {}, cause: {} }}",
                    stringify!($error),
                    self.kind,
                    self.cause()
                )
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for $error {}
    };
}

#[cfg(all(feature = "riscv-ulp-hal", not(feature = "esp-idf-sys")))]
#[allow(unused_macros)]
macro_rules! embedded_hal_error {
    ($error:ident, $errortrait:ty, $kind:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $error {
            kind: $kind,
            cause: crate::riscv_ulp_hal::sys::EspError,
        }

        impl $error {
            pub fn new(kind: $kind, cause: crate::riscv_ulp_hal::sys::EspError) -> Self {
                Self { kind, cause }
            }
            pub fn other(cause: crate::riscv_ulp_hal::sys::EspError) -> Self {
                Self::new(<$kind>::Other, cause)
            }
            pub fn cause(&self) -> crate::riscv_ulp_hal::sys::EspError {
                self.cause
            }
        }
        impl From<crate::riscv_ulp_hal::sys::EspError> for $error {
            fn from(e: crate::riscv_ulp_hal::sys::EspError) -> Self {
                Self::other(e)
            }
        }

        impl $errortrait for $error {
            fn kind(&self) -> $kind {
                self.kind
            }
        }

        impl core::fmt::Display for $error {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{} {{ kind: {}, cause: {} }}",
                    stringify!($error),
                    self.kind,
                    self.cause()
                )
            }
        }
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! into_ref {
    ($($name:ident),*) => {
        $(
            let $name = $name.into_ref();
        )*
    }
}

#[allow(unused_macros)]
macro_rules! impl_peripheral_trait {
    ($type:ident) => {
        unsafe impl Send for $type {}

        impl $crate::peripheral::sealed::Sealed for $type {}

        impl $crate::peripheral::Peripheral for $type {
            type P = $type;

            #[inline]
            unsafe fn clone_unchecked(&mut self) -> Self::P {
                $type { ..*self }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_peripheral {
    ($type:ident) => {
        pub struct $type(::core::marker::PhantomData<*const ()>);

        impl $type {
            /// # Safety
            ///
            /// Care should be taken not to instantiate this peripheral instance, if it is already instantiated and used elsewhere
            #[inline(always)]
            pub unsafe fn new() -> Self {
                $type(::core::marker::PhantomData)
            }
        }

        $crate::impl_peripheral_trait!($type);
    };
}

#[allow(unused_imports)]
pub(crate) use embedded_hal_error;
#[allow(unused_imports)]
pub(crate) use impl_peripheral;
#[allow(unused_imports)]
pub(crate) use impl_peripheral_trait;
