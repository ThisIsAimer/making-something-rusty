// Problem 2: Fix the code by making suitable changes to the signatures 
// of the functions 'get_vehicle' and 'operate_vehicle' 

trait Vehicle {
    fn start_engine(&self) -> String;
    fn drive(&self) -> String;
}
#[derive(Debug)]
struct Car;

struct Bicycle;

impl Vehicle for Car {
    fn start_engine(&self) -> String {
        "🚗 Engine started".to_string()
    }

    fn drive(&self) -> String {
        "🚗 Driving the car".to_string()
    }
}

impl Vehicle for Bicycle {
    fn start_engine(&self) -> String {
        "🚲 No engine to start for a bicycle".to_string()
    }

    fn drive(&self) -> String {
        "🚲 Pedaling the bicycle".to_string()
    }
}

fn get_vehicle(vehicle_type: &str) -> &dyn Vehicle { // This line needs a fix 
    match vehicle_type {
        "car" => &Car,
        "bicycle" => &Bicycle,
        _ => panic!("No vehicle of that type available"),
    }
}

fn operate_vehicle(driver: &dyn Vehicle) { // This line needs a fix 
    println!("{}", driver.start_engine());
    println!("{}", driver.drive());
}

fn main() {
    //this code only works cause rust allows us to create references to
    //structs that have no memory. otherwise we would need to use Box pointer.
    let my_vehicle = get_vehicle("car"); 
    operate_vehicle(my_vehicle);

    let my_vehicle = get_vehicle("bicycle");
    operate_vehicle(my_vehicle);
}

/*
// Problem 2: Fix the code by making suitable changes to the signatures 
// of the functions 'get_vehicle' and 'operate_vehicle' 
// Solution: 

trait Vehicle {
    fn start_engine(&self) -> String;
    fn drive(&self) -> String;
}

struct Car;

struct Bicycle;

impl Vehicle for Car {
    fn start_engine(&self) -> String {
        "🚗 Engine started".to_string()
    }

    fn drive(&self) -> String {
        "🚗 Driving the car".to_string()
    }
}

impl Vehicle for Bicycle {
    fn start_engine(&self) -> String {
        "🚲 No engine to start for a bicycle".to_string()
    }

    fn drive(&self) -> String {
        "🚲 Pedaling the bicycle".to_string()
    }
}

fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "car" => Box::new(Car),
        "bicycle" => Box::new(Bicycle),
        _ => panic!("No vehicle of that type available"),
    }
}

fn operate_vehicle(driver: &dyn Vehicle) { 
    println!("{}", driver.start_engine());
    println!("{}", driver.drive());
}

fn main() {
    let my_vehicle = get_vehicle("car"); 
    operate_vehicle(my_vehicle.as_ref());

    let my_vehicle = get_vehicle("bicycle");
    operate_vehicle(my_vehicle.as_ref());
}

*/