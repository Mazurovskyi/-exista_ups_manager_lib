pub mod data_structures;
use data_structures::*;

use std::error::Error;

use std::sync::{Arc, Mutex, mpsc, mpsc::{Receiver, Sender}};

use crate::modbus;
use crate::mqtt;

use serial::SystemPort;
use paho_mqtt::AsyncClient;


pub fn config<'a>(path: String)
->Result<AppEntities<ModbusMsg<'a>, RequestMsg<'a>>, Box<dyn Error>>{

    let app_config = match path.is_empty(){
        true => Configuration::new(None)?,
        false => Configuration::new(Some(&path))?,
    };


    let modbus_channel = Channel::<ModbusMsg>::new(2);
    let mut request_channel = Channel::<RequestMsg>::new(2);

    let mqtt_transimit = request_channel.get_transmitter()?;
    let timer_transmit = request_channel.get_transmitter()?;

    let port = modbus::config(&app_config)?;
    let mqtt_client = mqtt::config(&app_config, mqtt_transimit)?;
    

    let modbus_entities = ModbusEnt::new(port, modbus_channel, request_channel, timer_transmit);
    let mqtt_entities = MqttEnt::new(mqtt_client);
    
    
    Ok(AppEntities::<ModbusMsg<'a>, RequestMsg<'a>>::
    new(modbus_entities, mqtt_entities))
}


pub fn run<'a>(app_entities: AppEntities<ModbusMsg<'a>, RequestMsg<'a>>)
->Result<(), Box<dyn Error>>{

    if let Err(err) = modbus::run(app_entities.modbus_entities()){
        return Err(format!("error running modbus: {err}").into())
    };

    if let Err(err) = mqtt::run(app_entities.mqtt_entities()){
        return Err(format!("error running mqtt: {err}").into())
    };

    Ok(())
}






