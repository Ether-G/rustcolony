use crate::position::Position;
use rand::Rng;

/// Unique identifier for entities
pub type EntityId = u64;

/// Different types of entities in the simulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Gatherer,
    Resource,
    Predator,
}

/// Core entity structure
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub position: Position,
    pub energy: u32,
    pub entity_type: EntityType,
    pub color: u32,
    pub max_energy: u32,
    pub speed: f32,
    pub size: u32,
    pub age: f32,
    pub energy_consumption_timer: f32,
    pub time_since_last_hunt: f32,
}

impl Entity {
    /// Create a new gatherer entity
    pub fn new_gatherer(id: EntityId, position: Position) -> Self {
        Entity {
            id,
            position,
            energy: 150,
            entity_type: EntityType::Gatherer,
            color: 0x00FF00,
            max_energy: 200,
            speed: 2.0,
            size: 3,
            age: 0.0,
            energy_consumption_timer: 0.0,
            time_since_last_hunt: 0.0,
        }
    }

    /// Create a new resource entity
    pub fn new_resource(id: EntityId, position: Position) -> Self {
        Entity {
            id,
            position,
            energy: 80,
            entity_type: EntityType::Resource,
            color: 0xFFFF00,
            max_energy: 80,
            speed: 0.0,
            size: 2,
            age: 0.0,
            energy_consumption_timer: 0.0,
            time_since_last_hunt: 0.0,
        }
    }

    /// Create a new predator entity
    pub fn new_predator(id: EntityId, position: Position) -> Self {
        Entity {
            id,
            position,
            energy: 150,
            entity_type: EntityType::Predator,
            color: 0xFF0000,
            max_energy: 220,
            speed: 4.5,
            size: 4,
            age: 0.0,
            energy_consumption_timer: 0.0,
            time_since_last_hunt: 0.0,
        }
    }

    /// Update entity behavior
    pub fn update(&mut self, delta_time: f32, world_width: usize, world_height: usize, rng: &mut impl Rng) {
        self.age += delta_time;
        self.energy_consumption_timer += delta_time;
        
        if self.entity_type == EntityType::Predator {
            self.time_since_last_hunt += delta_time;
        }
        
        match self.entity_type {
            EntityType::Gatherer => self.update_gatherer(delta_time, world_width, world_height, rng),
            EntityType::Resource => self.update_resource(delta_time),
            EntityType::Predator => self.update_predator(delta_time, world_width, world_height, rng),
        }
    }

    /// Gatherer behavior: random movement, energy consumption
    fn update_gatherer(&mut self, _delta_time: f32, world_width: usize, world_height: usize, rng: &mut impl Rng) {
        if self.energy_consumption_timer >= 2.0 {
            if self.energy > 0 {
                self.energy = self.energy.saturating_sub(1);
            }
            self.energy_consumption_timer = 0.0;
        }

        if rng.gen_bool(0.6) {
            self.position.add_random_offset(self.speed as i32, rng);
            self.position.clamp_to_bounds(world_width, world_height);
        }

        let energy_ratio = self.energy as f32 / self.max_energy as f32;
        let green_intensity = (255.0 * energy_ratio) as u32;
        self.color = green_intensity << 8;
    }

    /// Resource behavior: static, slowly regenerates
    fn update_resource(&mut self, _delta_time: f32) {
        if self.energy_consumption_timer >= 1.0 {
            if self.energy < self.max_energy {
                self.energy = (self.energy + 2).min(self.max_energy);
            }
            self.energy_consumption_timer = 0.0;
        }

        let energy_ratio = self.energy as f32 / self.max_energy as f32;
        let intensity = (255.0 * energy_ratio) as u32;
        self.color = (intensity << 16) | (intensity << 8);
    }

    /// Predator behavior: hunt gatherers, more complex movement
    fn update_predator(&mut self, _delta_time: f32, world_width: usize, world_height: usize, rng: &mut impl Rng) {
        if self.energy_consumption_timer >= 3.0 {
            let mut energy_loss = 1;
            
            if self.time_since_last_hunt > 25.0 {
                energy_loss = 3;
                println!("Predator {} is starving (no hunt for {:.1}s)", self.id, self.time_since_last_hunt);
            } else if self.time_since_last_hunt > 18.0 {
                energy_loss = 2;
            }
            
            if self.energy > 0 {
                self.energy = self.energy.saturating_sub(energy_loss);
            }
            self.energy_consumption_timer = 0.0;
        }
        
        if self.age > 180.0 {
            self.energy = 0;
            println!("Predator {} died of old age at {:.1}s", self.id, self.age);
            return;
        }

        if rng.gen_bool(0.7) {
            self.position.add_random_offset((self.speed * 1.2) as i32, rng);
            self.position.clamp_to_bounds(world_width, world_height);
        }

        let energy_ratio = self.energy as f32 / self.max_energy as f32;
        let mut red_intensity = (255.0 * energy_ratio) as u32;
        
        if self.time_since_last_hunt > 18.0 {
            red_intensity = red_intensity / 2;
        }
        
        self.color = red_intensity << 16;
    }

    /// Check if entity is dead (no energy)
    pub fn is_dead(&self) -> bool {
        self.energy == 0
    }

    /// Check if entity can interact with another entity
    pub fn can_interact_with(&self, other: &Entity) -> bool {
        let distance = self.position.distance_squared_to(&other.position);
        let interaction_range = ((self.size + other.size) * 3) as i32;
        distance <= interaction_range * interaction_range
    }

    /// Gatherer consumes a resource
    pub fn consume_resource(&mut self, resource: &mut Entity) -> bool {
        if self.entity_type == EntityType::Gatherer 
            && resource.entity_type == EntityType::Resource 
            && self.can_interact_with(resource)
            && resource.energy > 0 {
            
            let energy_transfer = resource.energy.min(30);
            resource.energy -= energy_transfer;
            self.energy = (self.energy + energy_transfer).min(self.max_energy);
            
            println!("Gatherer {} consumed {} energy from resource {} (now has {} energy)", 
                     self.id, energy_transfer, resource.id, self.energy);
            true
        } else {
            false
        }
    }

    /// Predator hunts a gatherer
    pub fn hunt_gatherer(&mut self, gatherer: &mut Entity) -> bool {
        if self.entity_type == EntityType::Predator 
            && gatherer.entity_type == EntityType::Gatherer 
            && self.can_interact_with(gatherer) {
            
            let energy_stolen = gatherer.energy.min(40);
            gatherer.energy = gatherer.energy.saturating_sub(energy_stolen);
            self.energy = (self.energy + energy_stolen / 2).min(self.max_energy);
            
            self.time_since_last_hunt = 0.0;
            
            println!("Predator {} hunted gatherer {} for {} energy", 
                     self.id, gatherer.id, energy_stolen);
            true
        } else {
            false
        }
    }

    /// Find the closest entity of a specific type
    pub fn find_closest_entity<'a>(
        &self, 
        entities: &'a [Entity], 
        target_type: EntityType
    ) -> Option<&'a Entity> {
        entities
            .iter()
            .filter(|e| e.entity_type == target_type && e.id != self.id)
            .min_by_key(|e| self.position.distance_squared_to(&e.position))
    }
} 