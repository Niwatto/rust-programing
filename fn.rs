

fn main() {
    great_user("Bigger");
    let sum = add(5,10);
    println!("ผลบวก: {}", sum);

    println!("=============== โจทย์");
    println!("สร้างฟังก์ชัน multiply ที่รับค่าตัวเลขสองตัวแล้วคืนค่าผลคูณ");
    println!("เรียกใช้ฟังก์ชัน multiply และแสดงผลลัพธ์"); 
    let a: i32 = 2;
    let b: i32 = 2;
    let result = multiply(a,b);
    println!("ผลคูณของ a ({}) และ b ({}) คือ {}",a,b,result);
}



fn great_user(name:&str){
    println!("สวัสดี, {}", name);
}

fn add(a:i8,b :i8) -> i8 {
    a+b
}

fn multiply(a:i32,b:i32) -> i32 {
    a * b
}
