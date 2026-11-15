fn main(){
    //integer allowed to borrow.

    let mut s= String::from("hello"); //heap allocated growable string
    func(&mut s);
    print!("{}", s);

    //so basically we don't allow 2 mutable borrows at the same time.
    //in order to avoid data races, we can have multiple immutable borrows or one mutable borrow at a time.

}

fn func(s: &mut String){
    s.push_str("!")
}