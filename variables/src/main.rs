fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the innner scope is: {}", x);
    }
    println!("The value of x is : {}", x);
}
