fn mx_sub_array_sum(nums: Vec<i32>)->i32{
    let mut max_so_far=nums[0];
    let mut max_ending_here=nums[0];
for &num in nums.iter().skip(1){
    max_ending_here=i32::max(num,max_ending_here+num);
    max_so_far=i32::max(max_so_far,max_ending_here);

}
max_so_far
}
 fn main()
 {
    let nums=vec![-2,1,-3,4,-1,2,1,-5,4];
    println!("maximum subarray sm{}",mx_sub_array_sum(nums));
 }