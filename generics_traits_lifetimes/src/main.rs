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

// TRAITS
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : String,
    pub retweet : String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as param
pub fn alert(item : &impl Summary) {
    println!("Alert! {}", item.summarize())
}
// If we want the same type that implements the Trait we need to use generic
pub fn same_type_alert<T: Summary>(item1 : &T, item2 : &T) {
    println!("summary 1 : {}", item1.summarize());
    println!("summary 2 : {}", item2.summarize());
}

// Returning types that implement traits
fn return_summarizable() -> impl Summary {
    Tweet{
        username: String::from("kinho"),
        content: String::from("Getting Rusty"),
        reply: String::from("here we go"),
        retweet: String::from("yes")
    }
}

fn main() {
    let p1 = Point{x : "p1x", y: "p1y"};
    let p2 = Point{x : 1, y : 2};
    let p3 = p1.create_composite_point(p2);
    let tweet = return_summarizable();
    println!("we have the composite point is {:?}", p3);
}


