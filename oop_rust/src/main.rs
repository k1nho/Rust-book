// Encapsulation
struct AverageCalculator {
    list: Vec<i32>,
    avg: f64,
}

impl AverageCalculator {
    pub fn push(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let res = self.list.pop();
        match res {
            Some(x) => {
                self.update_average();
                Some(x)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.avg = sum as f64 / self.list.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.avg
    }
}

// INHERITANCE (with traits)
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// if we were to have a generic implementation then the idea is that every component will be of
// a type that implements Draw but an specific type rather than a collections of types
// (monomorphization)
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let mut avg_calc = AverageCalculator {
        list: vec![32, 50, 74],
        avg: 0.0,
    };

    // we can use operation with the object's API, but we cannot get the fileds itself
    avg_calc.push(20);
    avg_calc.push(90);
    let opt = avg_calc.pop();

    match opt {
        Some(val) => println!("element {} was popped from the vector", val),
        None => println!("no value was popped from the vector"),
    }

    println!("The average of the vector is {}", avg_calc.average());
}
