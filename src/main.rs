fn main() {
    println!("Hello, world!");
    let tup = (500, 6.4, 1);
    println!("Value of z is {}",anotherfunction(tup));
}
fn anotherfunction(tup: (i32,f64,u8))-> u8{
 println!("Value of y is {}",tup.1);
tup.2
}
