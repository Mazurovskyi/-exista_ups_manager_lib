extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};
use std::io::prelude::{Read, Write, Seek};

use std::error::Error;
use std::io::ErrorKind;

pub fn parse(msg: &[u8], len: usize)->Result<&str, Box<dyn Error>>{

    if len < 4{
        return Err(format!("Message {msg:?} len is less than 4. Message error.").into())
    }

    // msg = battery_info_registers (addr 0x11, func 0x03) | firmware_version (addr 0x01, func 0x03)| 
    // hartbeat_response (addr 0x01, func 0x06)| event_msg (addr 0x00, func 0x64)

    match &msg[0..2]{
        [0x11, 0x03] => Ok("battery_info_registers"),
        [0x01, 0x03] => Ok("firmware_version"),
        [0x01, 0x06] => Ok("hartbeat_response"),
        [0x00, 0x64] => Ok("event_msg"),
        _=> Err(format!("incorrect_msg: {:?}",msg).into())
    }
}


pub fn new(data: &[u16])->[u8; 8]{

    let addr = data[0] as u8;
    let code = data[1] as u8;
    let (offset_h, offset_l) = into_8_bit(data[2] as u16);
    let (count_h, count_l) =  into_8_bit(data[3] as u16);


    let msg = [addr, code, offset_h, offset_l, count_h, count_l];

    let (crc_h, crc_l) = into_8_bit(crc(&msg));

    [addr, code, offset_h, offset_l, count_h, count_l, crc_l, crc_h]
}


fn crc(data: &[u8])->u16{
    let table:[u16;2] = [ 0x0000, 0xA001];
    let mut crc = 0xFFFF as u16;
    let mut xor = 0;

    for el in data{
        crc^=*el as u16;
        for _ in 0..8{
            xor = crc & 0x01;
            crc>>=1;
            crc^=table[xor as usize]
        }
    }
    crc
}


fn into_8_bit(val: u16)->(u8,u8){
    let high = ((val & (0xFF00 as u16)) >> 8) as u8;
    let low = (val & (0x00FF as u16)) as u8;
    (high, low)
}


//============================== TESTS ==============================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_conv() {
        let x: u16 = 0x17;
        dbg!((0xFFFF as u16) ^ (0x17 as u16));
        assert_eq!(into_8_bit(x), (0x00 as u8, 0x17 as u8))
    }

    #[test]
    fn check_reverse(){

        let get_current =    new(&[0x11, 0x03, 0x12, 0x01]);
        let handshake =      new(&[0x01, 0x06, 0x50, 0x00]);

        println!("get_current: {get_current:?}");
        println!("handshake:   {handshake:?}");
        assert_eq!(handshake, get_current);
        //get_current: [17, 3, 0, 18, 0, 1, 254, 255]
        //handshake:   [1, 6, 0, 80, 0, 0, 168, 255]
    }
}

//Successfully read 7 bytes. Read: [17, 3, 2, 1, 231, 56, 93, 0]
//Successfully read 7 bytes. Read: [17, 3, 2, 0, 6, 249, 133, 0]
//Successfully read 4 bytes. Read: [1, 6, 128, 34, 0, 0, 0, 0]





//============================== DRAFT ==============================

fn _crc_not_works(data: &[u8], rev: u8)->u16{

    let mut crc:u16 = 0xFFFF;

    for el in data.iter(){

        crc^= *el as u16;

        for _i in 8..0{
            if (crc & 0x0001) != 0{
                crc >>= 1;
                crc ^= 0xA001;
            }
            else{
                crc >>=1;
            }
        }
    }
    if rev == 1{
        return !crc
    }
    println!("{crc}");
    crc
    
}

pub fn send(msg: &[u8], port: &mut SystemPort)->Result<(), Box<dyn Error>>{
    
    match port.write(msg){
        Ok(n) if n  == msg.len() => {
            println!("Query has transmitted successfully. Send {n} bytes.");
            Ok(()) 
        },
        Ok(n) if n < msg.len() => {
            println!("The number of bytes sent is less than the query length. 
            Send {n} bytes. Query len: {}",msg.len());
            Ok(()) 
        },
        Ok(0)=> {
            println!("Underlying object is no longer able to accept bytes and will likely 
            not be able to in the future as well, or the provided buffer is empty");
            Ok(()) 
        },
        Ok(n)=> {
            println!("The number of bytes sent is more than the request length. 
            Send {n} bytes. Request len: {}", msg.len());
            Ok(()) 
        },
        Err(err)=>{println!("Error sending modbus message: {err}"); return Err(Box::new(err))
        }
    }
}

pub fn get(port: &mut SystemPort)->Result<[u8;8], Box<dyn Error>>{
    let mut buf: [u8;8] = [0;8];

    loop{
        match port.read(&mut buf){
            Err(_) => continue,
    
            Ok(n) if (n != 0) =>{
                println!("Data was read from buffer succsessfully. Get: {n} bytes.\n");
                // send buf via channel
            }
            Ok(_) => {
                println!("0 (zero) bytes have received from buffer. 
                There are no avaliable data in buffer or the connection was shut down correctly\n");
            }
        }
    }
        
}
