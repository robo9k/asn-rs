// SPDX-FileCopyrightText: © The `asn` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(unsafe_code)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

/// Autonomous System Number (ASN)
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// serde Serialize, Deserialize with feature
pub struct Asn(u32);

impl Asn {
    #[inline]
    pub const fn new(asn: u32) -> Self {
        Self(asn)
    }
}

impl core::convert::From<Asn> for u32 {
    #[inline]
    fn from(asn: Asn) -> u32 {
        asn.0
    }
}

impl core::convert::From<u32> for Asn {
    #[inline]
    fn from(asn: u32) -> Asn {
        Asn::new(asn)
    }
}

// TryFrom<Asn> for u16

impl core::convert::From<u16> for Asn {
    #[inline]
    fn from(asn: u16) -> Asn {
        Asn::new(u32::from(asn))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Asn>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Asn>();
    }

    #[test]
    fn test_new() {
        let _ = Asn::new(0);
    }

    #[test]
    fn test_u32_from_asn() {
        let _: u32 = Asn::new(0).into();
    }

    #[test]
    fn test_asn_from_u32() {
        let _: Asn = 0_u32.into();
    }

    #[test]
    fn test_asn_from_u16() {
        let _: Asn = 0_u16.into();
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Asn::new(0)), "Asn(0)");
    }
}
