fn partition(arr:&mut[i32],low:usize,high:usize)->usize{
    let pivot=arr[high];
    let mut i=low;

    for j in low..high{
        if arr[j]< pivot{
            arr.swap(i.j);
            i+=1;
        }
    }
    arr.swap(i,high);
    i
}
fn quickselect(arr: &mut[i32], low: usize,high:usize,k:usize)->i32{
    if low==high{
        return arr[low];
    }
    let pivot_index=partition(arr,low,high);
    if k==pivot_index{
        arr[k]
    }
    else if k < pivot_index{
        quickselect(arr,low,pivot_index-1,k)

    }
    else{
        quickselect(arr,pivot_index+1,high,k)
    }
    
}
fn kth_smallest(arr:&mut [i32],k:usize)->i32{
    if k==0 ||k>arr.len(){
        panic!("k is out of bound");
    }
    quickselect(arr,0,arr.len()-1,k-1)

}
 fn main()
 {

    let mut arr=vec![7,10,4,3,20,15];
    let k=3;
    println!("the {}th  smallest term is {}",k,kth_smallest(&mut arr,k));
    let mut arr2=vec![7,10,4,3,20,15];
    let k2=4; 
    println!("the {}the smallest is {}",k2,kth_smallest(&mut arr2,k2));
 }