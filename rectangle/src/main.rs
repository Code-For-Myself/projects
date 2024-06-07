#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 50;
    let height1 = 30;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The Area of the rectangle is {} square pixels!", area(width1, height1));
    println!("The Area of the rectangle is {} square pixels!", area2(rect1));
    println!("The Area of the rectangle is {} square pixels!", area3(&rect2));
    println!("rect2 is {:?}", rect2)
}
fn area(width: u32, height: u32) -> u32 {
    width*height
}
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0*dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width*rectangle.height
}
