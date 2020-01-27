mod guess_number;
mod string;
mod enum_;
mod vec_;
mod hashmap;
mod file;
mod generic;
pub fn main()
{
  //string::string_test();
  //guess_number::guess_number();
  enum_::enum_test();
  enum_::mod_test::test();
  enum_::mod1::hello();
  vec_::test_vec();
  hashmap::test_hash_map();
  hashmap::character_statistics();
  //file::file_test();
  generic::test();
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
  let mut nums = nums.clone();
  nums.sort();
  let sz =  nums.len() as i32;
  let mut left = 0;
  let mut right = sz - 1;
  while left < right {
    if nums[left as usize] + nums[right as usize] == target {
      return vec![left, right];
    }else if nums[left as usize] + nums[right as usize] < target {
      left += 1;
    }else{
      right +=1;
    }
  }
  vec![]
}