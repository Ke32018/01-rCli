use std::time::Duration;

use crate::{parse_duration, process_jwt_sign, process_jwt_verify, CmdExector};

use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[clap(about = "sign a jwt token")]
    Sign(JwtClaim),
    #[command(about = "verify a jwt token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtSignOpts {
    pub sub: String,
    pub aud: String,
    #[serde(with = "serde_duration")]
    pub exp: Duration,
}

#[derive(Debug, Parser, Serialize, Deserialize)]

pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExector for JwtClaim {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_sign(self.sub, self.aud, self.exp)?;
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_jwt_verify(self.token)?;
        Ok(())
    }
}

// 自定义序列化和反序列化 Duration 的方式
mod serde_duration {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(duration.as_secs() as i64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = i64::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds as u64))
    }
}

#[derive(Debug, Parser)]
pub struct JwtClaim {
    #[clap(long)]
    sub: String,
    #[clap(long)]
    aud: String,
    #[clap(long)]
    exp: String, // Change this to a string to pass the duration representation
}

impl JwtSignOpts {
    // Assuming you have a method to set the exp field based on the current time and a duration string
    pub fn set_exp_from_duration(&mut self, duration_str: &str) -> Result<(), &'static str> {
        let exp_duration = parse_duration(duration_str)?;
        self.exp = exp_duration;
        Ok(())
    }
}
