use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use color_eyre::eyre::Result;
use tokio::runtime::Builder;

use crate::loader::program_loader;

pub mod arg;
pub mod env;
pub mod error;
pub mod loader;
pub mod platforms;
pub mod utils;
pub mod visual;

fn main() -> Result<()> {
    Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(program_loader())?;

    Ok(())
}
