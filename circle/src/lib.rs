#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center : Point,
	pub radius : f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius : f64) -> Self {
        Circle {
            center: Point (x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn intersect(&self: &Circle, other: Circle) -> bool {
        return self.center.distance(other.center) <= (self.radius + other.radius);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point (pub f64, pub f64);

impl Point {
    pub fn distance(&self: &Point, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}