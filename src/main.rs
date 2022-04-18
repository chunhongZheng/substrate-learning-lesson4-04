////@author caspar    2022/04/14
//为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
trait TrafficLightState {
    fn time(state: TrafficLight) -> u16;
}
enum TrafficLight{
    Red,
    Green,
    Yellow,
}
impl TrafficLightState for TrafficLight {
    fn time(state: TrafficLight)->u16{
        match state {
            TrafficLight::Red => {
                println!("traitLight Red time is {} second", 30);
                30
            },
            TrafficLight::Green => {
                println!("traitLight Green time is {} second", 40);
                40
            },
            TrafficLight::Yellow => {
                println!("traitLight Yellow time is {} second", 10);
                10
            },
        }

    }
}

fn main() {
    let  redLight = TrafficLight::Red;
    println!("red light time is {}",TrafficLight::time(redLight));
    let greenLight = TrafficLight::Green;
    println!("green light time is {}",TrafficLight::time(greenLight));




}
