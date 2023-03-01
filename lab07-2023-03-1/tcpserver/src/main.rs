use std::net::TcpListener;
use std::io::{ Read, Write };
//use std::http::*;

fn main() {
    // connection_listener ทำการผูก ip กับ port **127.0.0.1 คือ loopback
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    // วนที่อ่านเข้ามาจาก connection_listener  ใส่ตัวแปร stream ** วนไปเรื่อยๆ ||.incoming() คือเข้ามาได้เลย
    for stream in connection_listener.incoming(){
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 12];         // สร้างกล่องมา มีขนาด 0-1024
        stream.read(&mut buffer).unwrap();  // อ่านค่าที่รับมาทีละช่อง
        stream.write(&mut buffer).unwrap(); // เขียนค่าที่รับมาทีละช่อง
        // โชว์ว่า client ส่งไรมา ** เอาค่า buffer มาเปลี่ยนเป็น Str แล้วโชว์  ** ถ้าเกินเหลือจะเป็น 0 
 	    println!("Got request from client:{:?}", std::str::from_utf8(&buffer).unwrap());
    }
}
