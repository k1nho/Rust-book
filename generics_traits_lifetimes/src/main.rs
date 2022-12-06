use std::fmt::Display;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Point<X1, Y1> {
    x : X1,
    y : Y1,
}

pub struct Pair<T> {
    x: T, 
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("x is greater : {}", self.x)
        }
        else {
            println!("y is greater: {}", self.y)
        }
    }
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

// LIFETIMES: scoping system that controls borrowing rules

// Lifetime anotations
// this says: let the returned string sliced have a lifetime that is the same as the min of the
// values referred by the function arguments
fn longest_string<'a>(x : &'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {x}
    else {y}
}

// Lifetimes have 3 rules in general
// lifetimes get assigned to each parameter passed
// 1 parameter only? then lifetime of return is the same 
// if self is one of the parameters then lifetime of return is the same as self

// 'static lifetimes can be used to denote that a reference lives the entire program

fn main() {
    let p1 = Point{x : "p1x", y: "p1y"};
    let p2 = Point{x : 1, y : 2};
    let p3 = p1.create_composite_point(p2);
    let tweet = return_summarizable();
    println!("we have the composite point is {:?}", p3);
    println!("the tweet is the following: {:?}", tweet.summarize());
    let num = 3;
    // to_string can be called on integer type because it implements the Display Trait
    let numstr = num.to_string();

    let s1 = "Infernape";
    let s2 = "Empoleon";
    let s3 = longest_string(s1, s2);
}


