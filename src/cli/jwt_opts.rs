use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use tracing::info;

use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JWTSubCommand {
    #[command(name = "sign", about = "Sign a JWT token")]
    Sign(JWTSignOpts),
    #[command(name = "verify", about = "Verify a JWT token")]
    Verify(JWTVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JWTSignOpts {
    #[arg(short, long, help = "The subject of the JWT token")]
    pub sub: String,
    #[arg(short, long, help = "The audience of the JWT token")]
    pub aud: String,
    #[arg(short, long, help = "The expiration time of the JWT token")]
    pub exp: u32,
}

#[derive(Debug, Parser)]
pub struct JWTVerifyOpts {
    #[arg(short, long, help = "The JWT token to verify")]
    pub token: String,
    #[arg(short, long, help = "The audience of the JWT token")]
    pub aud: String,
}

impl CmdExecutor for JWTSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = crate::process_jwt_sign(&self.sub, &self.aud, self.exp).await?;
        info!("token: {}", token);
        Ok(())
    }
}

impl CmdExecutor for JWTVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let claims = crate::process_jwt_verify(&self.token, &self.aud).await?;
        info!("claims: {:?}", claims);
        Ok(())
    }
}
