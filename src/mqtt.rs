mod msg;

use {paho_mqtt, paho_mqtt::Message};

use std::fmt::Display;
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::{env, process, sync::RwLock, thread, time::Duration};
use std::error::Error;

use paho_mqtt::{AsyncClient, connect_options, ConnectOptions};
//use crate::app::data_structures::{Configuration, Channel, AppEntities, RequestMsg};

use crate::app::config_data::*;
use crate::app::data_structures::*;


/// Doesn`t support reconnection if the failure connection occurs.
/// Reconnect only if the client loses the connection after existing connection.
pub fn config(mqtt_tx: Arc<Mutex<Sender<[u16; 4]>>>)-> Result<MqttEntities, Box<dyn Error>>{

    // client creation options
    let creation_options = paho_mqtt::CreateOptionsBuilder::new();
    println!("connection opt...");
    let creation_options = creation_options.server_uri(HOST)
        .client_id(CLIENT_ID)
        .user_data(Box::new(TOPICS))
        .mqtt_version(MQTT_VERSION)
        .finalize();

        println!("client new");
    // Create the new MQTT client based on creation options
    let client = paho_mqtt::AsyncClient::new(creation_options)?;

    // closure to be called when connection is established.
    client.set_connected_callback(|_cli: &paho_mqtt::AsyncClient| {
        println!("Connected.");
    });

    // closure to be called if the client loses the connection. Try to reconect
    client.set_connection_lost_callback(|client: &paho_mqtt::AsyncClient| {
        println!("Connection lost. Trying to reconnect.");
        thread::sleep(Duration::from_millis(1000));
        client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    });

    println!("set callback");
    // callback on incoming messages.
    client.set_message_callback(move |_client, msg: Option<Message>| msg::handler(msg, mqtt_tx.clone()));
    

    // client connection options. MQTT v3.x connection.
    let mut conn_opts = paho_mqtt::ConnectOptionsBuilder::new();
    
    let conn_opts = conn_opts
    .keep_alive_interval(Duration::from_secs(KEEP_ALIVE))
    .finalize();
    //.will_message(lwt);

    println!("return mqtt entities");
    Ok(MqttEntities::new(client, conn_opts))
//========================================================================================
    // connecting to the broker...
   // println!("Connecting to the MQTT server...");
    //client.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);

    //Ok(client)
//========================================================================================
}


    // Callback for a successful connection to the broker. Subscribe the topics
    fn on_connect_success(client: &paho_mqtt::AsyncClient, _msgid: u16){
        println!("Connection succeeded");
        client.subscribe_many(&TOPICS, QOS);
        println!("Subscribed to topics: {:?}", TOPICS);  
    }

    // Callback for a fail connection
    fn on_connect_failure(client: &paho_mqtt::AsyncClient, _msgid: u16, rc: i32){
            println!("Connection attempt failed with error code {}.\n", rc);
            thread::sleep(Duration::from_millis(1000));
            client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    }


pub fn run(mqtt_entities: &MqttEntities)->Result<(), Box<dyn Error>>{
    //client: AsyncClient, conn_opts: ConnectOptions

    println!("Connecting to the MQTT server...");
    mqtt_entities.client().connect_with_callbacks(mqtt_entities.conn_opts().clone(), on_connect_success, on_connect_failure);
    println!("COMEBACK MQTT YESSS");
    Ok(())
}