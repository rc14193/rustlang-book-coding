struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 40};

    println!("The area of a rect is {}",area_rect(&rect1));
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
