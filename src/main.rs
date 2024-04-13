use std::{
    fs::File,
    io::{self, Read, Write},
};

fn main() {
    // get input file
    // check arg for file name, only do following if absent
    println!("Welcome to Comparrian!");
    let mut input = String::new();
    let mut file;
    loop {
        print!("Enter a file name to sort:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let Ok(_file) = File::options().read(true).write(true).open(input.trim()) else {
            println!("Invalid input.");
            input.clear();
            continue;
        };
        file = _file;
        break;
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut count = Vec::new();
    for val in contents.split('\n') {
        // println!("{}; {}", val, val.len());
        count.push(val.len()); // temp
    }

    let mut sorted: Vec<char> = contents.chars().filter(|x| *x != '\n').collect();
    // dbg!(&sorted);

    sorted.sort_unstable();
    // let sorted: String = sorted.into_iter().collect();
    // println!("{}", &sorted);

    let mut final_str = Vec::new();
    let mut total = 0;
    for len in count {
        final_str.extend_from_slice(&sorted[total..(len + total)]);
        final_str.push('\n');
        total += len;
    }
    let final_str: String = final_str.into_iter().collect();
    // println!("{}", final_str);

    file.set_len(0).unwrap();
    file.write_fmt(format_args!("{}", final_str)).unwrap();

    // iterate through it, count characters until newline, hold as Vec<i64>
    // remove newlines from string
    // alphabetize string
    // pring to original file, using vec to insert newlines
}
