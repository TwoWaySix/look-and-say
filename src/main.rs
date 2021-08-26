use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let mut first_line: String = String::from("1");
    // let n_lines: usize = 3;
    //
    // run(&mut first_line, n_lines);
}

// fn run(line: &mut String, n_lines: usize) {
//     let mut next_line: String = line.clone();
//     for i in 0..n_lines {
//         next_line = create_next_line(next_line);
//     }
// }

fn create_next_line(line: String) -> String {
    let next_line: String = String::from("");
    return next_line;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nextline_with_input_line_1() {
        let mut line: String = String::from("1");
        let expected = String::from("11");

        line = create_next_line(line);
        assert_eq!(line, expected);
    }
}