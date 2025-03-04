use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: u32,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: &'static [u8],
    pub algorithm: Algorithm,
    pub expiry: u32,
}

pub async fn process_jwt_sign(sub: &str, aud: &str, exp: u32) -> Result<String> {
    let config = JwtConfig::default();

    let expiry = if exp > 0 {
        get_current_timestamp() + exp
    } else {
        get_current_timestamp() + config.expiry
    };

    let claims = Claims::new(sub.to_string(), aud.to_string(), expiry);
    let token = claims.encode().await?;

    info!("生成JWT令牌成功: sub={}, aud={}, exp={}", sub, aud, expiry);
    Ok(token)
}

pub async fn process_jwt_verify(token: &str, aud: &str) -> Result<Claims> {
    let claims = Claims::decode(token, aud).await?;

    Ok(claims)
}

/// 获取当前UNIX时间戳（秒）
fn get_current_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs() as u32
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: b"secret",
            algorithm: Algorithm::HS256,
            expiry: 3600,
        }
    }
}

impl Claims {
    pub fn new(sub: String, aud: String, exp: u32) -> Self {
        Self { sub, aud, exp }
    }

    pub async fn encode(&self) -> Result<String> {
        let config = JwtConfig::default();
        let header = Header::new(config.algorithm);
        let key = EncodingKey::from_secret(config.secret);
        let token = encode::<Claims>(&header, self, &key).context("JWT令牌编码失败")?;

        Ok(token)
    }

    pub async fn decode(token: &str, aud: &str) -> Result<Self> {
        let config = JwtConfig::default();
        let mut validation = Validation::new(config.algorithm);
        // 验证是否过期
        validation.validate_exp = true;
        // 允许有60秒的时间偏差
        validation.leeway = 60;
        validation.set_audience(&[aud]);
        // 解码JWT令牌
        let key = DecodingKey::from_secret(config.secret);
        let claims = decode::<Claims>(token, &key, &validation)?;

        Ok(claims.claims)
    }
}
