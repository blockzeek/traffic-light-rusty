enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Duration {
    fn duration(&self) -> u8;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match &self {
            TrafficLight::Red => 0,
            TrafficLight::Green => 1,
            TrafficLight::Yellow => 255,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    println!("red light is {}", red_light.duration());

    let green_light = TrafficLight::Green;
    println!("green light is {}", green_light.duration());

    let yellow_light = TrafficLight::Yellow;
    println!("yellow light is {}", yellow_light.duration());
}
