use core::num;
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calc: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, rand_num: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", 
            expensive_closure.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    }else {
        if rand_num == 3 {
            println!("Take a break today!");
        }else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn main() {
    let user_specified_val = 10;
    let rand_num = 7;

    generate_workout(
        user_specified_val,
        rand_num
    );
}
