const MAX_POINTS: u32 = 100_000; //non negative 32 bit integer, underscores are allowed for readability


fn main() {
    //scalar types
    let age: u32 = 30; //non negative 32 bit integer
    let num: i32 = 5; //signed 32 bit integer
    let temp: f64 = 3.14; //64 bit floating point number
    let is_active: bool = true; //boolean
    let grade: char = 'A'; //character

    print!("age is {}, num is {}, temp is {}, is_active is {}, grade is {}", age, num, temp, is_active, grade);

    greet("Alice");

    //compound types
    //tuples

    let person:(&str, u32) = ("Alice", 30); //tuple with a string slice and a 32 bit unsigned integer
    println!("person is ({}, {})", person.0, person.1); //accessing tuple elements using dot notation

    //arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //array of 5 signed 32 bit integers
    println!("numbers is {:?}", numbers);
    print!("numbers at 0th index is {}\n", numbers[0]); //accessing array elements using index notation

    //shadowing
    let y=5;
    println!("y is {}", y);

    let y=y+1; //shadowing the previous y
    println!("y is {}", y);
} 

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 { //function that takes two signed 32 bit integers and returns a signed 32 bit integer
    x + y //is an expreesion , its value will be returned from the function
}