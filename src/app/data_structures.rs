
use std::error::Error;
use std::sync::{Arc, Mutex, mpsc, mpsc::{Receiver, Sender}};
use std::sync::mpsc::RecvError;

use serial::SystemPort;
use paho_mqtt::AsyncClient;

//use std::iter::Iterator;
//use itertools::Itertools;

const TOPICS: [&str; 2] = ["connect", "gateway/batteryInfo.req"];
const QOS:   &[i32]=     &[0, 0];  
const MQTT_VERSION: u32 = 0;
const CLIENT_ID: &str = "exista_ups_manager";
const HOST: &str = "127.0.0.1:1883";
const KEEP_ALIVE: u64 = 60;


const PORT: &str = "/dev/ttyUPS0";
const TIMEOUT: u64 = 1;

const READ_DC_STATUS: &[u16] =      &[0x11, 0x03, 0x17, 0x01];
const READ_CURRENT_VALUE: &[u16] =  &[0x11, 0x03, 0x12, 0x01];
const READ_VOLTAGE: &[u16] =        &[0x11, 0x03, 0x04, 0x01];
const READ_BATTERY_STATUS: &[u16] = &[0x11, 0x03, 0x00, 0x01];
const READ_SOC: &[u16] =            &[0x11, 0x03, 0x1C, 0x01];
const READ_SOH: &[u16] =            &[0x11, 0x03, 0x1D, 0x01];
const READ_REMAIN_TIME: &[u16] =    &[0x11, 0x03, 0x1A, 0x01];
const READ_BACKUP_TIME: &[u16] =    &[0x11, 0x03, 0x1B, 0x01]; //same as REMAIN_TIME
const READ_MAX_AUTHONOMY_TIME: &[u16] = &[0x11, 0x03, 0x20, 0x01];
const READ_CHARGING_STATUS: &[u16]= &[0x11, 0x03, 0x19, 0x01];
const GET_SIGN: &[u16] =            &[0x11, 0x03, 0x11, 0x01];

const GET_TEMPERATURE: &[u16] =     &[0x01, 0x03, 0x55, 0x01];
const READ_FW_VERSION: &[u16] =     &[0x01, 0x03, 0x00, 0x01];
const CUBE_POWER_RESET: &[u16] =    &[0x01, 0x06, 0x1F, 0xAA55];

const HEARTBEAT: &[u16] =           &[0x01, 0x06, 0x50, 0x00];

const BATTERY_INFO_REQUEST: &[&[u16]] = &[READ_DC_STATUS, READ_CURRENT_VALUE, READ_VOLTAGE,
READ_BATTERY_STATUS, READ_SOC, READ_SOH, READ_REMAIN_TIME];

const CONNECT: &[&[u16]] = &[READ_FW_VERSION];





pub type ModbusMsg<'a> = (&'a [u8], usize);
pub type RequestMsg<'a> = &'a [u16];







///Represent of one receiver and many transmitters 
///related with one mpsc channel
pub struct Channel<T>{
    transmitters: Vec<Arc<Mutex<Sender<T>>>>,
    receiver: Receiver<T>
}



impl <T: Send>Channel<T>{

    ///Creates a new Channel with one Receiver and "tx_count" transmitters
    pub fn new(count: u8)->Self{

        let (tx,rx) = mpsc::channel::<T>();

        let tx = Arc::new(Mutex::new(tx));
        let tx_vect = vec![tx,];

        let mut channel = Channel {
            transmitters: tx_vect, 
            receiver: rx 
        };

        for _i in 0..count{
            channel.add_transmitter(Arc::clone(&tx))
        }

        channel
    }

    pub fn add_transmitter(&mut self, tx: Arc<Mutex<Sender<T>>>){
        self.transmitters.push(tx)
    }

    pub fn get_transmitter(&mut self)->Result<Arc<Mutex<Sender<T>>>, Box<dyn Error>>{
        self.transmitters.pop().ok_or(format!("All transmittes have been extracted!").into())
    }

    pub fn recv(&self)->Result<T, RecvError>{
        self.receiver.recv()
    }
}

pub struct ModbusEnt<T,R>{
    modbus_port: SystemPort,
    modbus_channel: Channel<T>,
    request_channel: Channel<R>,
    timer_transmit: Arc<Mutex<Sender<R>>>
}

impl<T, R> ModbusEnt<T, R>{
    pub fn new(modbus_port: SystemPort, modbus_channel: Channel<T>,
    request_channel: Channel<R>, timer_transmit: Arc<Mutex<Sender<R>>>)->Self{
        ModbusEnt{
            modbus_port,
            modbus_channel,
            request_channel,
            timer_transmit,
        }
    }

    pub fn modbus_port(&self)->SystemPort{
        self.modbus_port
    }
    pub fn modbus_channel(&self)->Channel<T>{
        self.modbus_channel
    }
    pub fn request_channel(&self)->Channel<R>{
        self.request_channel
    }
    pub fn timer_transmit(&self)->Arc<Mutex<Sender<R>>>{
        self.timer_transmit
    }

}

pub struct MqttEnt{
    mqtt_client: AsyncClient,
}
impl MqttEnt{
    pub fn new(mqtt_client: AsyncClient)->Self{
        MqttEnt{
            mqtt_client,
        }
    }
}

pub struct AppEntities<T,R>{
    modbus_entities: ModbusEnt<T,R>,
    mqtt_entities: MqttEnt
}

impl <T, R> AppEntities <T, R>{
    pub fn new(modbus_entities: ModbusEnt<T, R>, mqtt_entities: MqttEnt)->Self{
        AppEntities{
            modbus_entities,
            mqtt_entities
        }
    }

    pub fn modbus_entities(&self)->ModbusEnt<T, R>{
        self.modbus_entities
    }

    pub fn mqtt_entities(&self)-> MqttEnt{
        self.mqtt_entities
    }
}

pub struct Configuration<'a>{
    modbus_port: &'a str,
    modbus_timeout: u64,
    mqtt_host: &'a str,
    mqtt_version: u32,
    mqtt_clientId: &'a str,
    mqtt_topics: &'a[&'a str],
    mqtt_qos: &'a [i32],
    mqtt_keep_alive: u64,
    battery_info_request: &'a [&'a[u16]],
    connect: &'a [&'a [u16]],
    heartbeat: &'a [u16]
}

impl <'a>Configuration<'a>{
    pub fn new(path: Option<&str>)->Result<Self, Box<dyn Error>>{

        if let Some(path) = path{
            return Err("Custom configuration doesn`t support yet.".into())
        }

        Ok(Configuration {
            modbus_port: PORT,
            modbus_timeout: TIMEOUT,
            mqtt_host: HOST,
            mqtt_version: MQTT_VERSION,
            mqtt_clientId: CLIENT_ID,
            mqtt_topics: &TOPICS,
            mqtt_qos: QOS,
            mqtt_keep_alive: KEEP_ALIVE,
            battery_info_request: BATTERY_INFO_REQUEST,
            connect: CONNECT,
            heartbeat: HEARTBEAT
        })
    }

    pub fn modbus_port(&self)->&str{
        self.modbus_port
    }

    pub fn modbus_timeout(&self)->u64{
        self.modbus_timeout
    }

    pub fn mqtt_host(&self)->&str{
        self.mqtt_host
    }

    pub fn mqtt_version(&self)->u32{
        self.mqtt_version
    }

    pub fn mqtt_clientId(&self)->&str{
        self.mqtt_clientId
    }

    pub fn mqtt_topics(&self)->&[&str]{
        self.mqtt_topics
    }

    pub fn mqtt_qos(&self)->&[i32]{
        self.mqtt_qos
    }
    pub fn mqtt_keep_alive(&self)->u64{
        self.mqtt_keep_alive
    }
    pub fn battery_info_request(&self)->&[&[u16]]{
        self.battery_info_request
    }
    pub fn connect(&self)->&[&[u16]]{
        self.connect
    }
    pub fn heartbeat(&self)->&[u16]{
        self.heartbeat
    }

}