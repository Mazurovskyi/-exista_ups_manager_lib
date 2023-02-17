use paho_mqtt::Message;
use std::{borrow::Cow, error::Error, sync::{Arc, Mutex, mpsc::Sender}};
use crate::app::data_structures::{Channel, RequestMsg};
use crate::app::config_data::*;
use std::process;

pub fn handler(msg: Option<Message>, mqtt_tx: Arc<Mutex<Sender<[u16; 4]>>>){

    if let Some(msg) = msg {
        match msg.topic() {
            "gateway/batteryInfo.req" => send_request(mqtt_tx, BATTERY_INFO_REQUEST),
            "connect" => send_request(mqtt_tx, CONNECT),
            _=> println!("We have received an mqtt massage on unexpected topic: {}",msg.topic())
        }
    }
    else{
        println!("We have received an mqtt massage. But empty payload!")
    }
}


fn send_request(mqtt_tx: Arc<Mutex<Sender<[u16; 4]>>>, request: &[[u16; 4]]){
    let ch_guardian = mqtt_tx.lock().unwrap();

    for data in request{
        if let Err(err) = ch_guardian.send(*data){
            println!("Error handling the mqtt message! Mqtt requests receiver has been dropped!");
            process::exit(0);
        }
    }

    drop(ch_guardian);   
}


