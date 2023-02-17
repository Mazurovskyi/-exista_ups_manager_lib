
use std::borrow::Borrow;
use std::error::Error;
use std::sync::{Arc, Mutex, mpsc, mpsc::{Receiver, Sender}};
use std::sync::mpsc::RecvError;
use std::thread::JoinHandle;

use serial::SystemPort;
use paho_mqtt::AsyncClient;
use paho_mqtt::ConnectOptions;

//use std::iter::Iterator;
//use itertools::Itertools;







pub type ModbusMsg<'a> = (&'a [u8], usize);
pub type RequestMsg<'a> = &'a [u16];







///Represent of one receiver and many transmitters 
///related with one mpsc channel
pub struct Channel<T: Send>{
    transmitters: Vec<Arc<Mutex<Sender<T>>>>,
    receiver: Receiver<T>
}

impl <T: Send> Channel<T>{

    ///Creates a new Channel with one Receiver and "tx_count" transmitters
    pub fn new(count: u8)->Self{

        let (tx,rx) = mpsc::channel::<T>();

        let tx = Arc::new(Mutex::new(tx));

        let tx_vect = vec![Arc::clone(&tx),];

        let mut channel = Channel {
            transmitters: tx_vect, 
            receiver: rx 
        };

        for _i in 0..count{
            channel.transmitters.push(Arc::clone(&tx));
        }

        channel
    }

    pub fn get_transmitter(&mut self)->Result<Arc<Mutex<Sender<T>>>, Box<dyn Error>>{
        self.transmitters.pop().ok_or(format!("All transmittes have been extracted!").into())
    }

    pub fn recv(&self)->Result<T, RecvError>{
        self.receiver.recv()
    }
}





pub struct AppEntities{
    mqtt_entities: MqttEntities,
    modbus_entities: ModbusEntities,
    modbus_channel: Channel<([u8; 8], usize)>
}

impl AppEntities{
    pub fn new(mqtt_entities: MqttEntities, modbus_entities: ModbusEntities, modbus_channel: Channel<([u8; 8], usize)>)->Self{
        AppEntities{
            mqtt_entities,
            modbus_entities,
            modbus_channel
        }
    }

    pub fn modbus_entities(&self)->&ModbusEntities{
        self.modbus_entities.borrow()
    }
    
    pub fn modbus_channel(&self)->&Channel<([u8; 8], usize)>{
        self.modbus_channel.borrow()
    }

    pub fn mqtt_entities(&self)->& MqttEntities{
        self.mqtt_entities.borrow()
    }
    

}





pub struct MqttEntities{
    client: AsyncClient,
    conn_opts: ConnectOptions
}
impl MqttEntities{
    pub fn new(client: AsyncClient, conn_opts: ConnectOptions)->Self{
        MqttEntities{
            client,
            conn_opts,
        }
    }


    pub fn client(&self)->&AsyncClient{
        self.client.borrow()
    }
    pub fn conn_opts(&self)->&ConnectOptions{
        self.conn_opts.borrow()
    }

}


pub struct ModbusEntities{
    read_handler: JoinHandle<()>,
    send_handler: JoinHandle<()>, 
    timer_handler: JoinHandle<()>
}
impl ModbusEntities{
    pub fn new(read_handler: JoinHandle<()>, send_handler: JoinHandle<()>, timer_handler: JoinHandle<()>)->Self{
        ModbusEntities{
            read_handler,
            send_handler, 
            timer_handler
        }
    }
}