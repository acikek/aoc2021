use crate::solution;
use crate::util::lines;

pub fn first_second(lines: &Vec<String>) -> (isize, isize) {
    // Problem 1
    let mut x = 0;
    let mut y1 = 0;

    // Problem 2
    let mut y2 = 0;
    let mut aim = 0;

    for line in lines {
        let pair = line.split_once(" ").unwrap();
        let n = pair.1.parse::<isize>().unwrap();

        match pair.0 {
            "forward" => { 
                x += n;
                y2 += aim * n;
            }
            "down" => { 
                y1 += n;
                aim += n;
            }
            "up" => {
                y1 -= n;
                aim -= n;
            }
            _ => unimplemented!()
        }
    }

    (x * y1, x * y2)
}

pub fn exec() {
    let values = first_second(&lines());
    solution!(1: values.0);
    solution!(2: values.1);
}