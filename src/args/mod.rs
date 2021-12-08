use std::result::Result;

pub struct CallArgs {
    pub privileges: Privilege,
    pub command: Command,
}

impl CallArgs {
    #[allow(clippy::single_match)]
    pub async fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub enum Privilege {
    Normal,
    Experimental,
}

pub enum Command {
    Dummy
}

pub struct ClapArgumentLoader {}

impl ClapArgumentLoader {
    pub async fn load_from_cli() -> std::result::Result<CallArgs, Box<dyn std::error::Error>> {
        let command = clap::App::new("viking")
            .version(env!("CARGO_PKG_VERSION"))
            .about("A rusty text templating application for CLIs.")
            .author("Weber, Alexander <aw@voidpointergroup.com>")
            .arg(clap::Arg::with_name("experimental")
                    .short("e")
                    .long("experimental")
                    .value_name("EXPERIMENTAL")
                    .help("Enables experimental features that do not count as stable.")
                    .required(false)
                    .takes_value(false))
            .subcommand(clap::App::new("dummy")
                .about("Initializes a dummy default configuration in \"./.viking/config.yaml\"."))
            .get_matches();

        let privileges = if command.is_present("experimental") {
            Privilege::Experimental
        } else {
            Privilege::Normal
        };

        if command.subcommand_matches("dummy").is_some() {
            return Ok(CallArgs {
                privileges,
                command: Command::Dummy,
            });
        }
        Err(Box::new(crate::error::UnknownCommand::default()))
    }
}
