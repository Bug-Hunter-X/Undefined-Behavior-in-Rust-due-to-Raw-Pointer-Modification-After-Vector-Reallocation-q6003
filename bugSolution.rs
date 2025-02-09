fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; //Correct and safe way to modify the value
    println!("{:?}", v);
}