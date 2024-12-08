enum TrafficLight {
    Red,
    Yellow,
    Green
}

fn main(){
    let light = TrafficLight::Green;

    match light {
        TrafficLight::Red => println!("หยุด!!"),
        TrafficLight::Yellow => println!("ชะลอ!!"),
        TrafficLight::Green => println!("ไปเลย!!")
    }

    
}