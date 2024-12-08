fn main(){
    let mut count = 0;
    // infinite loop
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("จบลูป loop เมื่อ count = {}",count);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("Countdown: {}", number);
        number -= 1;
    }

    // for loop
    for i in 1..5 {
        println!("ตัวเลขในลูป for: {}",i);
    }


    println!("โจทย์");
    println!("	1.	เขียนโปรแกรมที่นับเลขจาก 1 ถึง 10 โดยใช้ลูป for");
    println!("	2.	เขียนโปรแกรมที่นับถอยหลังจาก 10 ถึง 1 โดยใช้ลูป while");

    for i in 1..11 {
        println!("นับเลข -> {}",i);
    }
    let mut count_down = 10;
    while count_down != 0 {
        println!("นับเลขถอยหลัง -> {}",count_down);
        count_down -= 1;
    }

}