mod msg;

use {paho_mqtt, paho_mqtt::Message};

use std::fmt::Display;
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::{env, process, sync::RwLock, thread, time::Duration};
use std::error::Error;

use paho_mqtt::AsyncClient;
use crate::app::data_structures::{Configuration, Channel, AppEntities, MqttEnt, RequestMsg};




/// Doesn`t support reconnection if the failure connection occurs.
/// Reconnect only if the client loses the connection after existing connection.
pub fn config(app_config: &Configuration, mqtt_transimit: Arc<Mutex<Sender<RequestMsg>>>)
-> Result<AsyncClient, Box<dyn Error>>{

    // Callback for a successful connection to the broker. Subscribe the topics
    let on_connect_success = |client: &paho_mqtt::AsyncClient, _msgid: u16| {
        println!("Connection succeeded");
        client.subscribe_many(app_config.mqtt_topics(), app_config.mqtt_qos());
        println!("Subscribed to topics: {:?}", app_config.mqtt_topics());  
    };

    // Callback for a fail connection
    let on_connect_failure = |client: &paho_mqtt::AsyncClient, _msgid: u16, rc: i32|{
            println!("Connection attempt failed with error code {}.\n", rc);
            //thread::sleep(Duration::from_millis(1000));
            //client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    };

    // client creation options
    let creation_options = paho_mqtt::CreateOptionsBuilder::new();
    
    let creation_options = creation_options.server_uri(app_config.mqtt_host())
        .client_id(app_config.mqtt_clientId())
        .user_data(Box::new(app_config.mqtt_topics()))
        .mqtt_version(app_config.mqtt_version())
        .finalize();

    // Create the new MQTT client based on creation options
    let client = paho_mqtt::AsyncClient::new(creation_options)?;

    // closure to be called when connection is established.
    client.set_connected_callback(|_cli: &paho_mqtt::AsyncClient| {
        println!("Connected.");
    });

    // closure to be called if the client loses the connection. Try to reconect
    client.set_connection_lost_callback(|client: &paho_mqtt::AsyncClient| {
        println!("Connection lost. Trying to reconnect.");
        thread::sleep(Duration::from_millis(2000));
        client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    });


    let callback = |_client, msg: Option<Message>|{
        if let Some(msg) = msg {

            let send_request = |request: &[&[u16]]|->Result<(), Box<dyn Error>>{
                let guard = mqtt_transimit.lock().unwrap();
        
                for data in request{
                    guard.send(data)?
                }
                Ok(())
            };
        
            let result = match msg.topic(){
                "gateway/batteryInfo.req" => {
                    send_request(app_config.battery_info_request())
                }
                "connect" => {
                    send_request(app_config.connect())
                }
                topic => {
                    eprintln!("Unknown topic message have received: {topic}");
                    Ok(())
                }
            };

            if let Err(err) = result{
                 println!("Receiver of \"request_channel\" crashed before mqtt still working!");
                 process::exit(0)
            }
        }
        else{
            println!("We have received an mqtt massage. But empty payload!")
        }
    };




    // callback on incoming messages.
    client.set_message_callback(callback);
    

    // client connection options. MQTT v3.x connection.
    let mut conn_opts = paho_mqtt::ConnectOptionsBuilder::new();

    let conn_opts = conn_opts
    .keep_alive_interval(Duration::from_secs(app_config.mqtt_keep_alive()))
    .finalize();
    //.will_message(lwt);

    
    // connecting to the broker...
    println!("Connecting to the MQTT server...");
    client.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);

    Ok(client)
}







pub fn run(mqtt_entities: MqttEnt)->Result<(), Box<dyn Error>>{
    loop {
        thread::sleep(Duration::from_millis(2000));
    }
}