/// Represents a 2D position in the simulation world
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Create a new position
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    /// Calculate distance to another position
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate squared distance (faster for comparisons)
    pub fn distance_squared_to(&self, other: &Position) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Move towards another position by a given amount
    pub fn move_towards(&mut self, target: &Position, distance: f32) {
        let current_distance = self.distance_to(target);
        if current_distance > 0.0 {
            let ratio = distance / current_distance;
            let dx = ((target.x - self.x) as f32 * ratio) as i32;
            let dy = ((target.y - self.y) as f32 * ratio) as i32;
            self.x += dx;
            self.y += dy;
        }
    }

    /// Add a random offset to the position
    pub fn add_random_offset(&mut self, max_offset: i32, rng: &mut impl rand::Rng) {
        self.x += rng.gen_range(-max_offset..=max_offset);
        self.y += rng.gen_range(-max_offset..=max_offset);
    }

    /// Clamp position to stay within bounds
    pub fn clamp_to_bounds(&mut self, width: usize, height: usize) {
        self.x = self.x.max(0).min(width as i32 - 1);
        self.y = self.y.max(0).min(height as i32 - 1);
    }
} 