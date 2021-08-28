use std::env;

fn main() {
    let (mut first_line, n_lines) = parse_arguments();

    println!("\nCreating {} lines of look-and-say starting with {}", n_lines, first_line);
    run(&mut first_line, n_lines);
}

fn parse_arguments() -> (String, usize) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("ERROR: This tool needs exactly two command line arguments:\n\
        1: First line = Starting 'number'\n\
        2: Number of following lines to create.");
        std::process::exit(1);
    }

    let mut first_line: String = args[1].clone();

    let n_lines = args[2].parse::<usize>();
    let n_lines = match n_lines {
        Ok(i) => { i },
        Err(_) => {
            println!("ERROR: Given amount of lines '{}' cannot be interpreted \
                      as an unsigned integer.", args[2]);
            std::process::exit(1)
        },
    };

    first_line = filter_all_numeric_characters(first_line);
    if first_line.len() == 0 {
        println!("ERROR: Given first line does not even contain a number.");
        std::process::exit(1);
    }

    (first_line, n_lines)
}

fn run(line: &mut String, n_lines: usize) {
    let mut next_line: String = line.clone();

    println!("1: {}", line);
    for i in 0..n_lines {
        next_line = create_next_line(next_line);
        println!("{}: {}", i+2, next_line);
    }
}

fn filter_all_numeric_characters(line: String) -> String {
    line.chars()
        .filter(|c| c.is_numeric())
        .collect()
}

fn create_next_line(line: String) -> String {
    let mut next_line: String = String::from("");
    let mut prev_number = line.chars().next().unwrap().clone();
    let mut counter = 1;

    for number in line.chars().skip(1) {
        if number == prev_number {
            counter += 1;
        } else {
            next_line = format_output_line(&next_line, counter, &prev_number);
            counter = 1;
        }
        prev_number = number.clone();
    }
    next_line = format_output_line(&next_line, counter, &prev_number);
    next_line
}

fn format_output_line(next_line: &str, counter: i32, prev_number: &char) -> String {
    format!("{}{}{}", next_line, counter, prev_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input_line() {
        assert_eq!(filter_all_numeric_characters(String::from("0123")), String::from("0123"));
        assert_eq!(filter_all_numeric_characters(String::from("a123")), String::from("123"));
        assert_eq!(filter_all_numeric_characters(String::from("a1Z3")), String::from("13"));
        assert_eq!(filter_all_numeric_characters(String::from("!929!%)d0s")), String::from("9290"));
        assert_eq!(filter_all_numeric_characters(String::from("00")), String::from("00"));
        assert_eq!(filter_all_numeric_characters(String::from("aasdafg")), String::from(""));
    }

    #[test]
    fn test_nextline_with_input_line() {
        assert_eq!(
            create_next_line(String::from("1")),
            String::from("11")
        );
        assert_eq!(
            create_next_line(String::from("12")),
            String::from("1112")
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
        assert_eq!(
            create_next_line(String::from("000000000")),
            String::from("90")
        );
        assert_eq!(
            create_next_line(String::from("0000000000")),
            String::from("100")
        );
    }

    #[test]
    fn test_formatting() {
        let c: char = "9".chars().next().unwrap();
        assert_eq!(
            format_output_line(
                &String::from("2"),
                11,
                &c
            ),
            String::from("2119"));
    }
}