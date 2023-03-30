use clap::{Args, Subcommand};

pub(crate) use create::CreateCommand;
pub(crate) use list::ListCommand;

use crate::CommandGlobalOpts;

mod create;
mod list;

/// Manage Forwarders
#[derive(Clone, Debug, Args)]
#[command(arg_required_else_help = true, subcommand_required = true)]
pub struct ForwarderCommand {
    #[command(subcommand)]
    subcommand: ForwarderSubCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ForwarderSubCommand {
    Create(CreateCommand),
    List(ListCommand),
}

impl ForwarderCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        match self.subcommand {
            ForwarderSubCommand::Create(c) => c.run(opts),
            ForwarderSubCommand::List(c) => c.run(opts),
        }
    }
}
