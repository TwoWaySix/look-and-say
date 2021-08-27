// use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let mut first_line: String = String::from("1");
    // let n_lines: usize = 3;
    //
    // run(&mut first_line, n_lines);
    let a = create_next_line(String::from("221113"));
}

// fn run(line: &mut String, n_lines: usize) {
//     let mut next_line: String = line.clone();
//     for i in 0..n_lines {
//         next_line = create_next_line(next_line);
//     }
// }

fn clean_input_line(line: String) -> String {
    line.chars()
        .filter(|c| c.is_numeric())
        .collect()
}

fn create_next_line(line: String) -> String {
    let mut next_line: String = String::from("");
    let mut prev_number = line.chars().next().unwrap().clone();
    let mut counter = 1;

    println!("\n{}", line);
    for (i, number) in line.chars().enumerate().skip(1) {
        if number == prev_number {
            counter += 1;
        } else {
            println!("{} times {}", counter, prev_number);
            next_line.push(char::from_digit(counter, 10).unwrap());
            next_line.push(prev_number);
            counter = 1;
        }
        prev_number = number.clone();
        next_line.push(number);
    }
    println!("{} times {}", counter, prev_number);
    next_line.push(char::from_digit(counter, 10).unwrap());
    next_line.push(prev_number);

    next_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input_line() {
        assert_eq!(clean_input_line(String::from("0123")), String::from("0123"));
        assert_eq!(clean_input_line(String::from("a123")), String::from("123"));
        assert_eq!(clean_input_line(String::from("a1Z3")), String::from("13"));
        assert_eq!(clean_input_line(String::from("!929!%)d0s")), String::from("9290"));
        assert_eq!(clean_input_line(String::from("00")), String::from("00"));
        assert_eq!(clean_input_line(String::from("aasdafg")), String::from(""));
    }

    #[test]
    fn test_nextline_with_input_line() {
        assert_eq!(
            create_next_line(String::from("1")),
            String::from("11")
        );
        assert_eq!(
            create_next_line(String::from("333122")),
            String::from("331122")
        );
        assert_eq!(
            create_next_line(String::from("112")),
            String::from("2112")
        );
        assert_eq!(
            create_next_line(String::from("321")),
            String::from("131211")
        );
    }
}