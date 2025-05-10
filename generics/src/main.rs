fn hypotenuse(a:f64,b:f64) -> f64 {
    (a.powi(2)+b.powi(2)).sqrt()
}

fn main() {

    let a:= 3.0;
    let b = 4.0;
    println!("{}", hypotenuse(a, b));
}
