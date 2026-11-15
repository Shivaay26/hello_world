fn main(){
    //integer allowed to borrow.
    let x=5;
    let y=x;
    print!("x: {}\n", x);
    print!("y: {}\n", y);

    let s1= String::from("hello"); //heap allocated growable string
    // let s2=s1;
    let s2=s1.clone();
    print!("s1: {}\n", s1);
    print!("s2: {}\n", s2);
}