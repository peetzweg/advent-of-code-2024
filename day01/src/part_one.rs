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
                let mut left: Vec<u32> = vec![];
                let mut right: Vec<u32> = vec![];
                for line in reader {
                    if let Ok(line) = line {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        left.push(parts[0].parse().unwrap());
                        right.push(parts[1].parse().unwrap());
                    }
                }
                left.sort();
                right.sort();
                println!("{:?}", left);
                println!("{:?}", right);

                let mut total_distance: u32 = 0;
                left.iter().zip(right.iter()).for_each(|(l, r)| {
                    println!("{} {}", l, r);
                    let distance = if (l > r) { l - r } else { r - l };

                    total_distance += distance;
                });
                println!("total_distance: {}", total_distance);
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
