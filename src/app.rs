pub mod data_structures;
pub mod config_data;
use data_structures::*;

use std::error::Error;

use std::sync::{Arc, Mutex, mpsc, mpsc::{Receiver, Sender}};

use crate::modbus;
use crate::mqtt;

use serial::SystemPort;
use paho_mqtt::AsyncClient;


pub fn config()->Result<AppEntities, Box<dyn Error>>{

    let mut request_channel: Channel<[u16; 4]> = Channel::new(2);
    let mut modbus_channel: Channel<([u8; 8], usize)> = Channel::new(2);

    let mqtt_tx = request_channel.get_transmitter()?;
    let reader_tx = modbus_channel.get_transmitter()?;
    let sender_tx = modbus_channel.get_transmitter()?;
    println!("mqtt config ...");
    let mqtt_entities = mqtt::config(mqtt_tx)?; // -> mqtt_client, connect_options
    println!("modbus config ...");
    let modbus_entities = modbus::config(request_channel, reader_tx, sender_tx)?; // -> read_handler, send_handler, timer_handler
    
    println!("return entities");
    Ok(AppEntities::new(mqtt_entities, modbus_entities, modbus_channel))
}





pub fn run(app_entities: AppEntities)->Result<(), Box<dyn Error>>{

    println!("Running mqtt..");
    if let Err(err) = mqtt::run(app_entities.mqtt_entities()){
        return Err(format!("error running mqtt: {err}").into())
    };

    println!("Running modbus..");
    if let Err(err) = modbus::run(app_entities.modbus_channel()){
        return Err(format!("error running modbus: {err}").into())
    };  

    Ok(())
}






