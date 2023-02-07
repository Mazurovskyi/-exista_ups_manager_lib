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

    
pub fn config(path: &str, timeout: u64)->Result<SystemPort, Box<dyn Error>>{

    let mut port = serial::open(path)?;

    port.reconfigure(&|config|{
        config.set_baud_rate(serial::Baud115200)?;
        config.set_char_size(serial::Bits8);
        config.set_parity(serial::ParityNone);
        config.set_stop_bits(serial::Stop1);
        config.set_flow_control(serial::FlowNone);

        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(timeout))?;    //timeout listening the port

    Ok(port)
}
    

pub fn run(mut port: SystemPort, rev: u8)->Result<(), Box<dyn Error>>{
    /*
    let _msg = match type_msg{
        "inv_get_current_lifo" =>if rev == 1{[119, 63, 255, 207, 255, 127, 100, 249]}else {[119, 63, 255, 207, 255, 127, 155, 6]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "inv_handshake_lifo" =>  if rev == 1{[127, 159, 255, 179, 255, 255, 145, 219]}else {[127, 159, 255, 179, 255, 255, 110, 36]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),
        
        "inv_get_current_lifo_rev" =>if rev == 1{[249, 100, 127, 255, 207, 255, 63, 119]}else {[6, 155, 127, 255, 207, 255,63, 119]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "inv_handshake_lifo_rev" =>  if rev == 1{[219, 145, 255, 255, 179, 255, 159, 127]}else {[36, 110, 255, 255, 179, 255, 159, 127]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),
        
        //==============================

        "get_current_lifo" =>if rev == 1{[136, 192, 0, 72, 0, 128, 155, 6]}else {[136, 192, 0, 72, 0, 128, 100, 249]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "handshake_lifo" =>  if rev == 1{[128, 96, 0, 76, 0, 0, 110, 36]}else {[128, 96, 0, 76, 0, 0, 145, 219]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),
        
        "get_current_lifo_rev" =>if rev == 1{[6, 155, 128, 0, 72, 0, 192, 136]}else {[249, 100, 128, 0, 72, 0, 192, 136]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "handshake_lifo_rev" =>  if rev == 1{[36, 110, 0, 0, 76, 0, 96, 128]}else {[219, 145, 0, 0, 76, 0, 96, 128]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),

        //============ORIG===============
        //fifo 0
        "get_current_fifo" =>if rev == 1{[17, 3, 0, 18, 0, 1, 217, 96]}else {[17, 3, 0, 18, 0, 1, 38, 159]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "handshake_fifo" =>  if rev == 1{[1, 6, 0, 80, 0, 0, 118, 36]}else {[1, 6, 0, 80, 0, 0, 137, 219]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),
        
        "get_current_fifo_rev" =>if rev == 1{[96, 217, 1, 0, 18, 0, 3, 17]}else {[159, 38,  1, 0, 18, 0, 3, 17]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "handshake_fifo_rev" =>  if rev == 1{[36, 118, 0, 0, 80, 0, 6, 1]}else {[219, 137, 0, 0, 80, 0, 6, 1]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),

        //==============================

        "inv_get_current_fifo" =>if rev == 1{[238, 252, 255, 237, 255, 254, 38, 159]}else {[238, 252, 255, 237, 255, 254, 217, 96]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "inv_handshake_fifo" =>  if rev == 1{[254, 249, 255, 175, 255, 255, 137, 219]}else {[254, 249, 255, 175, 255, 255, 118, 36]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),
        
        "inv_get_current_fifo_rev" =>if rev == 1{[159, 38, 254, 255, 237, 255, 252, 238]}else {[96, 217, 254, 255, 237, 255, 252, 238]},      //msg::new(0x11, 0x03, 0x12, 0x01, rev = 1),
        "inv_handshake_fifo_rev" =>  if rev == 1{[219, 137, 255, 255, 175, 255, 249, 254]}else {[36, 118, 255, 255, 175, 255, 249, 254]},      //msg::new(0x01, 0x06, 0x50, 0x00, rev = 1),
        
        _=> return Err("There are no such message type!".into())
    };
    */
    
    

    let read_dc_status = msg::new(0x11, 0x03, 0x17, 0x01, rev);
    let read_current_value = msg::new(0x11, 0x03, 0x12, 0x01, rev);
    let read_voltage = msg::new(0x11, 0x03, 0x04, 0x01, rev);
    let read_battery_status = msg::new(0x11, 0x03, 0x00, 0x01, rev);
    let read_soc = msg::new(0x11, 0x03, 0x1C, 0x01, rev);
    let read_soh = msg::new(0x11, 0x03, 0x1D, 0x01, rev);
    let read_remain_time = msg::new(0x11, 0x03, 0x1A, 0x01, rev);

    let data = [read_dc_status, read_current_value, read_voltage,
    read_battery_status, read_soc, read_soh, read_remain_time];

    let mut buf: [u8;8] = [0;8];

    for el in data{
        'sending: loop{
            if let Ok(n) = port.write(&el){
                println!("Successfully write {n} bytes. Write: {el:?}");
                break 'sending
            }
        }
        
        if let Ok(n) = port.read(&mut buf){
            println!("Successfully read {n} bytes. Read: {buf:?}");
        }
        else{
            println!("timeout is over reading the buffer");
        }
        thread::sleep(Duration::from_millis(2000))
    }

    /*
    
    let share_port = Arc::new(Mutex::new(port));
    let sender_port = Arc::clone(&share_port);
    let reader_port = Arc::clone(&share_port);

    let (tx, rx) = mpsc::channel::<[u8;8]>();


    let sender = move||{
        sender::run(&data[..], sender_port)
    };

    let reader = move||{
        reader::run(reader_port, tx)
    };

    let send_handler = thread::spawn(sender);
    let read_handler = thread::spawn(reader);

    send_handler.join();
    //read_handler.join();

    'receive: loop{
        let received = rx.recv()?;
        //println!("Received {0} bytes: {received:?}", received.len());
    }
    
    */
    
    
    Ok(())
}


