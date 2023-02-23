fn main() {
    // A
    let s = "this cat this bat this rat";

    // step 1 word
    let arr: Vec<&str> = s.split(' ').collect();

    // step 2 unique
    let arr = step2(arr);

    // step 3 count
    let size = count(arr);

    //B
    println!("{}",size);
}

fn count(arr: Vec<&str>) -> u8 {
    let si:u8 = arr.len().try_into().unwrap();
    return si;
}


fn step2(mut arr: Vec<&str>) -> Vec<&str> {
    let mut i = 0;
    while i<arr.len(){  
        for j in 0..i{
            if arr[i] == arr[j] {
                arr.remove(i);
            } 
        }
        i += 1;     
    }
    arr     // ก็คือ retuen เหมือนกัน
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_work() {
        let s = "this cat this bat this rat";
        let result: Vec<&str> = s.split(' ').collect();
        let result = step2(result);
        let end = count(result);
        assert_eq!(end, 4);
    }
}