const PI:f64 = 3.1415926;


struct Rectangle {
    a: f64,
    b: f64,
}   // end of struct Rectangle


struct Circle {
    r: f64,
}   // end of struct Circle

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}   // end of struct Triangle


enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}   // end of enum Shape


impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle(ref r) => r.area(),
            Shape::Circle(ref c) => c.area(),
            Shape::Triangle(ref t) => t.area(),
        }
    }   // end of fn area
}   // end of impl Shape



trait Area {
    fn area(&self) -> f64;
}   // end of trait HasArea



impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }   // end of fn area
}   // end of impl Rectangle

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }   // end of fn area
}   // end of impl Circle

impl Area for Triangle {
    fn area(&self) -> f64 { 
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c) / 2.0;
        (p * (p - a) * (p - b) * (p - c)).sqrt()
    }   // end of fn area   
    
}   // end of impl Triangle


fn main() {
    let rectangle = Rectangle { a: 3.0, b: 4.0 };

    let shape = Shape::Rectangle(rectangle);

    println!("Area of rectangle: {}", shape.area());

}
