use rand::{random, thread_rng, Rng};
use vector2::Vector2;
use std::thread;

pub struct Actor {
	// Name should be unique
	name: String,
	position: Vector2,
}

impl Actor {
	pub fn new(name: String) -> Actor {
		let mut rng = thread_rng();
		Actor {
			name: name,
			position: Vector2 {x: rng.gen_range(-32, 33), y: rng.gen_range(-32, 33)}
		}
	}
	pub fn get_name(&self) -> String {
		return self.name.clone();
	}
	pub fn get_position(&self) -> Vector2 {
		return self.position.clone();
	}
	pub fn think(&mut self, actors: &Vec<Actor>) {
		//println!("{} is thinking", self.name);
		thread::sleep_ms(25);

		// 1. Goal
		// Be close to other Actors
		// Be within 2 units away
		// Am I within 2 units of another Actor?
			// yes, no need to move
			// no, move towards closest Actor

		// 2. Observe
		// Locate nearest Actor
		let nearest_actor_position = self.get_nearest_actor_position(&actors);
		// println!("The nearest Actor to {} is at ({}, {})",
		// 	self.name,
		// 	nearest_actor_position.x,
		// 	nearest_actor_position.y);

		// 3. Act
		// Move towards nearest Actor
		{
			let mut position_other = self.position.clone();

			// Randomly decide which axis to move on to make more natural
			if random() {
				if nearest_actor_position.x > self.position.x {
				    position_other.x += 1;
				    if !Actor::check_if_occupied(position_other, &actors) {
				    	self.move_right();
				    }
				} else if nearest_actor_position.x < self.position.x {
				    position_other.x -= 1;
				    if !Actor::check_if_occupied(position_other, &actors) {
				    	self.move_left();
				    }
				}
			} else {
				if nearest_actor_position.y > self.position.y {
				    position_other.y += 1;
				    if !Actor::check_if_occupied(position_other, &actors) {
				    	self.move_up();
				    }
				} else if nearest_actor_position.y < self.position.y {
				    position_other.y -= 1;
				    if !Actor::check_if_occupied(position_other, &actors) {
				    	self.move_down();
				    }
				}
			}
		}

		// TODO should two Actors be able to occupy same space?
	}
	pub fn say_position(&self) {
		println!("{}: My position is ({}, {})", self.name, self.position.x, self.position.y);
	}
	fn check_if_occupied(position: Vector2, actors: &Vec<Actor>) -> bool {
	    for actor in actors {
	        if actor.position == position {
	            return true;
	        }
	    }
	    false
	}
	fn set_position(&mut self, x: i32, y: i32) {
		self.position.x = x;
		self.position.y = y;
	}
	fn move_up(&mut self) {
		let current_y = self.position.y;
		self.position.y = current_y + 1;
		println!("\t{} moved up", self.name);
	}
	fn move_down(&mut self) {
		let current_y = self.position.y;
		self.position.y = current_y - 1;
		println!("\t{} moved down", self.name);
	}
	fn move_left(&mut self) {
		let current_x = self.position.x;
		self.position.x = current_x - 1;
		println!("\t{} moved left", self.name);
	}
	fn move_right(&mut self) {
		let current_x = self.position.x;
		self.position.x = current_x + 1;
		println!("\t{} moved right", self.name);
	}
	fn move_direction(&mut self, direction: Direction) {
		match direction {
		    Direction::Up => self.move_up(),
		    Direction::Down => self.move_down(),
		    Direction::Left => self.move_left(),
		    Direction::Right => self.move_right(),
		}
	}
	fn get_nearest_actor_position(&self, actors: &Vec<Actor>) -> Vector2 {
		// Get a new copy of the Vec without ourselves in it
		let other_actors: Vec<Actor> = self.get_other_actors(&actors);

		let mut closest_position: Vector2 = other_actors[0].position.clone();
		let mut best_distance: f64 = other_actors[0].position.distance_from(&self.position);

		for actor in other_actors {
			let distance: f64 = actor.position.distance_from(&self.position);
			if distance < best_distance {
			    best_distance = distance;
			    closest_position.clone_from(&actor.position);
			}
		}

		closest_position
	}
	fn move_random(&mut self) {
		if random() {
		    if random() {
			    self.move_up();
			} else {
			    self.move_down();
			}
		} else {
		    if random() {
			    self.move_left();
			} else {
			    self.move_right();
			}
		}
	}
	fn get_other_actors(&self, actors: &Vec<Actor>) -> Vec<Actor> {
		// Get a new copy of the Vec without ourselves in it
		let mut other_actors: Vec<Actor> = Vec::new();
		for actor in actors {
		    if self.name != actor.name {
		        other_actors.push(actor.clone());
		    }
		}
		other_actors
	}
}

impl Clone for Actor {
    fn clone(&self) -> Self {
    	Actor {
    		name: self.name.clone(),
    		position: self.position.clone(),
    	}
    }
    fn clone_from(&mut self, source: &Self) {
    	self.name.clone_from(&source.name);
    	self.position.clone_from(&source.position);
    }
}

enum Direction {
	Left,
	Up,
	Right,
	Down
}
