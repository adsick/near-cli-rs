use std::path::PathBuf;
use clap::Clap;

use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockHeight};

use near_cli_visual::{PromptInput, prompt_variant};
use near_cli_derive as cli;
#[derive(Debug, Default, Clone, Clap, cli::Interactive)]
pub struct CliBlockIdHeight {
    block_id_height: Option<BlockHeight>,
}

#[derive(Debug, Default, Clone, Clap, cli::Interactive)]
pub struct CliBlockIdHash {
    block_id_hash: Option<CryptoHash>,
}

#[derive(Debug, Clone, EnumDiscriminants, Clap, cli::Interactive)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum CliBlockId {
    /// Specify a block ID final to view this account
    #[strum_discriminants(strum(message = "At final block"))]
    AtFinalBlock,
    /// Specify a block ID height to view this account
    #[strum_discriminants(strum(message = "At block height"))]
    AtBlockHeight(CliBlockIdHeight),
    /// Specify a block ID hash to view this account
    #[strum_discriminants(strum(message = "At block hash"))]
    AtBlockHash(CliBlockIdHash),
}

impl PromptInput for CliBlockId {
    fn prompt_input() -> Self {
        match prompt_variant::<CliBlockIdDiscriminants>("Choose your action") {
            CliBlockIdDiscriminants::AtFinalBlock => CliBlockId::AtFinalBlock,
            CliBlockIdDiscriminants::AtBlockHeight => CliBlockId::AtBlockHeight(Default::default()),
            CliBlockIdDiscriminants::AtBlockHash => CliBlockId::AtBlockHash(Default::default()),
        }
    }
}

#[derive(Debug, Default, Clone, Clap, cli::Interactive)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliContractHash {
    #[clap(subcommand)]
    selected_block_id: Option<CliBlockId>,
}

#[derive(Debug, Default, Clone, Clap, cli::Interactive)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliContractFile {
    // file_path: Option<PathBuf>,

    #[clap(subcommand)]
    selected_block_id: Option<CliBlockId>,
}

#[derive(Debug, Clone, Clap, cli::Interactive, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum CliDownloadMode {
    /// Download a contract file
    #[strum_discriminants(strum(message = "Download a contract file"))]
    Download(CliContractFile),
    /// View a contract hash
    #[strum_discriminants(strum(message = "View a contract hash"))]
    Hash(CliContractHash),
}

impl PromptInput for CliDownloadMode {
    fn prompt_input() -> Self {
        match prompt_variant("Choose your action") {
            CliDownloadModeDiscriminants::Download => CliDownloadMode::Download(Default::default()),
            CliDownloadModeDiscriminants::Hash => CliDownloadMode::Hash(Default::default()),
        }
    }
}

#[derive(Debug, Default, Clone, Clap, cli::Interactive)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliContract {
    pub contract_id: Option<AccountId>,
    #[clap(subcommand)]
    download_mode: Option<CliDownloadMode>,
}

#[derive(Debug, Clone, Clap, cli::Interactive)]
pub enum CliSendTo<T> where T: Default {
    /// Specify a contract
    SendTo(T),
}

impl<T> PromptInput for CliSendTo<T> where T: Default {
    fn prompt_input() -> Self {
        Self::SendTo(Default::default())
    }
}

#[derive(Debug, Clone, Clap, cli::Interactive)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliServer<T> {
    #[clap(subcommand)]
    pub send_to: Option<T>,
}

// Needed to ignore <T: Default> trait bound requirement
impl<T> Default for CliServer<T> {
    fn default() -> Self {
        Self { send_to: Default::default() }
    }
}
#[derive(Debug, Clone, Clap, cli::Interactive)]
#[clap(
    version,
    author,
    about,
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliCustomServer<T> {
    // #[clap(long)]
    // pub url: Option<AvailableRpcServerUrl>,
    #[clap(subcommand)]
    send_to: Option<T>,
}

// Needed to ignore <T: Default> trait bound requirement
impl<T> Default for CliCustomServer<T> {
    fn default() -> Self {
        Self { send_to: Default::default() }
    }
}

#[derive(Debug, Clone, Clap, EnumDiscriminants, cli::Interactive)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum CliSelectServer<T> where T: clap::Args + Clone + Default {
    /// предоставление данных для сервера https://rpc.testnet.near.org
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(CliServer<CliSendTo<T>>),
    /// предоставление данных для сервера https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(CliServer<CliSendTo<T>>),
    /// предоставление данных для сервера https://rpc.betanet.near.org
    #[strum_discriminants(strum(message = "Betanet"))]
    Betanet(CliServer<CliSendTo<T>>),
    /// предоставление данных для сервера, указанного вручную
    #[strum_discriminants(strum(message = "Custom"))]
    Custom(CliCustomServer<CliSendTo<T>>),
}

impl<T> PromptInput for CliSelectServer<T> where T: clap::Args + Clone + Default {
    fn prompt_input() -> Self {
        match prompt_variant("") {
            CliSelectServerDiscriminants::Testnet => Self::Testnet(Default::default()),
            CliSelectServerDiscriminants::Mainnet => Self::Mainnet(Default::default()),
            CliSelectServerDiscriminants::Betanet => Self::Betanet(Default::default()),
            CliSelectServerDiscriminants::Custom => Self::Custom(Default::default()),
        }
    }
}

#[derive(Debug, Clone, Clap, cli::Interactive)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliNetworkArgs<T> {
    #[clap(subcommand)]
    selected_server: Option<T>,
}

impl<T> Default for CliNetworkArgs<T> {
    fn default() -> Self {
        Self { selected_server: Default::default() }
    }
}

#[derive(Debug, Clone, Clap, cli::Interactive)]
pub enum CliMode<T> {
    /// Execute a change method with online mode
    Network(T),
}

impl<T> PromptInput for CliMode<T> where T: Default {
    fn prompt_input() -> Self {
        Self::Network(T::default())
    }
}

#[derive(Debug, Default, Clone, Clap, cli::Interactive)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::DisableVersionForSubcommands)
)]
pub struct CliOperationMode<T> {
    #[clap(subcommand)]
    mode: Option<T>,
}

#[derive(Debug, Clone, EnumDiscriminants, Clap, cli::Interactive)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum CliQueryRequest {
    /// View properties for an account
    #[strum_discriminants(strum(message = "View properties for an account"))]
    AccountSummary(
        CliOperationMode<CliMode<CliNetworkArgs<CliSelectServer<CliContract>>>>
    )
}
