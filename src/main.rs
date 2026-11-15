#[derive(Debug)]

enum TrafficLight{
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
enum Shapes{
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main(){
    let light = TrafficLight::Red;
    print!("{:?}\n", light);

    let shape=Shapes::Circle(10.0);
    print!("{:?}\n", shape);

    match light{
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }

    //match is exhaustive, so we need to handle all variants of the enum
    match shape{
        Shapes::Circle(radius) => println!("Circle with radius: {}", radius),
        Shapes::Rectangle(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
        Shapes::Square(side) => println!("Square with side: {}", side),
        _ => println!("Unknown shape"),
    }
}

