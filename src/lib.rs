mod cli_opts;
mod process;
mod utils;

pub use cli_opts::*;
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
