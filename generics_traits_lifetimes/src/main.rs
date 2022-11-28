#[derive(Debug)]
pub struct Point<X1, Y1> {
    x : X1,
    y : Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn create_composite_point<X2, Y2>(self, p : Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x : self.x,
            y:  p.y
        }
    } 

}

fn main() {
    let p1 = Point{x : "p1x", y: "p1y"};
    let p2 = Point{x : 1, y : 2};
    let p3 = p1.create_composite_point(p2);
    println!("we have the composite point is {:?}", p3);
}


