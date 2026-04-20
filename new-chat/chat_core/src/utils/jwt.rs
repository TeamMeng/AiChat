use crate::User;
use jwt_simple::prelude::*;
use std::ops::Deref;

const ACCESS_DURATION: u64 = 60 * 15; // 15 min
const REFRESH_DURATION: u64 = 60 * 60 * 24 * 7; // 7 day
const ACCESS_ISSUER: &str = "chat_server:access";
const REFRESH_ISSUER: &str = "chat_server:refresh";
const JWT_AUDIENCE: &str = "chat_web";

pub struct EncodingKey(Ed25519KeyPair);

pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign_access(&self, user: User) -> Result<String, jwt_simple::Error> {
        self.sign_token(user, ACCESS_ISSUER, ACCESS_DURATION)
    }

    pub fn sign_refresh(&self, user: User) -> Result<String, jwt_simple::Error> {
        self.sign_token(user, REFRESH_ISSUER, REFRESH_DURATION)
    }

    pub fn sign_token(
        &self,
        user: User,
        issuer: &str,
        duration: u64,
    ) -> Result<String, jwt_simple::Error> {
        let claims = Claims::with_custom_claims(user, Duration::from_secs(duration))
            .with_issuer(issuer)
            .with_audience(JWT_AUDIENCE);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    /// 验证 Access Token
    pub fn verify_access(&self, token: &str) -> Result<User, jwt_simple::Error> {
        self.verify_token(token, ACCESS_ISSUER)
    }

    /// 验证 Refresh Token
    pub fn verify_refresh(&self, token: &str) -> Result<User, jwt_simple::Error> {
        self.verify_token(token, REFRESH_ISSUER)
    }

    fn verify_token(&self, token: &str, expected_issuer: &str) -> Result<User, jwt_simple::Error> {
        let options = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[expected_issuer])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUDIENCE])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let fullname = "TeamMeng";
        let email = "TeamMeng@123.com";
        let user = User::new(1, fullname, email);

        let token = ek.sign_access(user.clone())?;
        let ret = dk.verify_access(&token)?;

        assert_eq!(ret, user);

        let token = ek.sign_refresh(user.clone())?;
        let ret = dk.verify_refresh(&token)?;

        assert_eq!(ret, user);

        Ok(())
    }
}
