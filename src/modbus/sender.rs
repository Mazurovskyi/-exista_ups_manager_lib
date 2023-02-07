extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use chrono::{offset::Utc, DateTime};

use std::time::{Duration, SystemTime};
use std::io::prelude::Write;
use std::{thread, error::Error};
use std::sync::{Arc, Mutex, mpsc};
use std:: {io, process};



pub fn run(data: &[[u8;8]], sender_port: Arc<Mutex<SystemPort>>){

        for el in data{

            'sending: loop{
                if let Ok(n) = sender_port.lock().unwrap().write(el){
                    println!("Successfully write {n} bytes. Write: {el:?}");
                    break 'sending
                }
            }

            thread::sleep(Duration::from_millis(2000)); //Interval to separate modbus messages
        }
    
}