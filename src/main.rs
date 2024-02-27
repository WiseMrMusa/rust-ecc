use ecc::finite_field::{field::{self,Field, FieldTrait}, ball};

fn main() {
    let a = Field::new(3,9);
    let b = Field::new(8,9);
    let c = a + b;
    println!("{} is a finite field element", a);
    println!("{} is a finite field element", b);
    println!("{} + {} = {}", a,b,c);
    
    let a = Field::new(3,9);
    let b = Field::new(8,9);
    let c = a * b;
    println!("{} is a finite field element", a);
    println!("{} is a finite field element", b);
    println!("{} * {} = {}", a,b,c);
    println!("Hello, world!");
    ball::play();

    field::main();
}
