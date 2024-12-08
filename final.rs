fn main(){
    println!("ลองทำโปรเจกต์ขนาดเล็ก");

    println!("1.	สร้างโปรแกรมคำนวณเกรดของนักเรียน:");
    println!("   •	รับคะแนนจากผู้ใช้");
    println!("   •	แปลงคะแนนเป็นเกรด (A, B, C, D, F)");
    let score: i8 = 101;
    match grade(&score) {
        Err(e) => println!("{}", e),
        Ok(grade) => println!("คะแนนของนักเรียนคือ {} จะได้เกรด {}", score, grade),
    }
   


    println!("2.	สร้างโปรแกรมคำนวณพื้นที่รูปเรขาคณิต (เช่น สี่เหลี่ยม วงกลม และสามเหลี่ยม):");
    println!("   •	ใช้ Struct สำหรับเก็บข้อมูลรูปทรง");
    println!("   •	ใช้ Enum สำหรับเลือกประเภทของรูปทรง");
    let width:i32 = 40;
    let height:i32 = 50;

    let geometry:Geometry = Geometry::Cycle;

    let cycleResult = calculate_area(Geometry::Cycle,width,height);
    println!("ผลคำนวนของ วงกลมด้วยรัศมี {} คือ {}",width,cycleResult );

    let squareResult = calculate_area(Geometry::Square,width,height);
    println!("ผลคำนวนของ สี่เหลี่ยมด้วยกว้่างและสูง {} {} คือ {}",width,height,squareResult ); 

    let triangleResult = calculate_area(Geometry::Triangle,width,height);
    println!("ผลคำนวนของ สามเหลี่ยมด้วยกว้่างและสูง {} {} คือ {}",width,height,triangleResult ); 

}

fn calculate_area(geometry:Geometry,width:i32,height:i32) -> f32 {
    match geometry {
        Geometry::Square => (width * height) as f32,
        Geometry::Cycle =>  3.14 * (width * width) as f32,
        Geometry::Triangle => (width * height) as f32 / 2.0 
    }
}

enum Geometry {
    Square,
    Cycle,
    Triangle
}

struct Square {
   height: i32,
   width: i32
}

struct Cycle {
    radius:i32
}

struct Triangle {
    height: i32,
    width: i32
} 



fn grade(score: &i8) -> Result<String, String> {
    if *score < 0 || *score > 100 {
        Err(String::from("ไม่สามารถคำนวณเกรดได้เนื่องจากคะแนนผิดพลาด!!"))
    } else if *score >= 80 {
        Ok(String::from("A"))
    } else if *score >= 70 {
        Ok(String::from("B"))
    } else if *score >= 60 {
        Ok(String::from("C"))
    } else if *score >= 50 {
        Ok(String::from("D"))
    } else {
        Ok(String::from("F"))
    }
}