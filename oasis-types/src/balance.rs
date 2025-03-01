#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Display,
    LowerHex,
    UpperHex,
    FromStr,
    From,
    Into,
    Add,
    Div,
    Mul,
    Rem,
    Sub,
    AddAssign,
    DivAssign,
    MulAssign,
    RemAssign,
    SubAssign,
)]
#[repr(C)]
pub struct Balance(pub u128);

impl Balance {
    // Alias for `mem::size_of::<Balance>()`.
    pub const fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

impl serde::ser::Serialize for Balance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

impl<'de> serde::de::Deserialize<'de> for Balance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de;

        const EXPECTATION: &str = "16 bytes";

        struct BalanceVisitor;
        impl<'de> de::Visitor<'de> for BalanceVisitor {
            type Value = Balance;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(EXPECTATION)
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                if let Some(len) = seq.size_hint() {
                    if len != Balance::size() {
                        return Err(de::Error::invalid_length(len, &EXPECTATION));
                    }
                }

                let mut bytes = [0u8; Balance::size()];
                let mut i = 0;
                loop {
                    match seq.next_element()? {
                        Some(el) if i < Balance::size() => bytes[i] = el,
                        None if i == Balance::size() => break,
                        _ => return Err(de::Error::invalid_length(i, &EXPECTATION)),
                    }
                    i += 1;
                }

                Ok(Balance(u128::from_be_bytes(bytes)))
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.len() == Balance::size() {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(value);
                    Ok(Balance(u128::from_be_bytes(bytes)))
                } else {
                    Err(de::Error::invalid_length(value.len(), &EXPECTATION))
                }
            }
        }

        deserializer.deserialize_u128(BalanceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut bal = Balance::from(3);
        assert_eq!(bal - Balance::from(2), Balance::from(1));
        bal += 1u128.into();
        assert_eq!(bal, Balance(4))
    }

    #[test]
    fn test_mul() {
        let mut bal = Balance::from(3);
        bal *= 2;
        assert_eq!(u128::from(bal), 6u128);
        assert_eq!(bal % 4, Balance(2));
        assert_eq!(bal / 4, Balance(1));
    }

    #[test]
    fn test_from_str() {
        use std::str::FromStr;
        assert_eq!(
            Balance::from_str("21267647932558653966460912964485513216"),
            Ok(Balance(21267647932558653966460912964485513216u128))
        );
    }

    #[test]
    fn serde_balance() {
        let orig_bal = Balance(21267647932558653966460912964485513216u128);
        let bal: Balance = serde_cbor::from_slice(&serde_cbor::to_vec(&orig_bal).unwrap()).unwrap();
        assert_eq!(bal, orig_bal);
    }
}
