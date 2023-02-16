use paho_mqtt::Message;
use std::{borrow::Cow, error::Error, sync::{Arc, Mutex, mpsc::Sender}};
use crate::app::data_structures::{Channel, RequestMsg};


pub fn handler(msg: Message, mqtt_tx: Arc<Mutex<Sender<RequestMsg>>>)->Result<(), Box<dyn Error>>{
    Ok(())
    /*
    let payload_str = match msg.payload_str(){ 
        Cow::Borrowed(payload_str) => payload_str.to_string(),
        Cow::Owned(payload_str) => {
            println!("Payload is in non-UTF-8 format! 
            Paylodad was replaced by any invalid UTF-8 sequences with U+FFFD REPLACEMENT CHARACTER");
            payload_str
        },
    };   

    println!("Payload: {payload_str}\n")
    */    

}


