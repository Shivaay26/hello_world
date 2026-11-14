fn main(){
    let num=6;
    if num%2==0{
        print!("{} is even\n",num);
    }else{
        print!("{} is odd\n ",num);
    }

    let mut counter=0;
    let result=loop{
        counter+=1;
        if counter==5{
            break counter*2;//this will be returned from the loop and assigned to result
        }
    };
    print!("result is {}\n",result);

    let mut n=3;
    while n!=0{
        print!("{}!",n);
        n-=1;
    };

    print!("LIFTOFF!!!\n");
}