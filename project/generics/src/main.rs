fn largest<T: PartialOrd + Copy + Clone>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let num_list = vec![34,59,25,100,65];
    let result = largest(&num_list);
    println!("The largest num is: {}", result);

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("The largest char is: {}", result);

    use generics::tweeter;
    tweeter()
}
