fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("ความยาวของ '{}' : {}",s1,len);
    change(&mut s1);
    println!("ข้อความที่เปลี่ยน: {}",s1);
    println!("ความยาวของ '{}' : {}",s1,calculate_length(&s1));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world");
}