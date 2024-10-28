Newtype for Autonomous System Number (ASN)

```rust
use asn::Asn;

let asn = Asn::new(0);
assert_eq!(asn, Asn::ZERO);

let asn = Asn::from_str("4294967295").expect("valid const last 32-bit ASN");
assert_eq!(asn, Asn::LAST4);
```
