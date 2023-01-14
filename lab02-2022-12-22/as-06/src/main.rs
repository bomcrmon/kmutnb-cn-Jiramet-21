use std::io;
fn main() {
    
    println!("Enter the array elements, separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut a: Vec<i32> = input
        .split_whitespace()             //แยกแต่ละเลขด้วยช่องว่าง
        .map(|s| s.parse().unwrap())    //เอาแต่ละเลขที่ได้นำมาแปลงจาก String เป็น int
        .collect();                     //นำค่าที่ได้คืน a
        
    println!("Input : arr[] = {:?}",a);
    for i in 0..a.len()-1{
        if a[i] == 0 {
            let mut num_idx = i + 1;
            while a[num_idx] == 0{
                if num_idx < a.len()-1{
                   num_idx += 1; 
                }else{
                    break;
                }
            }
            a[i] = a[num_idx];
            a[num_idx] = 0;
        }

    }
    
    
    
    println!("Output : arr[] = {:?}",a);
}