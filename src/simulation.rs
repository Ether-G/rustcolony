use crate::entity::{Entity, EntityId, EntityType};
use crate::position::Position;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Core simulation struct
pub struct Simulation {
    entities: Vec<Entity>,
    world_width: usize,
    world_height: usize,
    next_entity_id: EntityId,
    simulation_time: f32,
    rng: StdRng,
    spawn_timer: f32,
    interaction_cooldown: f32,
}

impl Simulation {
    /// Create a new simulation with initial entities
    pub fn new(world_width: usize, world_height: usize) -> Self {
        let mut simulation = Simulation {
            entities: Vec::new(),
            world_width,
            world_height,
            next_entity_id: 1,
            simulation_time: 0.0,
            rng: StdRng::from_entropy(),
            spawn_timer: 0.0,
            interaction_cooldown: 0.0,
        };

        simulation.initialize_world();
        simulation
    }

    /// Initialize the world with starting entities
    fn initialize_world(&mut self) {
        println!("Initializing world with starting entities...");
        
        for _ in 0..10 {
            self.add_random_gatherers(1);
        }
        
        for _ in 0..25 {
            self.add_random_resources(1);
        }
        
        for _ in 0..2 {
            self.add_random_predators(1);
        }
        
        println!("World initialized with {} entities", self.entities.len());
    }

    /// Main update loop
    pub fn update(&mut self, delta_time: f32) {
        self.simulation_time += delta_time;
        self.spawn_timer += delta_time;
        self.interaction_cooldown -= delta_time;

        for entity in &mut self.entities {
            entity.update(delta_time, self.world_width, self.world_height, &mut self.rng);
        }

        if self.interaction_cooldown <= 0.0 {
            self.handle_entity_interactions();
            self.interaction_cooldown = 0.05;
        }

        self.implement_smart_behaviors();

        self.remove_dead_entities();

        if self.spawn_timer > 5.0 {
            self.spawn_periodic_entities();
            self.spawn_timer = 0.0;
        }
    }

    /// Handle interactions between entities
    fn handle_entity_interactions(&mut self) {
        let mut interactions = Vec::new();
        
        for i in 0..self.entities.len() {
            for j in (i + 1)..self.entities.len() {
                let entity_a = &self.entities[i];
                let entity_b = &self.entities[j];
                
                if entity_a.can_interact_with(entity_b) {
                    interactions.push((i, j, entity_a.entity_type, entity_b.entity_type));
                }
            }
        }
        
        for (i, j, type_a, type_b) in interactions {
            match (type_a, type_b) {
                (EntityType::Gatherer, EntityType::Resource) => {
                    let (left, right) = self.entities.split_at_mut(j);
                    let gatherer = &mut left[i];
                    let resource = &mut right[0];
                    gatherer.consume_resource(resource);
                }
                (EntityType::Resource, EntityType::Gatherer) => {
                    let (left, right) = self.entities.split_at_mut(j);
                    let resource = &mut left[i];
                    let gatherer = &mut right[0];
                    gatherer.consume_resource(resource);
                }
                (EntityType::Predator, EntityType::Gatherer) => {
                    let (left, right) = self.entities.split_at_mut(j);
                    let predator = &mut left[i];
                    let gatherer = &mut right[0];
                    predator.hunt_gatherer(gatherer);
                }
                (EntityType::Gatherer, EntityType::Predator) => {
                    let (left, right) = self.entities.split_at_mut(j);
                    let gatherer = &mut left[i];
                    let predator = &mut right[0];
                    predator.hunt_gatherer(gatherer);
                }
                _ => {}
            }
        }
    }

    /// Implement smart behaviors for entities
    fn implement_smart_behaviors(&mut self) {
        let mut behavior_updates = Vec::new();
        
        for (index, entity) in self.entities.iter().enumerate() {
            match entity.entity_type {
                EntityType::Gatherer => {
                    if let Some(target) = entity.find_closest_entity(&self.entities, EntityType::Resource) {
                        let target_pos = target.position;
                        behavior_updates.push((index, target_pos));
                    }
                }
                EntityType::Predator => {
                    if let Some(target) = entity.find_closest_entity(&self.entities, EntityType::Gatherer) {
                        let target_pos = target.position;
                        behavior_updates.push((index, target_pos));
                    }
                }
                EntityType::Resource => {}
            }
        }
        
        for (index, target_pos) in behavior_updates {
            if let Some(entity) = self.entities.get_mut(index) {
                let move_probability = match entity.entity_type {
                    EntityType::Gatherer => {
                        let energy_ratio = entity.energy as f32 / entity.max_energy as f32;
                        if energy_ratio < 0.3 {
                            0.8
                        } else if energy_ratio < 0.6 {
                            0.4
                        } else {
                            0.2
                        }
                    }
                    EntityType::Predator => {
                        if entity.time_since_last_hunt > 15.0 {
                            0.9
                        } else if entity.time_since_last_hunt > 8.0 {
                            0.6
                        } else {
                            0.4
                        }
                    }
                    _ => 0.0,
                };
                
                if self.rng.gen_bool(move_probability) {
                    entity.position.move_towards(&target_pos, entity.speed);
                    entity.position.clamp_to_bounds(self.world_width, self.world_height);
                }
            }
        }
    }

    /// Remove dead entities
    fn remove_dead_entities(&mut self) {
        let initial_count = self.entities.len();
        self.entities.retain(|entity| !entity.is_dead());
        let removed_count = initial_count - self.entities.len();
        
        if removed_count > 0 {
            println!("Removed {} dead entities", removed_count);
        }
    }

    /// Spawn new entities periodically
    fn spawn_periodic_entities(&mut self) {
        if self.count_entities_of_type(EntityType::Resource) < 30 {
            self.add_random_resources(3);
            println!("Spawned 3 resources to maintain food supply");
        }
        
        if self.count_entities_of_type(EntityType::Gatherer) < 3 {
            self.add_random_gatherers(2);
            println!("Spawned 2 gatherers to maintain population");
        }
        
        if self.rng.gen_bool(0.15) && self.count_entities_of_type(EntityType::Predator) < 5 {
            self.add_random_predators(1);
            println!("Spawned predator - survival depends on hunting success");
        }
    }

    pub fn add_random_gatherers(&mut self, count: usize) {
        for _ in 0..count {
            let position = self.random_position();
            let entity = Entity::new_gatherer(self.next_entity_id, position);
            self.entities.push(entity);
            self.next_entity_id += 1;
        }
    }

    pub fn add_random_resources(&mut self, count: usize) {
        for _ in 0..count {
            let position = self.random_position();
            let entity = Entity::new_resource(self.next_entity_id, position);
            self.entities.push(entity);
            self.next_entity_id += 1;
        }
    }

    pub fn add_random_predators(&mut self, count: usize) {
        for _ in 0..count {
            let position = self.random_position();
            let entity = Entity::new_predator(self.next_entity_id, position);
            self.entities.push(entity);
            self.next_entity_id += 1;
        }
    }

    fn random_position(&mut self) -> Position {
        let x = self.rng.gen_range(10..(self.world_width as i32 - 10));
        let y = self.rng.gen_range(10..(self.world_height as i32 - 10));
        Position::new(x, y)
    }

    fn count_entities_of_type(&self, entity_type: EntityType) -> usize {
        self.entities.iter().filter(|e| e.entity_type == entity_type).count()
    }

    /// Get immutable reference to entities
    pub fn get_entities(&self) -> &[Entity] {
        &self.entities
    }
} 