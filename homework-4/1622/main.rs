// 定义一个名为 TrafficLightDuration 的 trait，包含一个返回 u32 类型的 duration 方法
trait TrafficLightDuration {
    fn duration(&self) -> u32;
}

// 定义一个枚举 TrafficLight，包含 Red、Yellow 和 Green 三种状态
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLight 实现 TrafficLightDuration trait
impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        // 使用 match 表达式根据 TrafficLight 的不同状态返回不同的持续时间
        match self {
            TrafficLight::Red => 60,   // 红灯持续60秒
            TrafficLight::Yellow => 10, // 黄灯持续10秒
            TrafficLight::Green => 45,  // 绿灯持续45秒
        }
    }
}

// 定义一个函数 sum_u32，接受一个 u32 类型的切片，返回 Option<u32>
// 使用 iter 和 try_fold 方法计算切片中所有元素的和，如果发生溢出则返回 None
fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers.iter()
        .try_fold(0u32, |acc, &x| acc.checked_add(x))
}

// 定义一个名为 Area 的 trait，包含一个返回 f64 类型的 area 方法
trait Area {
    fn area(&self) -> f64;
}

// 定义一个结构体 Circle，包含一个 f64 类型的 radius 字段
struct Circle {
    radius: f64,
}

// 为 Circle 实现 Area trait
impl Area for Circle {
    fn area(&self) -> f64 {
        // 计算圆的面积，使用 std::f64::consts::PI 表示 圆周率
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义一个结构体 Triangle，包含两个 f64 类型的 base 和 height 字段
struct Triangle {
    base: f64,
    height: f64,
}

// 为 Triangle 实现 Area trait
impl Area for Triangle {
    fn area(&self) -> f64 {
        // 计算三角形的面积
        0.5 * self.base * self.height
    }
}

// 定义一个结构体 Square，包含一个 f64 类型的 side 字段
struct Square {
    side: f64,
}

// 为 Square 实现 Area trait
impl Area for Square {
    fn area(&self) -> f64 {
        // 计算正方形的面积
        self.side * self.side
    }
}

// 定义一个泛型函数 print_area，接受一个实现了 Area trait 的类型 T
// 打印该类型的面积
fn print_area<T: Area>(shape: T) {
    println!("The area is {}", shape.area());
}

// main 函数是程序的入口点
fn main() {
    // 创建一个 TrafficLight::Red 实例
    let red_light = TrafficLight::Red;
    // 打印交通灯的持续时间
    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", TrafficLight::Yellow.duration());
    println!("Green light duration: {} seconds", TrafficLight::Green.duration());
    // 创建一个包含一些 u32 数字的向量
    let numbers = vec![1, 2, 3, 4];
    // 计算这些数字的和，并根据结果打印相应的消息
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Sum overflowed"),
    }

    // 创建一个 Circle 实例并打印其面积
    let circle = Circle { radius: 3.0 };
    print_area(circle);

    // 创建一个 Triangle 实例并打印其面积
    let triangle = Triangle { base: 4.0, height: 5.0 };
    print_area(triangle);

    // 创建一个 Square 实例并打印其面积
    let square = Square { side: 6.0 };
    print_area(square);
}