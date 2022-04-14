////@author caspar    2022/04/14
//为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
enum traitLight{
    Red,
    Green,
    Yellow,
}
impl traitLight {
    fn time(&self)->u16{
        match &self {
            traitLight::Red => {
                println!("traitLight Red time is {} second", 30);
                30
            },
            traitLight::Green => {
                println!("traitLight Green time is {} second", 40);
                40
            },
            traitLight::Yellow => {
                println!("traitLight Yellow time is {} second", 10);
                10
            },
        }

    }
}

fn main() {
    let  redLight = traitLight::Red;
    println!("red light time is {}",redLight.time());
    let greenLight = traitLight::Green;
    println!("green light time is {}",greenLight.time());
}
