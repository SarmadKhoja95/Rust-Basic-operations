fn main() {
   let a:u32 = 1000;
   let b:u32 = 500;
   println!("Addition of A and B = {}", a+b);
   println!("Multiplication of A and B = {}", a * b);
   println!("Division of A and B = {}", a/b);
   println!("Subtraction of A and B = {}", a-b);



  let fruit:(&str,f32,i8) = ("MANGO", 0.2 , 25);
  let (x,y,z) = fruit;
  println!(" Fruit name: {}", x);
  println!(" Weight in Kg : {}", y);
  println!(" Price in Rs : {}", z);


 let cricket_teams= ["Pakistan","SriLanka","India","Australia","England"];
 let year = [1992,1996,2000,2004,2008];
 println!(" Cricket Tean: {} -       Year:  {}", cricket_teams[0] , year[0]);
 println!(" Cricket Tean: {} -       Year:  {}", cricket_teams[1] , year[1]);
 println!(" Cricket Tean: {} -       Year:  {}", cricket_teams[2] , year[2]);
 println!(" Cricket Tean: {} -       Year:  {}", cricket_teams[3] , year[3]);


}
