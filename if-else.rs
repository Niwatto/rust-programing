use std::io;

fn main() {
    println!("Conditional");
    let number = 6;

    if number % 2 == 0 {
        println!("เลข {} เป็นเลขคู่่",number);
    }else {
        println!("เลข {} เป็นเลขคี่",number);
    }


    println!("โจทย์ ");
    println!("	1.	เขียนโปรแกรมที่ตรวจสอบว่าเลขที่ผู้ใช้ป้อนเป็นเลขบวก เลขลบ หรือศูนย์");
    println!("	2.	แสดงผลว่าเลขนั้นอยู่ในหมวดหมู่ใด");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: i8 = input.trim().parse().unwrap();

    if number == 0{
        println!("ค่าที่รับเข้ามานั้น เป็น 0");
    } else if number > 0{
        println!("ค่าที่รับเข้ามานั้น เป็นค่าบวก");
    } else {
        println!("ค่าที่รับเข้ามานั้น เป็นค่าลบ"); 
    }


}