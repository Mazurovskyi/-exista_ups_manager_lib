extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use chrono::{offset::Utc, DateTime};

use std::time::{Duration, SystemTime};
use std::io::prelude::{Write, Read};
use std::{thread, error::Error};
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver}};
use std:: {io, process};

use crate::app::data_structures::{Channel, ModbusMsg, RequestMsg};
use crate::modbus::msg;


pub fn run(sender_port: Arc<Mutex<SystemPort>>, sender_tx: Arc<Mutex<Sender<([u8; 8], usize)>>>, request_channel: Channel<[u16; 4]>){

    let mut buf: [u8; 8] = [0;8];


    for request in request_channel.recv(){

        let modbus_msg = msg::new(&request);

        let mut port_guard = sender_port.lock().unwrap();

        'sending: loop{
            if let Ok(n) = port_guard.write(&modbus_msg){
                println!("Successfully write {n} bytes. Write: {request:?}");
                break 'sending
            }
        }

        'reading: loop{
            if let Ok(n) = port_guard.read(&mut buf){
                println!("Successfully read {n} bytes. Read: {buf:?}");
                sender_tx.lock().unwrap().send((buf, n)).unwrap();
                //buf = [0;8];
                break 'reading
            }
            println!("REDER WORKING IN SENDER...")
        }

        // Drop and sleep may be skiped if the "read" method 
        // will stabillity wait a message 2+ millis.
        // If it will, the "send" operation will be fully athomary.

        drop(port_guard );
        thread::sleep(Duration::from_millis(2)); //Interval to separate modbus messages
    }

}