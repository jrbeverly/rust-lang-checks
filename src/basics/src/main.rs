fn main() {
    println!("Hello, world!");
    println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');

    let mut a_number : u32;
    let a_word = "Ten";

    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);
    a_number = 15;
    println!("The number is {}.", a_number);

    print_byrange(500, 0, 512);

    let simple_struct = generate_struct();
    print_struct(simple_struct);
}

struct DataOf { name: String, scale: u8 }

fn generate_struct() -> DataOf {
    let result = DataOf { name: String::from("ABC"), scale: 5 };
    return result;   
}

fn print_struct(st : DataOf) {
    println!("The name is {}.", st.name);
    println!("The scale is {}.", st.scale);
 }

 fn print_byrange(num: i32, min : i32, max: i32) {
    let out_of_range: bool;
    if num < min {
        out_of_range = true;
    } else if num == min {
        out_of_range = true;
    } else if num > max {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("The number is {}.", out_of_range);
 }