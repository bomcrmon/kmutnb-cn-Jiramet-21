use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {   
//     // สร้าง connetction ที่จะต่อ server ip กับ port
//    if let Ok(stream) = TcpStream::connect("127.0.0.1:3000") {
//        println!("Connected to the server!");
//     // ถ้ายังไม่รัน tcpserver จะเข้า else
//    } else {
//        println!("Couldn't connect to server...");
//    }

    // 127.0.0.1 == localhost
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello Nirut".as_bytes()).unwrap();  // เปลี่ยน hello เป็น byte แล้วเขียนใน strem
    let mut buffer = [0; 10];            // สร้างกล่อง ขนาด 0-5
    stream.read(&mut buffer).unwrap();  // เอากล่องมารอรับ ค่าที่ server ตอบกลับมา
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );

}
