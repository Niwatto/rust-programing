fn main() {
    println!("ตัวอย่างการใช้ตัวแปร immutable และ mutable");
    let x = 5;
    println!("ค่าของ x คือ: {}", x);

    let mut y = 10;
    y += 20;
    println!("ค่าของ y คือ: {}", y);

    let is_active = true;
    let letter: char = 'A';
    println!("สถานะของ: {}, ตัวอักษรคือ: {}", is_active, letter);


    println!("โจทย์ข้อที่ 1: เขียนโปรแกรมที่สร้างตัวแปรชื่อ a และ b โดย:");
    println!("a เป็นตัวแปร immutable ที่มีค่าเริ่มต้นเป็น 15");
    println!("b เป็นตัวแปร mutable ที่เริ่มต้นด้วยค่า 20 และเพิ่มค่า 10");
    let a = 15;
    let mut b = 20;
    b += 10;
    
    println!("แสดง a: {} , b: {}",a,b)
}
