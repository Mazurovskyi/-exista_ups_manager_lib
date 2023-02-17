mod msg;
mod reader;
mod sender;
mod timer;

extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use chrono::{offset::Utc, DateTime};

use std::io::{Write, Read};
use std::time::{Duration, SystemTime};
use std::{thread, error::Error};
use std::sync::{Arc, Mutex, mpsc};
use std:: {io, process};
use std::sync::mpsc::Sender;

use crate::app::config_data::*;
use crate::app::data_structures::{Channel, AppEntities, ModbusMsg, ModbusEntities};

pub fn config(
    mut request_channel: Channel<[u16; 4]>, 
    reader_tx: Arc<Mutex<Sender<([u8; 8], usize)>>>, 
    sender_tx: Arc<Mutex<Sender<([u8; 8], usize)>>>)
    ->Result<ModbusEntities, Box<dyn Error>>
    {
    println!("set the directory");
    let mut port = serial::open(PORT)?;

    println!("serial port settings");
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
    port.set_timeout(Duration::from_millis(TIMEOUT))?;    



    let share_port =  Arc::new(Mutex::new(port));
    let sender_port = Arc::clone(&share_port);
    let reader_port = Arc::clone(&share_port);


    let timer_tx = request_channel.get_transmitter()?;


    let timer = move||{
      timer::run(timer_tx)
    };

    let reader = move||{
        reader::run(reader_port, reader_tx)
    };

    let sender = move||{
        sender::run(sender_port, sender_tx, request_channel)
    };

    let read_handler = thread::spawn(reader);
    let send_handler = thread::spawn(sender);
    let timer_handler = thread::spawn(timer);

    println!("return modbus entities");
    Ok(ModbusEntities::new(read_handler, send_handler, timer_handler))
}
    

pub fn run(modbus_channel: &Channel<([u8; 8], usize)>)->Result<(), Box<dyn Error>>{

    
    //send_handler.join().expect("SEND HANDLER ERROR");
    //read_handler.join().expect("READ HANDLER ERROR");
    //imer_handler.join().expect("TIMER HANDLER ERROR");

    println!("Waiting message on modbus_rx...");
    //the corresponding Sender has disconnected, or it disconnects while this call is blocking
    while let Ok(msg) = modbus_channel.recv(){
        println!("Received {0} bytes: {1:?}", msg.1, msg.0);
        match msg::parse(&msg.0, msg.1){
            Ok("battery_info_registers") => {println!("RECEIVED battery_info_registers: {:?}", msg.0)},
            Ok("firmware_version") => {println!("RECEIVED firmware_version: {:?}", msg.0)},
            Ok("hartbeat_response") => {println!("RECEIVED hartbeat_response: {:?}", msg.0)},
            Ok("event_msg")=> {println!("RECEIVED event_msg: {:?}", msg.0)}
            Err(err) => println!("RECEIVED MSG ERROR: {err}"),
            _=> {}
        }
    }

    
    Ok(())
}


