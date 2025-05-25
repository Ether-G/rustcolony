use crate::entity::{Entity, EntityType};
use crate::position::Position;

/// Renderer manages the pixel buffer and handles drawing
pub struct Renderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    background_color: u32,
}

impl Renderer {
    /// Create a new renderer with the specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            buffer: vec![0; width * height],
            width,
            height,
            background_color: 0x000020,
        }
    }

    /// Get immutable reference to the buffer
    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }

    /// Clear the buffer to background color
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    /// Draw the entire world
    pub fn draw_world(&mut self, entities: &[Entity]) {
        self.draw_background();
        
        for entity in entities {
            self.draw_entity(entity);
        }
        
        self.draw_ui_info(entities);
    }

    /// Draw a subtle background pattern
    fn draw_background(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                
                if x % 50 == 0 || y % 50 == 0 {
                    self.buffer[index] = 0x001040;
                } else {
                    self.buffer[index] = self.background_color;
                }
            }
        }
    }

    /// Draw a single entity
    fn draw_entity(&mut self, entity: &Entity) {
        let size = entity.size as i32;
        let half_size = size / 2;
        
        for dy in -half_size..=half_size {
            for dx in -half_size..=half_size {
                let x = entity.position.x + dx;
                let y = entity.position.y + dy;
                
                if dx * dx + dy * dy <= half_size * half_size {
                    self.set_pixel(Position::new(x, y), entity.color);
                }
            }
        }
        
        match entity.entity_type {
            EntityType::Gatherer => self.draw_gatherer_decoration(entity),
            EntityType::Resource => self.draw_resource_decoration(entity),
            EntityType::Predator => self.draw_predator_decoration(entity),
        }
    }

    /// Draw gatherer-specific decoration (energy indicator)
    fn draw_gatherer_decoration(&mut self, entity: &Entity) {
        let energy_ratio = entity.energy as f32 / entity.max_energy as f32;
        let bar_width = 8;
        let bar_height = 2;
        let bar_y = entity.position.y - entity.size as i32 - 3;
        
        for x in 0..bar_width {
            for y in 0..bar_height {
                let pos = Position::new(
                    entity.position.x - bar_width / 2 + x,
                    bar_y + y
                );
                self.set_pixel(pos, 0x404040);
            }
        }
        
        let fill_width = (bar_width as f32 * energy_ratio) as i32;
        for x in 0..fill_width {
            for y in 0..bar_height {
                let pos = Position::new(
                    entity.position.x - bar_width / 2 + x,
                    bar_y + y
                );
                let color = if energy_ratio > 0.5 {
                    0x00FF00
                } else if energy_ratio > 0.25 {
                    0xFFFF00
                } else {
                    0xFF0000
                };
                self.set_pixel(pos, color);
            }
        }
    }

    /// Draw resource-specific decoration (pulsing effect)
    fn draw_resource_decoration(&mut self, entity: &Entity) {
        let energy_ratio = entity.energy as f32 / entity.max_energy as f32;
        
        if energy_ratio > 0.8 {
            let ring_radius = entity.size as i32 + 2;
            for angle in 0..16 {
                let radians = (angle as f32) * std::f32::consts::PI * 2.0 / 16.0;
                let x = entity.position.x + (ring_radius as f32 * radians.cos()) as i32;
                let y = entity.position.y + (ring_radius as f32 * radians.sin()) as i32;
                self.set_pixel(Position::new(x, y), 0xFFFFAA);
            }
        }
    }

    /// Draw predator-specific decoration (hunting indicator)
    fn draw_predator_decoration(&mut self, entity: &Entity) {
        let spike_length = entity.size as i32 + 1;
        for angle in 0..8 {
            let radians = (angle as f32) * std::f32::consts::PI * 2.0 / 8.0;
            let x = entity.position.x + (spike_length as f32 * radians.cos()) as i32;
            let y = entity.position.y + (spike_length as f32 * radians.sin()) as i32;
            self.set_pixel(Position::new(x, y), 0xFF4444);
        }
    }

    /// Draw UI information overlay
    fn draw_ui_info(&mut self, entities: &[Entity]) {
        let gatherer_count = entities.iter().filter(|e| e.entity_type == EntityType::Gatherer).count();
        let resource_count = entities.iter().filter(|e| e.entity_type == EntityType::Resource).count();
        let predator_count = entities.iter().filter(|e| e.entity_type == EntityType::Predator).count();
        
        for i in 0..gatherer_count.min(20) {
            let pos = Position::new(10 + (i * 3) as i32, 10);
            self.set_pixel(pos, 0x00FF00);
        }
        
        for i in 0..resource_count.min(20) {
            let pos = Position::new(10 + (i * 3) as i32, 15);
            self.set_pixel(pos, 0xFFFF00);
        }
        
        for i in 0..predator_count.min(20) {
            let pos = Position::new(10 + (i * 3) as i32, 20);
            self.set_pixel(pos, 0xFF0000);
        }
    }

    /// Set a pixel in the buffer - handles bounds checking
    fn set_pixel(&mut self, position: Position, color: u32) {
        if position.x >= 0 && position.x < self.width as i32 
            && position.y >= 0 && position.y < self.height as i32 {
            let index = (position.y as usize) * self.width + (position.x as usize);
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    /// Draw a line between two points (for debugging/visualization)
    #[allow(dead_code)]
    fn draw_line(&mut self, start: Position, end: Position, color: u32) {
        let dx = (end.x - start.x).abs();
        let dy = (end.y - start.y).abs();
        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = start.x;
        let mut y = start.y;
        
        loop {
            self.set_pixel(Position::new(x, y), color);
            
            if x == end.x && y == end.y {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
} 