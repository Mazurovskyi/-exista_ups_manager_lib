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


pub fn run(sender_port: Arc<Mutex<SystemPort>>, 
           sender_transmit: Arc<Mutex<Sender<ModbusMsg>>>, 
           request_receiver: Channel<RequestMsg>){

    let mut buf: [u8; 8] = [0;8];

    //waiting for mqtt request or hartbeat timer. loop.
    //Data to send takes from here and pushes into vector. In for loop vector will be iterating

    for request in request_receiver.recv(){

        let request = msg::new(request);

        let mut port_guard = sender_port.lock().unwrap();
        

        'sending: loop{
            if let Ok(n) = port_guard.write(&request){
                println!("Successfully write {n} bytes. Write: {request:?}");
                break 'sending
            }
        }

        'reading: loop{
            if let Ok(n) = port_guard.read(&mut buf){
                println!("Successfully read {n} bytes. Read: {buf:?}");
                sender_transmit.lock().unwrap().send((&buf, n)).unwrap();
                buf = [0;8];
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