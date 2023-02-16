mod msg;
mod reader;
mod sender;

extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use chrono::{offset::Utc, DateTime};

use std::io::{Write, Read};
use std::time::{Duration, SystemTime};
use std::{thread, error::Error};
use std::sync::{Arc, Mutex, mpsc};
use std:: {io, process};

use crate::app::data_structures::{Configuration, Channel, AppEntities, ModbusEnt, ModbusMsg, RequestMsg};

pub fn config(app_config: &Configuration)->Result<SystemPort, Box<dyn Error>>{

    let mut port = serial::open(app_config.modbus_port())?;

    //serial port settings
    port.reconfigure(&|port_config|{
        port_config.set_baud_rate(serial::Baud115200)?;
        port_config.set_char_size(serial::Bits8);
        port_config.set_parity(serial::ParityNone);
        port_config.set_stop_bits(serial::Stop1);
        port_config.set_flow_control(serial::FlowNone);

        Ok(())
    })?;

    //timeout listening the port
    port.set_timeout(Duration::from_millis(app_config.modbus_timeout()))?;    

    Ok(port)
}
    

pub fn run<'a>(modbus_entities: ModbusEnt<ModbusMsg<'a>, RequestMsg<'a>>)
->Result<(), Box<dyn Error>>{

    let share_port =  Arc::new(Mutex::new(modbus_entities.modbus_port()));
    let sender_port = Arc::clone(&share_port);
    let reader_port = Arc::clone(&share_port);

    let reader_transmit = modbus_entities.modbus_channel().get_transmitter()?;
    let sender_transmit = modbus_entities.modbus_channel().get_transmitter()?;
    let request_receiver = modbus_entities.request_channel();
    let timer_transmit = modbus_entities.timer_transmit();

    //let timer = move||{
    //  timer::run(timer_transmit)
    //}

    let reader = move||{
        reader::run(reader_port, reader_transmit)
    };

    let sender = move||{
        sender::run(sender_port, sender_transmit, request_receiver)
    };


    let read_handler = thread::spawn(reader);
    let send_handler = thread::spawn(sender);
    //let timer_handler = thread::spawn(timer);
    

    //send_handler.join().expect("SEND HANDLER ERROR");
    //read_handler.join().expect("READ HANDLER ERROR");
    //imer_handler.join().expect("TIMER HANDLER ERROR");
  

    for msg in modbus_entities.modbus_channel().recv(){
        println!("Received {0} bytes: {1:?}", msg.1, msg.0);
        match msg::parse(&msg.0, msg.1){
            Ok("battery_info_registers") => {println!("RECEIVED battery_info_registers: {:?}", msg.0)},
            Ok("firmware_version") => {println!("RECEIVED firmware_version: {:?}", msg.0)},
            Ok("hartbeat_response") => {println!("RECEIVED hartbeat_response: {:?}", msg.0)},
            Ok("event_msg")=> {println!("RECEIVED event_msg: {:?}", msg.0)}
            Err(err) => println!("MSG ERROR: {err}"),
            _=> {}
        }
    }
    
    
    Ok(())
}


