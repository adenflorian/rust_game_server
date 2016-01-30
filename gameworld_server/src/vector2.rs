pub struct Vector2 {
	pub x: i32,
	pub y: i32,
}

impl Vector2 {
	pub fn new(x: i32, y: i32) -> Vector2 {
		Vector2 {
			x: x,
			y: y
		}
	}
	pub fn distance_from(&self, other_vector2: &Vector2) -> f64 {
		let mut a = self.x - other_vector2.x;
		a *= a;
		let mut b = self.y - other_vector2.y;
		b *= b;
		let c = a + b;
		let distance = (c as f64).sqrt();
		distance
	}
}

impl Clone for Vector2 {
	fn clone(&self) -> Self {
		Vector2 {
			x: self.x.clone(),
			y: self.y.clone(),
		}
	}
    fn clone_from(&mut self, source: &Self) {
    	self.x = source.x.clone();
    	self.y = source.y.clone();
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
    	self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Vector2) -> bool {
    	self.x != other.x || self.y == other.y
    }
}

impl Eq for Vector2 {}
