use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "fennel-cli")]
#[clap(about = "A tool for interacting with the Fennel Network", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encrypt { plaintext: String, identity: u32 },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decrypt { ciphertext: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Verify {
        message: String,
        signature: String,
        identity: u32,
    },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Sign { message: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    DecryptBacklog { identity: u32 },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    SendMessage {
        sender_id: u32,
        message: String,
        recipient_id: u32,
    },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    GetMessages { id: u32 },

    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    CreateIdentity { id: u32 },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    RetrieveIdentity { id: u32 },
}