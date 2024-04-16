use clap::Parser;
use minos::engine::EngineInfo;

use crate::Container;

#[derive(Debug, Parser)]
pub struct ResourcesArgs {
    /// Print only the selection length.
    #[clap(long)]
    len: bool,

    /// Print only the attributed resources. Actived by default.
    #[clap(long = "attr")]
    attributed: bool,

    /// Print resources and attributed resources.
    #[clap(long = "incl-attr")]
    include_attributed: bool,
}

impl ResourcesArgs {
    fn run_attributed_selected(&self, info: &EngineInfo) {
        if self.len {
            println!("{}", info.attr_resources_len());
            return;
        }

        Self::print_attr_resources(info);
    }

    fn run_include_attributed_selected(&self, info: &EngineInfo) {
        if self.len {
            println!("{}", info.resources_len() + info.attr_resources_len());
            return;
        }

        Self::print_resources(info);
        Self::print_attr_resources(info);
    }

    fn print_attr_resources(info: &EngineInfo) {
        let names = info.attr_resources_names();
        for (ty, id) in names {
            println!("type: {} | id: {}", ty.0, id);
        }
    }

    fn print_resources(info: &EngineInfo) {
        let names = info.resources_names();
        for name in names {
            println!("{}", name.0);
        }
    }

    pub fn run(&self, container: &Container) {
        let engine = container.engine();
        let info = engine.info();

        if self.attributed {
            self.run_attributed_selected(&info);
            return;
        }

        if self.include_attributed {
            self.run_include_attributed_selected(&info);
            return;
        }

        if self.len {
            println!("{}", info.resources_len());
            return;
        }

        Self::print_resources(&info);
    }
}
