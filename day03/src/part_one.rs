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
                let mut count = 0;
                for line in reader {
                    let re = regex::Regex::new(r"(mul\((\d+),(\d+)\))").unwrap();

                    if let Ok(line) = line {
                        // content.push_str(&line);

                        for (_, [all, left, right]) in re.captures_iter(&line).map(|c| c.extract())
                        {
                            let result = u32::from_str_radix(left, 10).unwrap()
                                * u32::from_str_radix(right, 10).unwrap();
                            println!("{}*{}={} | {}", left, right, result, all);
                            sum += result;
                            count += 1;
                        }
                    } else {
                        println!("POLOError reading line");
                    }
                    // println!("{}", content);
                }
                println!("sum: {}", sum);
                println!("count: {}", count);
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
