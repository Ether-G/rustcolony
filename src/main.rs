use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

mod entity;
mod position;
mod renderer;
mod simulation;

use simulation::Simulation;
use renderer::Renderer;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

/// Main application struct that manages the core systems
pub struct Application {
    window: Window,
    simulation: Simulation,
    renderer: Renderer,
    last_update: Instant,
}

impl Application {
    /// Create a new application instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window::new(
            "Rust Colony Simulation",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )?;
        
        window.limit_update_rate(Some(Duration::from_micros(16600)));

        let simulation = Simulation::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

        Ok(Application {
            window,
            simulation,
            renderer,
            last_update: Instant::now(),
        })
    }

    /// Main game loop
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting Colony Simulation");
        println!("Press ESC to exit");

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = Instant::now();
            let delta_time = now.duration_since(self.last_update).as_secs_f32();
            
            // Update simulation (mutable borrow)
            self.simulation.update(delta_time);
            
            // Render the world (immutable borrow of entities)
            self.renderer.clear();
            self.renderer.draw_world(self.simulation.get_entities());
            
            // Update window with new frame
            self.window
                .update_with_buffer(self.renderer.get_buffer(), WINDOW_WIDTH, WINDOW_HEIGHT)?;
            
            self.last_update = now;
            
            // Handle input
            self.handle_input();
        }

        println!("Simulation ended");
        Ok(())
    }

    /// Handle user input
    fn handle_input(&mut self) {
        // Add random resources on space key
        if self.window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            self.simulation.add_random_resources(5);
            println!("Added 5 new resources");
        }
        
        // Add gatherers on G key
        if self.window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            self.simulation.add_random_gatherers(3);
            println!("Added 3 new gatherers");
        }
        
        // Add predators on P key
        if self.window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            self.simulation.add_random_predators(1);
            println!("Added 1 new predator");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the application
    let mut app = Application::new()?;
    app.run()
} 