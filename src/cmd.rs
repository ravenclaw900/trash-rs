#![allow(clippy::module_name_repetitions)]

use argh::FromArgs;

#[derive(FromArgs)]
/// Small trash program
pub struct Args {
    #[argh(subcommand)]
    pub cmd: Commands,
    /// recursively delete or move directories
    #[argh(switch, short = 'r')]
    pub recursive: bool,
}

#[derive(FromArgs)]
#[argh(subcommand)]
/// Subcommands
pub enum Commands {
    Init(InitSubCmd),
    List(ListSubCmd),
    Add(AddSubCmd),
    Restore(RestoreSubCmd),
    Delete(DeleteSubCmd),
    Empty(EmptySubCmd),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "init")]
/// Init trash
pub struct InitSubCmd {}

#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
/// List files in trash
pub struct ListSubCmd {}

#[derive(FromArgs)]
#[argh(subcommand, name = "add")]
/// Add file to trash
pub struct AddSubCmd {
    #[argh(positional)]
    pub path: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "restore")]
/// Restore file from trash
pub struct RestoreSubCmd {
    #[argh(positional)]
    pub name: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "delete")]
/// Delete file in trash
pub struct DeleteSubCmd {
    #[argh(positional)]
    pub name: String,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "empty")]
/// Empty files in trash
pub struct EmptySubCmd {}
