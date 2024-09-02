use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[command(about = "Encrypt a text message")]
    Encrypt(EncryptArgs),
    #[command(about = "Decrypt a text message")]
    Decrypt(DecryptArgs),
}

#[derive(Debug, Parser)]
pub struct EncryptArgs {
    #[arg(long)]
    pub(crate) key: String,
    #[arg(long)]
    pub(crate) text: String,
}

#[derive(Debug, Parser)]
pub struct DecryptArgs {
    #[arg(long)]
    pub(crate) key: String,
    #[arg(long)]
    pub(crate) text: String,
}
