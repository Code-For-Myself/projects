
fn main() {
    let a = [1,2,3,4,5];
    let slice = &a[0..4];
    let x = slice.len();
    println!("Ich bin {x} slice.")
}
