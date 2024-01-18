fn main() {
    println!("Hello, world!");
    let x = 2;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x inside scope: {}", x);
    }
    println!("x outside scope: {}", x);
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
    let mut var = "string";
    // won't work, can't change type of mut variable
    // var = var.len();
    println!("var: {}", var);
    
    let mut test_shadow = "string"; // `mut` here is useless
    let mut test_shadow = 5;
    test_shadow = 6;
    println!("test_shadow: {}", test_shadow);

    let y = 5/3;
    println!("y rounded: {}", y);

}
