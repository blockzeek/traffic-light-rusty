enum TrafficLight {
	Red,
	Green,
	Yellow,
}

impl TrafficLight {
    fn duration(&self) -> u8{
        60
    }
}

fn main() {
    let yellow_light = TrafficLight::Yellow;
    println!("yellow light is {}", yellow_light.duration());
}