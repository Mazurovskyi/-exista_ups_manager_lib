extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

extern crate chrono;
use chrono::{offset::Utc, DateTime};

use std::time::{Duration, SystemTime};
use std::io::prelude::Read;
use std::{thread, error::Error};
use std::sync::{Arc, Mutex, mpsc};
use std:: {io, process};

pub fn run(reader_port: Arc<Mutex<SystemPort>>, tx: mpsc::Sender<[u8; 8]>){

    let mut buf: [u8; 8] = [0;8];
        
    loop{
        //println!("reading...");
        if let Ok(n) = reader_port.lock().unwrap().read(&mut buf){
            println!("Successfully read {n} bytes. Read: {buf:?}");
            tx.send(buf).unwrap();
            //clear(& mut buf);
            buf = [0;8];
        }
        //thread::sleep(Duration::from_millis(10))
    }
}

fn clear(buf: &mut [u8]){
    for el in buf.iter_mut(){
        *el = 0;
    }
}