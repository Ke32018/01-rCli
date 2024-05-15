use std::time::Duration;

use anyhow::{Error, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::JwtSignOpts;

static KEY: &[u8] = b"secret";

pub fn process_jwt_sign(sub: String, aud: String, exp: String) -> Result<String> {
    let mut opts = JwtSignOpts {
        sub,
        aud,
        exp: Duration::default(), // Initialize with a default duration
    };
    if let Err(e) = opts.set_exp_from_duration(&exp) {
        eprintln!("Failed to parse duration: {}", e);
        return Err(Error::msg("Failed to parse duration".to_string()));
    }

    let ret = encode(&Header::default(), &opts, &EncodingKey::from_secret(KEY))?;
    println!("{:?}", opts);
    println!("{:?}", ret);
    Ok(ret)
}

pub fn process_jwt_verify(token: String) -> Result<bool> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["device1"]);
    validation.sub = Some("acme".to_string());
    validation.set_required_spec_claims(&["exp", "sub", "aud"]);
    match decode::<JwtSignOpts>(&token, &DecodingKey::from_secret(KEY), &validation) {
        Ok(_) => {
            println!("Ok");
            Ok(true)
        }
        Err(err) => {
            eprintln!("{:?}", err);
            Err(anyhow::anyhow!("invalid token"))
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    // const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");
}
