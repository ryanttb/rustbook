#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    //let w = 30;
    //let h = 30;

    //let d = (30, 30);

    let r = Rectangle { w: 30, h: 30 };
    println!("r is {:?}", r);

    println!(
        "The area is {}",
        //area(w, h)
        //area(d)
        area(&r)
    );
}

//fn area(w: u32, h: u32) -> u32 {
//    w * h
//}

//fn area(d: (u32, u32)) -> u32 {
//    d.0 * d.1
//}

fn area(r: &Rectangle) -> u32 {
    r.w * r.h
}
