use clap::Parser;
use minos::engine::StaticContainer;
use minos_info::MinosInfo;
use policies_args::PoliciesArgs;
use resource_args::ResourcesArgs;

pub mod minos_info;
pub mod policies_args;
pub mod resource_args;

pub(crate) type Container = minos::Container<StaticContainer>;

#[derive(Debug, Parser)]
pub struct MinosCommand {
    #[clap(subcommand)]
    action: MinosAction,
}

impl MinosCommand {
    pub fn run(&self, container: &Container) {
        self.action.run(container)
    }
}

#[derive(Debug, Parser)]
pub enum MinosAction {
    /// List authorization data. Use --help for more information
    Ls(ListableSubCommand),

    /// Show information about the container
    Info,
}

impl MinosAction {
    fn run_ls(&self, container: &Container, listable: &Listable) {
        match listable {
            Listable::Resources(args) => args.run(container),
            Listable::Policies(args) => args.run(container),
        }
    }

    fn run_info(&self, container: &Container) {
        let info = MinosInfo { container };
        info.print()
    }

    pub fn run(&self, container: &Container) {
        match self {
            Self::Ls(cmd) => self.run_ls(container, &cmd.listable),
            Self::Info => self.run_info(container),
        }
    }
}

#[derive(Debug, Parser)]
pub struct ListableSubCommand {
    #[clap(subcommand)]
    listable: Listable,
}

#[derive(Debug, Parser)]
pub enum Listable {
    /// Resources names
    Resources(ResourcesArgs),

    /// Authorization policies
    Policies(PoliciesArgs),
}
