fn main() {
    let t = 450_000.0;
    let m = 1_500_000.0;
    let h = 750_000.0;
    let d = 2_850_000.0;
    let a = 250_000.0;
    let q1 = 2.0;
    let q2 = 1.0;
    let q3 = 3.0;
    let q4 = 3.0;
    let q5 = 1.0;

    let s1 = t * q1;
    let s2 = m * q2;
    let s3 = h * q3;
    let s4 = d * q4;
    let s5 = a * q5;

    let z = s1 + s2+ s3 + s4 + s5;
    let y = z / 5.0;

    println!("The sum is {}", z);
    println!("The average is {}", y);
}
