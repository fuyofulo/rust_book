#[derive(Debug)]

struct Rect {
    length: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        return self.length*self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2*(self.length+self.height);
    }
}

fn main() {
    
    let r1 = Rect {
        length: 15,
        height: 20
    };

    println!("Area of rectangle: {}", r1.area());
    println!("Perimeter of rectangle: {}", r1.perimeter());
}
