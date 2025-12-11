#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }

    
}

fn main(){
     let rect1: Rectangle = Rectangle{
        width: 30,
        height:50,
     };

     println!("The area of rentangle is rect1 {} ", rect1.area());
     println!("rect1 can hold rect {}", rect1.can_hold(&Rectangle{width:20, height:40}));
     println!("square {:?}", Rectangle::square(5));
}