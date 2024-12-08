fn divide(a:f32,b:f32) -> Result<f64,String> {
    if b == 0.0 {
        Err(String::from("ไม่สามารถหารด้วย 0 ได้"))
    }else {
        Ok((a/b).into())
    }
}

fn main(){
    match divide(10.0,3.0){
        Ok(result) => println!("ผลลัพท์: {}",result),
        Err(e) => println!("ข้อผิดพลาด: {}",e)
    }
}