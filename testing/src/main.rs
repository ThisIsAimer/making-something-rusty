enum Cordinates{
    Points(i32,i32)
}
impl Cordinates{
    fn add_points(&self) -> i32 {
    match self {
        Cordinates::Points(a,b) => {
            a+b
        }
    }
}

}

fn main(){
    let points = Cordinates::Points(5, 5);

    print!("{}",points.add_points())

}