#[derive(Debug)]

struct User{
    username: String,
    age: u32,
    active: bool,
}

fn main(){
    let user1= User{
        username: String::from("Alice"),
        age: 30,
        active: true,
    };
    print!("{}\n", user1.describe());
    print!("{:?}\n", user1);
}


//this is for adding methods to any given struct.
impl User{
    fn describe(&self) -> String{
        format!("Username: {}, Age: {}, Active: {}", self.username, self.age, self.active)
    }
}