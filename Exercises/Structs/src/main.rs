#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width >= rect2.width && self.length >= rect2.length
    }

    fn square(dimension: u32) -> Self {
        Self {
            width: dimension,
            length: dimension,
        }
    }
}

/**
 * Structs example from documentation
 */
fn main() {
    let width1 = 30;
    let length1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, length1)
    );

    let rect1 = (width1, length1);
    println!(
        "The area of the rectangle using tuples is {} square pixels.",
        area_tuple_refactor(rect1)
    );

    let real_rect: Rectangle = Rectangle {
        width: width1,
        length: length1,
    };
    dbg!(&real_rect);
    println!(
        "The area of the rectangle using structs is {} square pixels.",
        area_struct_refactor(&real_rect)
    );

    println!(
        "The area of the rectange using struct functions is {} square pixels.",
        real_rect.area()
    );

    let small_rect: Rectangle = Rectangle {
        width: 1,
        length: 1,
    };

    println!(
        "The small rectangle fits into the real: {}",
        real_rect.can_hold(&small_rect)
    );

    let square: Rectangle = Rectangle::square(4);
    println!("The area of the square is {}", square.area());
}

/** Calculates area of the retangle */
fn area(width: u32, length: u32) -> u32 {
    width * length
}

/** Pass only one parameter, but the elements are not named so less readability */
fn area_tuple_refactor(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/** Single parameter and it is very clear what is going on here. Width and length are clearly coupled by the struct and used to calculate the area. */
fn area_struct_refactor(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
