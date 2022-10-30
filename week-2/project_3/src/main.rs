fn main() {
   let p:f64 = 210_000.0;
   let r:f64 = 5.0; 
   let n:f64 = 3.0;

   // depriciation
   let z = 1.0 - (r / 100.0);
   let z = f64::powf(z,n); 

let ci = p * z;
   println!("The value of the TV after three years is {}", ci);
}
