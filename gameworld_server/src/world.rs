use actor::Actor;

pub struct World {
	inhabitants: Vec<Actor>,
}

impl World {
    pub fn new() -> World {
    	World {
    		inhabitants: Vec::new(),
    	}
    }
}
