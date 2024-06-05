fn is_plaindrome(s: &str)->bool{

    let cleaned:String=s.chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    cleaned == cleaned.chars().rev().collect::<String>()

}
fn main()
{
    let test_string="A man,a plan,a canal,panama";
    println!("{}",is_plaindrome(test_string));
}