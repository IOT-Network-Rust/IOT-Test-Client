use std::io::{Read, Write};
use std::net::TcpStream; 

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: usize = 8080;

const NAME: &str = "Outdoor_Light";
const ID: u32 = 245321;

fn main() {
    // Connect to TCP Server
    println!("Connection To Server");
    let mut stream = TcpStream::connect(format!("{}:{}", SERVER_IP, SERVER_PORT))
        .expect("Could Not Connect To Server");
    println!("Connection Success!");

    let message = Message {
        message_type:MessageType::PROFILE,
        data:get_device_data(),
    }.to_string();

    stream.write_all(message.as_bytes()).expect("couldn't send message");

    let message_two = Message {
        message_type:MessageType::UPDATE,
        data:
        r#"{
            "entries":[
                {"table":"BatteryVoltage","data":"726.2"},
                {"table":"SolarVoltage","data":"63.5"},
                {"table":"WaterLevel","data":"343255"}
                ]
            }"#.to_string(),
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
                "label":"BatteryVoltage",
                "data_type":"REAL"
            }},
            {{
                "label":"SolarVoltage",
                "data_type":"REAL"
            }},
            {{
                "label":"WaterLevel",
                "data_type":"INTEGER"
            }}
    ],
    "inputs":[
        {{
            "label":"Flash",
            "data_type":"BLOB"
        }}
    ]
}}"#
    )
}
#[derive(Debug)]
enum MessageType {
    PING,
    UPDATE,
    PROFILE,
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
