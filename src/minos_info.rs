use crate::Container;

#[derive(Debug)]
pub struct MinosInfo<'a> {
    pub container: &'a Container,
}

impl<'a> MinosInfo<'a> {
    fn paths(&self) -> Vec<&str> {
        self.container
            .paths()
            .iter()
            .filter_map(|p| p.to_str())
            .collect()
    }

    /// Print information about the container.
    pub fn print(&self) {
        println!("Minos Container info:");
        println!("id: {}", self.container.id());
        println!("description: {}", self.container.description());
        println!("paths: {:?}", self.paths());
        println!("engine_info: {:?}", self.container.engine().info());
    }
}
