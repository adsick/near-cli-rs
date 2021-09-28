use clap::Clap;
extern crate shell_words;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;

mod commands;
mod common;
mod consts;
// mod operation_mod;
// mod commands_mod;

type CliResult = color_eyre::eyre::Result<()>;

/// near-cli is a toolbox for interacting with NEAR protocol
// #[derive(Debug, Clap)]
// #[clap(
//     version,
//     author,
//     about,
//     setting(clap::AppSettings::ColoredHelp),
//     setting(clap::AppSettings::DisableHelpSubcommand),
//     setting(clap::AppSettings::VersionlessSubcommands),
//     // setting(clap::AppSettings::NextLineHelp)
// )]
// struct CliArgs {
//     #[clap(subcommand)]
//     top_level_command: Option<self::commands::CliTopLevelCommand>,
// }

#[derive(Debug, Clone, InteractiveClap)]
struct Args {
    #[interactive_clap(subcommand)]
    top_level_command: self::commands::TopLevelCommand,
}

impl CliArgs {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        let mut args = self
            .top_level_command
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default();
        args.push_front("./near-cli".to_owned());
        args
    }
}

// impl From<Args> for CliArgs {
//     fn from(cli_args: Args) -> Self {
//         Self {
//             top_level_command: Some(cli_args.top_level_command.into()),
//         }
//     }
// }

impl From<CliArgs> for Args {
    fn from(cli_args: CliArgs) -> Self {
        let top_level_command = match cli_args.top_level_command {
            Some(cli_subcommand) => self::commands::TopLevelCommand::from(cli_subcommand),
            None => self::commands::TopLevelCommand::choose_command(),
        };
        Self { top_level_command }
    }
}

impl Args {
    async fn process(self) -> CliResult {
        self.top_level_command.process().await
    }
}

fn main() -> CliResult {
    color_eyre::install()?;

    let cli = CliArgs::parse();

    // if let Some(self::commands::CliTopLevelCommand::GenerateShellCompletions(subcommand)) =
    //     cli.top_level_command
    // {
    //     subcommand.process();
    //     return Ok(());
    // }

    let args = Args::from(cli);

    let completed_cli = CliArgs::from(args.clone());


    let process_result = actix::System::new().block_on(args.process());

    println!(
        "Your console command:\n{}",
        shell_words::join(&completed_cli.to_cli_args())
    );

    process_result
}
