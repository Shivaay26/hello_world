use std::collections::HashMap;

fn main(){
    let mut scores: Vec<_> = vec![10, 20, 30, 40, 50];
    let mut ages: HashMap<String, u32> = HashMap::new();

    ages.insert(String::from("Praveen"), 25);
    ages.insert(String::from("John"), 30);

    if let Some(age)=ages.get("Praveen"){
        println!("Praveen's age is {}", age); 
    }
    scores.push(60);
    for score in scores {
        println!("Score: {}", score);
    }
}

