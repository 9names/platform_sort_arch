use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    for file in ["tier1", "tier2_w_hosttools", "tier2_wo_hosttools", "tier3"] {
        let mut outfile = File::create(format!("{file}_sorted.txt")).unwrap();
        if let Ok(lines) = read_lines(format!("{file}.txt")) {
            let mut results:Vec<_> = lines.flatten().collect();
            results.sort_by(|a,b| compare_platform_support_line(a, b));
            for line in results {
                outfile.write(line.as_bytes()).unwrap();
                outfile.write("\n".as_bytes()).unwrap();
            }
        }
        outfile.flush().unwrap();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn compare_platform_support_line(left:&str, right: &str) -> Ordering {
    // rely on existing ord relationships to get things in the right order
    extract_triple(left).cmp(&extract_triple(right))
}

fn extract_triple(input: &str) -> Vec<&str> {
    // We only care about the target triple, that should live between the first backticks
    // `msp430-none-elf` | * |  | 16-bit MSP430 microcontrollers
    // becomes
    // msp430-none-elf
    let triple:Vec<_> = input.split("`").collect();
    // since we split on backticks, the triple will live at index 1
    assert!(triple.len() > 1);
    // break the triple into segments delimited by -'s, eg msp430-none-elf becomes ["msp430", "none", "elf"]
    triple[1].split("-").collect()
}