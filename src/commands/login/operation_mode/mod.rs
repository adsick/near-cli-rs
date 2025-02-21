pub mod online_mode;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct OperationMode {
    #[interactive_clap(named_arg)]
    ///Execute a change method with online mode
    pub network: self::online_mode::NetworkArgs,
}

impl OperationMode {
    pub async fn process(self) -> crate::CliResult {
        self.network.process().await
    }
}
