#![forbid(unsafe_code)]

mod advisories;
mod attest;
mod feed;
mod http;
mod index;
mod rekor;
mod repack;
mod sign;
mod snapshot;
mod sync_aur;
mod tool_channel;
mod vendor;
mod verdict;

use std::ffi::OsString;

use eyre::Result;
use pacvamp_cli_support::version::{BinInfo, Version};
use usage_rs::RunWith;

const BIN: BinInfo = BinInfo {
    name: "pacvamp-repo",
    version: env!("CARGO_PKG_VERSION"),
};

/// Server-side tooling for a repository that serves pacvamp clients
///
/// Publish signed indexes, build provenance, verdicts, advisories, snapshots,
/// and vetted tool releases. Gate package signatures and AUR syncs, and generate
/// vendor packages from verified packslips. See https://pacvamp.com/adoption/opr
/// for the operator workflow and https://pacvamp.com/project-status for limitations.
#[derive(usage_rs::Cli)]
#[usage(completion = true)]
#[usage(
    bin = "pacvamp-repo",
    version,
    author = "Jeff Dickey <@jdx>",
    arg_required_else_help
)]
struct Cli {
    #[usage(subcommand)]
    command: Option<Commands>,
}

/// Generate a self-contained shell completion script
#[derive(Debug, usage_rs::Args)]
struct Completion {
    /// Shell: bash, zsh, fish, or powershell
    #[usage(arg)]
    shell: String,
}

impl RunWith<()> for Completion {
    type Output = Result<()>;

    fn run_with(self, _: ()) -> Self::Output {
        let shell = usage_rs::complete::Shell::from_name(&self.shell)
            .ok_or_else(|| eyre::eyre!("unsupported shell: {}", self.shell))?;
        print!("{}", Cli::completion_script(shell));
        Ok(())
    }
}

#[derive(usage_rs::Subcommands)]
enum Commands {
    Completion(Completion),
    Advisories(advisories::Advisories),
    Attest(attest::Attest),
    Index(index::IndexCmd),
    Repack(repack::Repack),
    Sign(sign::Sign),
    Snapshot(snapshot::Snapshot),
    SyncAur(sync_aur::SyncAur),
    ToolChannel(tool_channel::ToolChannel),
    Vendor(vendor::Vendor),
    Verdict(verdict::VerdictCmd),
    Version(Version),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Vec<OsString> = std::env::args_os().collect();
    if let Some(answer) = Cli::completion_request(&args[1..]) {
        print!("{answer}");
        return Ok(());
    }
    pacvamp_cli_support::dump_usage_spec_if_requested(&args, || Cli::spec().to_kdl());
    let argv = pacvamp_cli_support::argv(&args);
    let cli = pacvamp_cli_support::unwrap_or_exit(Cli::spec(), &argv, Cli::parse_from_argv(&argv));
    match cli.command {
        Some(Commands::Completion(cmd)) => cmd.run_with(()),
        Some(Commands::Advisories(cmd)) => cmd.run_with(()),
        Some(Commands::Attest(cmd)) => cmd.run_with(()),
        Some(Commands::Index(cmd)) => cmd.run_with(()),
        Some(Commands::Repack(cmd)) => cmd.run_with(()),
        Some(Commands::Sign(cmd)) => cmd.run_with(()),
        Some(Commands::Snapshot(cmd)) => cmd.run_with(()),
        Some(Commands::SyncAur(cmd)) => cmd.run_with(()),
        Some(Commands::ToolChannel(cmd)) => cmd.run_with(()),
        Some(Commands::Vendor(cmd)) => cmd.run_with(()),
        Some(Commands::Verdict(cmd)) => cmd.run_with(()),
        Some(Commands::Version(cmd)) => cmd.run_with(BIN),
        None => Ok(()),
    }
}
