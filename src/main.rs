fn main() {
    println!("Hello, world!");
    let tup = (500, 6.4, 1);
    println!("Value of z is {}",anotherfunction(tup));
}
fn anotherfunction(tup: (i32,f64,u8))-> u8{
 println!("Value of y is {}",tup.1);
 anotheranotherfunction(3);
tup.2
}
fn anotheranotherfunction(mut x: i32){
 while x!=0{
  println!("{}!",x);
  x-=1;
 } 
 println!("LIFTOFF!!");
}
