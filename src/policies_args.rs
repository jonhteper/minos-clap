use clap::Parser;
use minos::engine::Criteria;

use crate::Container;

#[derive(Debug, Parser)]
pub struct PoliciesArgs {
    /// Set resource's type.
    #[clap(long = "type")]
    resource_type: Option<String>,

    /// Set attributed resource's id.
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

            let policies = info.policies(Criteria::ResourceId(resource_id));
            for policy in policies {
                println!("{:?}", policy);
            }

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

            let policies = info.policies(Criteria::ResourceType(resource_type));
            for policy in policies {
                println!("{:?}", policy);
            }

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
