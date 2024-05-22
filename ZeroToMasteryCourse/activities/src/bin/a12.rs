// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColors {
    White,
    Grey,
    Black,
    Brown,
}

//a string is a struct is defined as "x_var: String," and then you assign it as "x_var: String::from("xxx"),"

//is dimension just heihgt, length and width?

struct ShippingBox {
    l: f32,
    w: f32,
    h: f32,
    weight: f32,
    color: BoxColors,

}

impl ShippingBox {

    fn create_new_box(length: f32, width: f32, height: f32, box_weight: f32, box_color: BoxColors,) -> Self {
        Self {
            l: length,
            w: width,
            h: height,
            weight: box_weight,
            color: box_color,
        }
    }

    fn print_box_info(&self) {
        println!("{:?}", self.l);
        println!("{:?}", self.w);
        println!("{:?}", self.h);
        println!("{:?}", self.weight);
        //make a match for enum
        match self.color {
            BoxColors::White => println!("color is white"),
            BoxColors::Grey => println!("color is grey"),
            BoxColors::Black => println!("color is black"),
            BoxColors::Brown => println!("color is brown"),
        }
    }
}

fn main() {

    let box_1 = ShippingBox::create_new_box(44.0, 34.0, 22.0, 500.45, BoxColors::White);
    box_1.print_box_info()
}
