use std::string;

fn main() {
    let testdata = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    println!("Hello, world!");

    let mut sum = 0;
    for line in testdata.lines() {
        let next = extract_calibration_value(line);
        sum += next;
    }

    println!("Sum: {}", sum);
}

fn extract_calibration_value(line: &str) -> i32 {
    let mut first_found = false;
    let mut last_found = false;
    let mut first_digit = ' ';
    let mut last_digit = ' ';
    let chars: Vec<_> = line.chars().collect();
    for i in 0..chars.len() {
        if !first_found {
            if chars[i].is_numeric() {
                first_digit = chars[i];
                first_found = true;
            }
        }
        if !last_found {
            if chars[chars.len() - i - 1].is_numeric() {
                last_digit = chars[chars.len() - i - 1];
                last_found = true;
            }
        }
    }

    let mut s = String::from("");
    s.push(first_digit);
    s.push(last_digit);

    s.parse::<i32>().unwrap()
}

// match chars[i].to_digit(10){
//     Some(d) => d,
//     None => 0
// };
