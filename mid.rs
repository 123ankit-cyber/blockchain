fn find_median(arr:&[i32])->f64{

    let len=arr.len();
    if len==0{
        println!("cannot find the median of an empty array");

    }

    if len %2 ==0{
        let mid1=len/2-1;
        let mid2=len/2;
        (arr[mid]+arr[mid+2]) as f64 /2.0
    }
    else {
        arr[len/2] as f64
    }
}
fn main()
{
    let sorted_array=vec![1,2,3,4,5];
    println!("the median is {}", find_median(&sorted_array));

    let sorted_array_even=vec![1,2,3,4,5,6];
    println!("the median is{}",find_median(&sorted_array_even));
}