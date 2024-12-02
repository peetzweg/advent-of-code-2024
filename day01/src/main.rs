use std::collections::HashMap;
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

                let mut score: u32 = 0;
                let mut similarity: HashMap<u32, u32> = HashMap::new();

                left.iter().for_each(|e| {
                    if !similarity.contains_key(e) {
                        let mut count = 0;
                        right.iter().for_each(|r| {
                            if r == e {
                                count += 1;
                            }
                        });
                        similarity.insert(*e, count);
                    }

                    score += e * similarity[e];

                    println!("{}*{}={} | {}", e, similarity[e], e * similarity[e], score);
                });

                println!("{:?}", similarity);
                println!("similarity: {}", score);
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
