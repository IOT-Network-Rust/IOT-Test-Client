use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: usize = 9070;

const NAME: &str = "Big_Pack";
const ID: u32 = 12;

fn send_message(conn: &mut TcpStream, message: &Message) {
    conn.write_all(message.to_string().as_bytes())
        .expect("couldn't send message");
}
fn main() {
    // Connect to TCP Server
    println!("Connecting To Server...");
    let mut stream = TcpStream::connect(format!("{}:{}", SERVER_IP, SERVER_PORT))
        .expect("Could Not Connect To Server");
    println!("Connected To Server");

    // Sending a profile message
    let message = Message {
        message_type: MessageType::PROFILE,
        data: get_device_data(),
    };
    send_message(&mut stream, &message);

    // Sending an update message
    let message = Message {
        message_type: MessageType::UPDATE,
        data: r#"{
            "entries":[
                {"table":"BatteryVoltage","data":"4424246.2"},
                {"table":"SolarVoltage","data":"482424.5"}
                ]
            }"#
        .to_string(),
    };
    thread::sleep(Duration::from_secs(1));
    send_message(&mut stream, &message);

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
