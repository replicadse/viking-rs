include!("check_features.rs");

use futures::executor::block_on;

use std::result::Result;

pub mod args;
pub mod error;

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = crate::args::ClapArgumentLoader::load_from_cli().await?;
    cmd.validate().await?;

    match cmd.command {
        crate::args::Command::Dummy => {
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    block_on(async_main())
}
