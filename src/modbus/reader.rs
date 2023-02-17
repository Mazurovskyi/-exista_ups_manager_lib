extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use chrono::{offset::Utc, DateTime};

use std::time::{Duration, SystemTime};
use std::io::prelude::Read;
use std::{thread, error::Error};
use std::sync::{Arc, Mutex, mpsc::Sender};
use std:: {io, process};

use crate::app::data_structures::ModbusMsg;



pub fn run(reader_port: Arc<Mutex<SystemPort>>, reader_tx: Arc<Mutex<Sender<([u8; 8], usize)>>>){

    let mut buf: [u8; 8] = [0;8];
        
    loop{
        if let Ok(n) = reader_port.lock().unwrap().read(&mut buf){

            println!("Successfully read {n} bytes. Read: {buf:?}");
            reader_tx.lock().unwrap().send((buf, n)).unwrap();
            //buf = [0;8];

        }
        //println!("REDER WORKING...")
    }
}
