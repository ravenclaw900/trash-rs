#![allow(clippy::module_name_repetitions)]

use argh::FromArgs;
use std::cmp::PartialEq;

#[derive(FromArgs)]
/// Small trash program
pub struct Args {
    #[argh(subcommand)]
    pub cmd: Commands,
    /// recursively delete or move directories
    #[argh(switch, short = 'r')]
    pub recursive: bool,
}

#[derive(FromArgs, PartialEq)]
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

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "init")]
/// Init trash
pub struct InitSubCmd {}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "list")]
/// List files in trash
pub struct ListSubCmd {}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "add")]
/// Add file to trash
pub struct AddSubCmd {
    #[argh(positional)]
    pub path: String,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "restore")]
/// Restore file from trash
pub struct RestoreSubCmd {
    #[argh(positional)]
    pub name: String,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "delete")]
/// Delete file in trash
pub struct DeleteSubCmd {
    #[argh(positional)]
    pub name: String,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "empty")]
/// Empty files in trash
pub struct EmptySubCmd {}
