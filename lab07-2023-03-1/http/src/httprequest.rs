#[derive(Debug, PartialEq)]
pub enum Method {
   Get,
   Post,
   Uninitialized,
}
impl From<&str> for Method {
    // ถ้า ที่รับมาจาก user ตรงกับ method ต่อไปนี้ เช่น รับ GET ก็ ตอบ GET รับ POST ก็ ตอบ POST อื่นๆ ตอบ Uninitialized
   fn from(s: &str) -> Method {
       match s {
           "GET" => Method::Get,
           "POST" => Method::Post,
           _ => Method::Uninitialized,
       }
   }
}

#[derive(Debug, PartialEq)]
pub enum Version {
   V1_1,
   V2_0,
   Uninitialized,
}
impl From<&str> for Version {
   fn from(s: &str) -> Version {
       match s {
           "HTTPS/1.1" => Version::V1_1,
           "HTTPS/2.0" => Version::V2_0,
           _ => Version::Uninitialized,
       }
   }
}
#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn test_version_into() {
       let m: Version = "HTTPS/2.0".into();
       assert_eq!(m, Version::V2_0);
   }

   #[test]
   fn test_method_into(){
        let k: Method = "POST".into();
        assert_eq!(k, Method::Post);
   }

}



// --------------------------------------------------------------------------------
// impl From<String> for HttpRequest {
//     fn from(req: String) -> Self {
//         let mut parsed_method = Method::Uninitialized;
//         let mut parsed_version = Version::V1_1;
//         let mut parsed_resource = Resource::Path("".to_string());
//         let mut parsed_headers = HashMap::new();
//         let mut parsed_msg_body = "";
//   // Parse the incoming HTTP request into HttpRequest struct
//         HttpRequest {
//             method: parsed_method, version: parsed_version,
//             resource: parsed_resource,headers: parsed_headers,
//             msg_body: parsed_msg_body.to_string(),
//         }
//     }
//  }

//    // Read each line in the incoming HTTP request
//    for line in req.lines() {
//     // If the line read is request line, call function process_req_line()
//     if line.contains("HTTP") {
//         let (method, resource, version) = process_req_line(line);
//         parsed_method = method; parsed_version = version;
//         parsed_resource = resource;
//     // If the line read is header line, call function process_header_line()
//     } else if line.contains(":") {
//         let (key, value) = process_header_line(line);
//         parsed_headers.insert(key, value);
//     //  If it is blank line, do nothing
//     } else if line.len() == 0 {// If none of these, treat it as message body
//     } else { parsed_msg_body = line; }
//     }


// fn process_header_line(s: &str) -> (String, String) {
//     // Parse the header line into words split by separator (':')
//     let mut header_items = s.split(":");
//     let mut key = String::from("");
//     let mut value = String::from("");
//     // Extract the key part of the header
//     if let Some(k) = header_items.next() {
//         key = k.to_string();
//     }
//     // Extract the value part of the header
//     if let Some(v) = header_items.next() {
//         value = v.to_string()
//     }
//     (key, value)
//  }

//  #[test]
//  fn test_read_http() {
//      let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
//      let mut headers_expected = HashMap::new();
//      headers_expected.insert("Host".into(), " localhost".into());
//      headers_expected.insert("Accept".into(), " */*".into());
//      headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
//      let req: HttpRequest = s.into();
//      assert_eq!(Method::Get, req.method); assert_eq!(Version::V1_1, req.version);
//      assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
//      assert_eq!(headers_expected, req.headers);
//  }

