// Remove warning on flexbuffers::FlexBufferType::VectorString match arm
#![allow(deprecated)]

extern crate flexbuffers;

use flexbuffers::Reader;
use futures::executor::block_on;
use std::{io, process, time::Duration};
use clap::{App, Arg};

use paho_mqtt as mqtt;

fn parse_data(data: &[u8]) {
    let root = Reader::get_root(data).unwrap();
    let buf_map = root.as_map();

    let keys = buf_map.keys_vector();

    for num in 0..keys.len() {
        
        let key_name = keys.idx(num).as_str();
        let field = buf_map.idx(key_name);

        match field.flexbuffer_type() {
            flexbuffers::FlexBufferType::Null => {}
            flexbuffers::FlexBufferType::Int => {
                println!("Key {:?} - value {:?}", key_name, field.as_i32());
            }
            flexbuffers::FlexBufferType::UInt => {
                println!("Key {:?} - value {:?}", key_name, field.as_u32());
            }
            flexbuffers::FlexBufferType::Float => {
                println!("Key {:?} - value {:?}", key_name, field.as_f32());
            }
            flexbuffers::FlexBufferType::Bool => {
                println!("Key {:?} - value {:?}", key_name, field.as_bool());
            }
            flexbuffers::FlexBufferType::Key => {}
            flexbuffers::FlexBufferType::String => {
                println!("Key {:?} - value {:?}", key_name, field.as_str());
            }
            flexbuffers::FlexBufferType::IndirectInt => {}
            flexbuffers::FlexBufferType::IndirectUInt => {}
            flexbuffers::FlexBufferType::IndirectFloat => {}
            flexbuffers::FlexBufferType::Map => {}
            flexbuffers::FlexBufferType::Vector => {}
            flexbuffers::FlexBufferType::VectorInt => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_i32());
                }
            }
            flexbuffers::FlexBufferType::VectorUInt => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_u32());
                }
            }
            flexbuffers::FlexBufferType::VectorFloat => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_f32());
                }
            }
            flexbuffers::FlexBufferType::VectorKey => {}
            flexbuffers::FlexBufferType::VectorString => {}
            flexbuffers::FlexBufferType::VectorBool => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_bool());
                }
            }
            flexbuffers::FlexBufferType::VectorInt2 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_i32());
                }
            }
            flexbuffers::FlexBufferType::VectorUInt2 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_u32());
                }
            }
            flexbuffers::FlexBufferType::VectorFloat2 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_f32());
                }
            }
            flexbuffers::FlexBufferType::VectorInt3 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_i32());
                }
            }
            flexbuffers::FlexBufferType::VectorUInt3 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_u32());
                }
            }
            flexbuffers::FlexBufferType::VectorFloat3 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_f32());
                }
            }
            flexbuffers::FlexBufferType::VectorInt4 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_i32());
                }
            }
            flexbuffers::FlexBufferType::VectorUInt4 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_u32());
                }
            }
            flexbuffers::FlexBufferType::VectorFloat4 => {
                println!("Key {:?}", key_name);
                let vec = field.as_vector();
                for num in 0..vec.len() {
                    println!("Value {:?}", vec.idx(num).as_f32());
                }
            }
            flexbuffers::FlexBufferType::Blob => {}
        }
    }
}

fn main() {
    let matches = App::new("Ditto Client")
    .version("0.1.0")
    .about("Receive and parse Ditto output")
    .arg(Arg::new("address")
        .short('a')
        .long("address")
        .about("Sets the MQTT broker address")
        .takes_value(true)
        .default_value("127.0.0.1"))
    .arg(Arg::new("topic")
        .short('t')
        .long("topic")
        .about("Sets the topic to subscribe")
        .takes_value(true)
        .default_value("TestTopic"))
    .get_matches();

    let address = matches.value_of("address").unwrap();
    let topic = matches.value_of("topic").unwrap();

    // Create the client.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(address)
        .client_id("")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(true)
            .finalize();

        // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", topic);
        cli.subscribe(topic, 0).await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        cli.set_message_callback(|_cli, msg| {
            if let Some(msg) = msg {
                parse_data(msg.payload());
            }
        });

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }

    io::stdin().read_line(&mut String::new()).unwrap();

    cli.unsubscribe(topic);
    cli.disconnect(None);
}
