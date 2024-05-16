use clap::Parser;
use minos::{
    engine::Criteria,
    text_repr::{environment_text_repr::EnvironmentsFormatter, to_text_repr::ToTextRepr},
};

use crate::Container;

#[derive(Debug, Parser)]
pub struct PoliciesArgs {
    /// Set resource's type to show policies by Environment.
    #[clap(long = "type")]
    resource_type: Option<String>,

    /// Set attributed resource's id to show policies by Environment.
    #[clap(long = "id")]
    resource_id: Option<String>,

    /// Print only the selection length.
    #[clap(long)]
    len: bool,
}

impl PoliciesArgs {
    pub fn run(&self, container: &Container) {
        let engine = container.engine();
        let info = engine.info();

        if let Some(resource_id) = &self.resource_id {
            if self.len {
                println!(
                    "{}",
                    info.policies_len(Some(Criteria::ResourceId(resource_id)))
                );
                return;
            }

            let maybe_envs = info.environments(Criteria::ResourceId(resource_id));
            if maybe_envs.is_none() {
                return;
            }

            let envs = maybe_envs.unwrap();
            let policies_text_repr = EnvironmentsFormatter::new(envs).to_text_repr();
            println!("{}", policies_text_repr);

            return;
        }

        if let Some(resource_type) = &self.resource_type {
            if self.len {
                println!(
                    "{}",
                    info.policies_len(Some(Criteria::ResourceType(resource_type)))
                );
                return;
            }

            let maybe_envs = info.environments(Criteria::ResourceType(resource_type));
            if maybe_envs.is_none() {
                return;
            }
            let envs = maybe_envs.unwrap();

            let policies_text_repr = EnvironmentsFormatter::new(envs).to_text_repr();
            println!("{}", policies_text_repr);

            return;
        }

        // Print the number of resources policies + attributed resources policies.
        if self.len {
            println!("{}", info.policies_len(None));
            return;
        }

        println!("pass values by flags. Use --help for more information");
    }
}
