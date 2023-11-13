// Copyright (c) 2023 xplm-sys developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

// Allow C-like conventions
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! bitfield_impls {
    ($t:ty) => {
        impl $t {
            #[inline]
            #[must_use]
            pub fn field_true(self, field: Self) -> bool {
                self & field == field
            }

            #[inline]
            #[must_use]
            pub fn field_false(self, field: Self) -> bool {
                !self.field_true(field)
            }
        }
        impl std::fmt::Binary for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let val = self.0;
                std::fmt::Binary::fmt(&val, f) // Delegate to interior.
            }
        }
    };
}

bitfield_impls!(XPLMDataTypeID);
bitfield_impls!(XPLMKeyFlags);
bitfield_impls!(XPLMNavType);
