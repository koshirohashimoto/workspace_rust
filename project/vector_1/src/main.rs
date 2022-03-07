use std::slice::RChunks;

fn main() {
    let v = vec![1,2,3,4,5];
    for i in &v {
        println!("{}", i);
    }
    
    let mut v2 = vec![10,20,30,40,50];
    for k in &mut v2 {
        *k += 50;
        println!("{}", k);
    } 
}
