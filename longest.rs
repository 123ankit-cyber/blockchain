fn longest_common_prefix(strs: &[String]) ->String{
    if strs.is_empty()
    {
        resturn String::new();
    }
    let mut prefix =strs[0].clone();
    for s in strs.iter().skip(1)
    {
        while !s.starts_with(&prefix){


            if prefix.is_empty()
            {
                return String::new();
            }
            prefix.pop();
        } 
       }
       prefix
}

fn main()
{
    let strs=vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    println!("the longest common prefix is()",longest_common_prefix(&strs));
    let strs2=vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),

    ];
    println!("the longest common prefix is {}",longest_common_prefix(&strs2));
}