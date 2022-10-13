fn main() {
    println!("Hello, world!");

    let v:Vec<i32> = Vec::new();
    let v1 = vec![1,2,3,4,5];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }
    
}
