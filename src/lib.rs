use serde::{Deserialize, Serialize};
mod renderer;
use renderer::*;
struct Environment {}

impl Environment {
    fn new(cfg: &HeliosConfiguration) -> Self  {
        todo!()
    }
}
#[derive(Deserialize, Serialize, Clone)]
pub struct HeliosConfiguration {
}

// Helios entry. Manages environment simulation.
pub struct Helios {
    environment: Environment,
    renderer: Renderer,
}

impl Helios {
    pub fn new(ctx: &mut dashi::Context, cfg: HeliosConfiguration) -> Self {
        Helios {
            environment: Environment::new(&cfg),
            renderer: Renderer::new(ctx, &cfg),
        }
    }

    pub fn new_from_file(ctx: &mut dashi::Context, path: &str) -> Self {
        todo!()
    }
}
