use crate::solution;
use crate::util::nums;

pub fn first(nums: &Vec<isize>) -> isize {
    let mut last = nums.first().unwrap();
    let mut count = 0;

    for n in nums {
        if n > last { 
            count += 1; 
        }

        last = n;
    }

    count
}

pub fn second(nums: &Vec<isize>) -> isize {
    let mut last = 0;
    let mut count = 0;

    for i in 0..(nums.len() - 2) {
        let slice = &nums[i..i + 3];
        let sum: isize = slice.iter().sum();

        if sum > last {
            count += 1;
        }

        last = sum;
    }

    count - 1
}

pub fn exec() {
    let nums = nums();
    solution!(1: first(&nums));
    solution!(2: second(&nums));
}