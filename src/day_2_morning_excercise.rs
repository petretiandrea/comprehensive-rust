use std::fmt;
use std::ops::Add;

pub fn day_2_run_morning_exercise() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
    bob.set_weight(30.2);
    bob.set_age(20);
    println!("Bob weight {} and bob age is {}", bob.weight(), bob.age());
}

struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        return User { name, age, weight };
    }
    pub fn name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn age(&self) -> u32 {
        return self.age;
    }

    pub fn weight(&self) -> f32 {
        return self.weight;
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight
    }
}

// polygon exercise
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    // add fields
    x: i32,
    y: i32,
}

impl Point {
    // add methods
    pub fn new(x: i32, y: i32) -> Self {
        return Point { x, y };
    }

    pub fn dist(self, p2: Point) -> f64 {
        let sum = (p2.x - self.x).pow(2) + (p2.y - self.y).pow(2);
        return (sum as f64).sqrt().abs();
    }

    pub fn magnitude(self) -> f64 {
        return self.dist(Point::new(0, 0));
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        return Point::new(self.x + rhs.x, self.y + rhs.y);
    }
}

#[derive(PartialEq, Debug)]
pub struct Polygon {
    // add fields
    points: Vec<Point>,
}

impl Polygon {
    // add methods
    pub fn new() -> Self {
        return Polygon { points: Vec::new() };
    }
    pub fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    pub fn left_most_point<>(&self) -> Option<Point> {
        return self.points.iter().min_by_key(|point| point.x).copied();
    }

    pub fn iter(&self) -> impl Iterator<Item=&Point> {
        self.points.iter()
    }

    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut result = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            result += last_point.dist(*point);
            last_point = *point;
        }
        result += last_point.dist(self.points[0]);
        result
    }
}

pub struct Circle {
    // add fields
    center: Point,
    radius: i32
}

impl Circle {
    // add methods
    pub fn new(center: Point, radius: i32) -> Self {
        return Circle { center, radius };
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.radius)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}