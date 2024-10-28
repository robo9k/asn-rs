// SPDX-FileCopyrightText: © The `asn` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(unsafe_code)]
#![cfg_attr(not(any(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]

/// Autonomous System Number (ASN)
///
/// Four-Octet ASN as per [RFC 6793](https://datatracker.ietf.org/doc/html/rfc6793)
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// TODO: serde Serialize, Deserialize with feature
pub struct Asn(u32);

impl Asn {
    /// Reserved ASN for RPKI AS 0 (zero) as per [RFC 7607](https://datatracker.ietf.org/doc/html/rfc7607)
    pub const ZERO: Self = Self::new(0);

    /// Reserved "Last" 16-bit ASN as per [RFC 7300](https://datatracker.ietf.org/doc/html/rfc7300)
    pub const LAST: Self = Self::new(65535);

    /// Reserved "Last" 32-bit ASN as per [RFC 7300](https://datatracker.ietf.org/doc/html/rfc7300)
    pub const LAST4: Self = Self::new(4294967295);

    /// Reserved for documentation use (16-bit number set) as per [RFC 5398](https://datatracker.ietf.org/doc/html/rfc5398)
    pub const RESERVED_DOCUMENTATION: core::ops::RangeInclusive<Self> =
        (Self::new(64496)..=Self::new(64511));

    /// Reserved for documentation use (32-bit number set) as per [RFC 5398](https://datatracker.ietf.org/doc/html/rfc5398)
    pub const RESERVED_DOCUMENTATION4: core::ops::RangeInclusive<Self> =
        (Self::new(65536)..=Self::new(65551));

    /// Reserved for private use (16-bit ASNs) as per [RFC 6996](https://datatracker.ietf.org/doc/html/rfc6996)
    pub const RESERVED_PRIVATE: core::ops::RangeInclusive<Self> =
        (Self::new(64512)..=Self::new(65534));

    /// Reserved for private use (32-bit ASNs) as per [RFC 6996](https://datatracker.ietf.org/doc/html/rfc6996)
    pub const RESERVED_PRIVATE4: core::ops::RangeInclusive<Self> =
        (Self::new(4200000000)..=Self::new(4294967294));

    /// Reserved to represent non-mappable four-octet AS numbers as two-octet AS numbers as per [RFC 6793](https://datatracker.ietf.org/doc/html/rfc6793)
    pub const TRANS: Self = Self::new(23456);

    /// Reserved as per [IANA 32-bit ASNs](https://www.iana.org/assignments/as-numbers/as-numbers.xhtml)
    pub const RESERVED_IANA4: core::ops::RangeInclusive<Self> =
        (Self::new(65552)..=Self::new(131071));

    #[inline]
    pub const fn new(asn: u32) -> Self {
        Self(asn)
    }

    pub const fn from_str(src: &str) -> Result<Self, ParseAsnError> {
        if src.is_empty() {
            return Err(ParseAsnError());
        }

        // all valid digits are ascii, so we will just iterate over the utf8 bytes
        // and cast them to chars. .to_digit() will safely return None for anything
        // other than a valid ascii digit for the given radix, including the first-byte
        // of multi-byte sequences
        let src = src.as_bytes();
        let mut digits = src;

        let mut result = 0;

        macro_rules! unwrap_or_PAE {
            ($option:expr) => {
                match $option {
                    Some(value) => value,
                    None => return Err(ParseAsnError()),
                }
            };
        }

        #[inline(always)]
        pub const fn can_not_overflow<T>(digits: &[u8]) -> bool {
            digits.len() <= core::mem::size_of::<T>() * 2
        }

        if can_not_overflow::<u32>(digits) {
            // If the len of the str is short compared to the range of the type
            // we are parsing into, then we can be certain that an overflow will not occur.
            // This bound is when `radix.pow(digits.len()) - 1 <= T::MAX` but the condition
            // above is a faster (conservative) approximation of this.
            //
            // Consider radix 16 as it has the highest information density per digit and will thus overflow the earliest:
            // `u8::MAX` is `ff` - any str of len 2 is guaranteed to not overflow.
            // `i8::MAX` is `7f` - only a str of len 1 is guaranteed to not overflow.
            while let [c, rest @ ..] = digits {
                result *= 10_u32;
                let x = unwrap_or_PAE!((*c as char).to_digit(10));
                result += x;
                digits = rest;
            }
        } else {
            while let [c, rest @ ..] = digits {
                // When `radix` is passed in as a literal, rather than doing a slow `imul`
                // the compiler can use shifts if `radix` can be expressed as a
                // sum of powers of 2 (x*10 can be written as x*8 + x*2).
                // When the compiler can't use these optimisations,
                // the latency of the multiplication can be hidden by issuing it
                // before the result is needed to improve performance on
                // modern out-of-order CPU as multiplication here is slower
                // than the other instructions, we can get the end result faster
                // doing multiplication first and let the CPU spends other cycles
                // doing other computation and get multiplication result later.
                let mul = result.checked_mul(10_u32);
                let x = unwrap_or_PAE!((*c as char).to_digit(10));
                result = unwrap_or_PAE!(mul);
                result = unwrap_or_PAE!(u32::checked_add(result, x));
                digits = rest;
            }
        }
        Ok(Self(result))
    }

    // TODO: pub const fn is_reserved_last ?

    // TODO: pub const fn is_reserved_documentation ?

    // TODO: pub const fn is_reserved_private ?

    // TODO: pub const fn is_reserved_trans ?

    // TODO: pub const fn is_reserved_iana ? (reserved4 or just reserved would clash with a fn encompassing the other reservations)

    // TODO: pub const fn is_public ?

    // TODO: pub const BITS
    // TODO: pub const MIN, pub const MAX
}

// TODO: with reference core::convert::From<&Asn> for u32

impl core::convert::From<Asn> for u32 {
    #[inline]
    fn from(asn: Asn) -> u32 {
        asn.0
    }
}

// TODO: infallible core::convert::TryFrom<u32> for Asn

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

impl core::str::FromStr for Asn {
    type Err = ParseAsnError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str(src)
    }
}

impl core::fmt::Display for Asn {
    /// Formats according to "asplain" decimal value representation as per [RFC 5396](https://datatracker.ietf.org/doc/html/rfc5396)
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: "asdot+" core::fmt::Display
// TODO: "asdot" core::fmt::Display
// https://doc.rust-lang.org/std/fmt/trait.Display.html#internationalization
// https://datatracker.ietf.org/doc/html/rfc5396#section-2

// TODO: nightly core::iter::Step for Asn

/// Error which can be returned when parsing an [`Asn`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAsnError();

impl core::fmt::Display for ParseAsnError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "can not parse ASN")
    }
}

impl core::error::Error for ParseAsnError {}

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

    #[test]
    fn test_reserved_documentation_contains() {
        assert!(Asn::RESERVED_DOCUMENTATION.contains(&Asn::new(64500)));
        assert!(Asn::RESERVED_DOCUMENTATION4.contains(&Asn::new(65540)));
    }

    #[test]
    fn test_reserved_private_contains() {
        assert!(Asn::RESERVED_PRIVATE.contains(&Asn::new(64520)));
        assert!(Asn::RESERVED_PRIVATE4.contains(&Asn::new(4242424242)));
    }

    #[test]
    fn test_trans_eq() {
        assert_eq!(Asn::TRANS, Asn::new(23456));
    }

    #[test]
    fn test_reserved_iana_contains() {
        assert!(Asn::RESERVED_IANA4.contains(&Asn::new(100000)));
    }

    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        // https://datatracker.ietf.org/doc/html/rfc5396#section-2
        assert_eq!(Asn::from_str("65526")?, Asn::new(65526));
        assert_eq!(Asn::from_str("65546")?, Asn::new(65546));

        assert_eq!(Asn::from_str("hurz").unwrap_err(), ParseAsnError());

        Ok(())
    }

    #[test]
    fn test_parseasnerror_display() {
        assert_eq!(
            format!("{}", Asn::from_str("hurz").unwrap_err()),
            "can not parse ASN"
        );
    }

    #[test]
    fn test_fromstr() {
        assert_eq!("65526".parse(), Ok(Asn::new(65526)));
    }
}
