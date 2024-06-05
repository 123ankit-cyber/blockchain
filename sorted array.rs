fn find_first_occurrence(arr: &[i32],target:i32)->Option<usize>
{
    let mut low=0;
    let mut high=arr.len();
    while low<high{
        let mid=low+(high-low)/2;
        if arr[mid]< target{
            low=mid+1;
        }
        else{
            high=mid;
        }
    }

    if low<arr.len()&&arr[low]==target{
        Some(low)
    }
    else{
        None
    }
}

fn main()
{
    let arr=[1,2,2,3,4,5,5,6];
    let target=2;
    match find_first_occurrence(&arr,target){
        Some(index)=> println("firt occurrence of {} is at index{}",target,index),
        None =>println("{} is not in the array",target),
    }
}