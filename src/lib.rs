// SPDX-FileCopyrightText: © The `asn` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(unsafe_code)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Autonomous System Number (ASN)
///
/// Four-Octet ASN as per [RFC 6793](https://datatracker.ietf.org/doc/html/rfc6793)
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// TODO: serde Serialize, Deserialize with feature
pub struct Asn(u32);

impl Asn {
    /// ASN for AS 0 (zero) as per [RFC 7607](https://datatracker.ietf.org/doc/html/rfc7607)
    pub const ZERO: Self = Self::new(0);

    /// Reserved "Last" 16-bit ASN as per [RFC 7300](https://datatracker.ietf.org/doc/html/rfc7300)
    pub const LAST: Self = Self::new(65535);

    /// Reserved "Last" 32-bit ASN as per [RFC 7300](https://datatracker.ietf.org/doc/html/rfc7300)
    pub const LAST4: Self = Self::new(4294967295);

    #[inline]
    pub const fn new(asn: u32) -> Self {
        Self(asn)
    }

    // TODO: pub const from_str

    // TODO: pub const fn is_reserved_last ?
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

// TODO: core::convert::TryFrom<Asn> for u16

impl core::convert::From<u16> for Asn {
    #[inline]
    fn from(asn: u16) -> Asn {
        Asn::new(u32::from(asn))
    }
}

// TODO: core::str::FromStr for Asn

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
impl alloc::fmt::Display for Asn {
    /// Formats according to "asplain" decimal value representation as per [RFC 5396](https://datatracker.ietf.org/doc/html/rfc5396)
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: "asdot+" alloc::fmt::Display
// TODO: "asdot" alloc::fmt::Display
// https://doc.rust-lang.org/std/fmt/trait.Display.html#internationalization
// https://datatracker.ietf.org/doc/html/rfc5396#section-2

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

    #[test]
    fn test_asn0_eq() {
        assert_eq!(Asn::ZERO, Asn::new(0));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_display() {
        // https://datatracker.ietf.org/doc/html/rfc5396#section-2
        assert_eq!(format!("{}", Asn::new(65526)), "65526");
        assert_eq!(format!("{}", Asn::new(65546)), "65546");
    }

    #[test]
    fn test_last_eq() {
        assert_eq!(Asn::LAST, Asn::new(65535));
        assert_eq!(Asn::LAST4, Asn::new(4294967295));
    }
}
