use std::io::{Read, Write};
use std::net::TcpStream;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: usize = 8080;

const NAME: &str = "Outdoor Light";
const ID: u32 = 245321;

fn main() {
    // Connect to TCP Server
    println!("Connection To Server");
    let mut stream = TcpStream::connect(format!("{}:{}", SERVER_IP, SERVER_PORT))
        .expect("Could Not Connect To Server");
    println!("Connection Success!");

    let message = Message {
        message_type:MessageType::CONNECT,
        data:get_device_data(),
    }.to_string();

    stream.write_all(message.as_bytes()).expect("couldn't send message");

    let message_two = Message {
        message_type:MessageType::RESPONSE,
        data:"Hi There Server".to_string(),
    }.to_string();
    stream.write_all(message_two.as_bytes()).expect("Failed To Send");
}

fn get_device_data() -> String {
    format!(
r#"{{
    "name":"{NAME}",
    "id":"{ID}",
    "sensors":[
            {{
                "label":"Battery Voltage",
                "data_type":"REAL"
            }}
    ],
    "inputs":[
        {{
            "label":"Flash",
            "data_type":"INTEGER"
        }}
    ]
}}"#
    )
}
#[derive(Debug)]
enum MessageType {
    PING,
    UPDATE,
    CONNECT,
    RESPONSE,
}
struct Message {
    message_type: MessageType,
    data: String,
}

impl Message {
    fn to_string(&self) -> String {
        format!("{:?}|{}|", self.message_type, self.data)
    }
}
