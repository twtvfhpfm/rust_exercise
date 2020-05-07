pub fn iter_test()
{
    let v1 = vec![1,2,3];

    for i in v1.iter() {
        println!("got value {}", i);
    }

    let i1 = v1.iter();
    let total: i32 = i1.sum();
    println!("sum is {}", total);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    odd_nums(v2);

    for i in Counter::new() {
        println!("nums in Counter {}", i);
    }
}

pub fn odd_nums(nums: Vec<i32>) -> Vec<i32>
{
    nums.into_iter().filter(|i| i % 2 == 0).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}