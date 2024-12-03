use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    match std::env::args().nth(1) {
        None => {
            println!("no filename, please provide as first argument");
        }
        Some(file_path) => {
            if let Ok(reader) = read_lines(file_path) {
                let mut sum = 0;
                let mut count_enabled = 0;
                let mut count_disabled = 0;
                let mut count_do = 0;
                let mut count_don = 0;
                let mut is_enabled = true;
                let re = regex::Regex::new(r"mul\((\d+,\d+)\)|(do)\(\)|(don)\'t\(\)").unwrap();
                for line in reader {
                    let line = line.expect("LINE TO BE OK");

                    for (_, [capture]) in re.captures_iter(&line).map(|c| c.extract()) {
                        // println!("{}", capture);
                        match capture {
                            "do" => {
                                is_enabled = true;
                                count_do += 1;
                            }
                            "don" => {
                                is_enabled = false;
                                count_don += 1;
                            }
                            _ => {
                                if is_enabled {
                                    let mut parts = capture.split(",");
                                    let left = parts.next().unwrap();
                                    let right = parts.next().unwrap();
                                    let result = u32::from_str_radix(left, 10).unwrap()
                                        * u32::from_str_radix(right, 10).unwrap();
                                    sum += result;
                                    println!("{}*{}={} | {}", left, right, result, sum);
                                    count_enabled += 1;
                                } else {
                                    count_disabled += 1;
                                }
                            }
                        }
                    }
                }
                println!("sum: {}", sum);

                println!("count_enabled: {}", count_enabled);
                println!("count_disabled: {}", count_disabled);

                println!("count_do: {}", count_do);
                println!("count_don: {}", count_don);
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
