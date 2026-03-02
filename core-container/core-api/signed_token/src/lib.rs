use base_x::{decode, encode};
use poly1305::{
    Poly1305,
    universal_hash::{KeyInit, UniversalHash, crypto_common::KeySizeUser},
};
use subtle::ConstantTimeEq;

const BASE: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+";

#[derive(Clone)]
pub struct Signed {
    secret: Box<[u8]>,
}

impl Signed {
    pub fn new(secret: impl AsRef<[u8]>) -> Option<Self> {
        let s = secret.as_ref().to_vec().into_boxed_slice();
        (s.len() == Poly1305::key_size()).then_some(Self { secret: s })
    }

    pub fn sign(&self, value: i64) -> Option<String> {
        let value_bytes = value.to_be_bytes();

        let mut mac = Poly1305::new_from_slice(&self.secret).ok()?;

        mac.update_padded(&value_bytes);
        let tag_bytes = mac.finalize();

        let mut raw_token = Vec::with_capacity(24);
        raw_token.extend_from_slice(&tag_bytes);
        raw_token.extend_from_slice(&value_bytes);

        Some(encode(BASE, &raw_token))
    }

    pub fn verify(&self, token: &str) -> Option<i64> {
        let decoded = decode(BASE, token).ok()?;

        if decoded.len() != 24 {
            return None;
        }

        let (tag_bytes, value_bytes) = decoded.split_at(16);

        let mut mac = Poly1305::new_from_slice(&self.secret).ok()?;
        mac.update_padded(value_bytes);
        let expected_tag = mac.finalize();

        if expected_tag.ct_eq(tag_bytes).unwrap_u8() != 1 {
            return None;
        }

        Some(i64::from_be_bytes(value_bytes.try_into().ok()?))
    }
}
