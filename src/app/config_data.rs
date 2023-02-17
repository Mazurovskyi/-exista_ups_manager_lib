pub const TOPICS: [&str; 2] = ["connect", "gateway/batteryInfo.req"];
pub const QOS:   &[i32]=     &[0, 0];  
pub const MQTT_VERSION: u32 = 0;
pub const CLIENT_ID: &str = "exista_ups_manager";
pub const HOST: &str = "127.0.0.1:1883";
pub const KEEP_ALIVE: u64 = 60;


pub const PORT: &str = "/dev/ttyUPS";
pub const TIMEOUT: u64 = 1;

const READ_DC_STATUS: [u16; 4] =      [0x11, 0x03, 0x17, 0x01];
const READ_CURRENT_VALUE: [u16; 4] =  [0x11, 0x03, 0x12, 0x01];
const READ_VOLTAGE: [u16; 4] =        [0x11, 0x03, 0x04, 0x01];
const READ_BATTERY_STATUS: [u16; 4] = [0x11, 0x03, 0x00, 0x01];
const READ_SOC: [u16; 4] =            [0x11, 0x03, 0x1C, 0x01];
const READ_SOH: [u16; 4] =            [0x11, 0x03, 0x1D, 0x01];
const READ_REMAIN_TIME: [u16; 4] =    [0x11, 0x03, 0x1A, 0x01];
const READ_BACKUP_TIME: [u16; 4] =    [0x11, 0x03, 0x1B, 0x01]; //same as REMAIN_TIME
const READ_MAX_AUTHONOMY_TIME: [u16; 4] = [0x11, 0x03, 0x20, 0x01];
const READ_CHARGING_STATUS: [u16; 4]= [0x11, 0x03, 0x19, 0x01];
const GET_SIGN: [u16; 4] =            [0x11, 0x03, 0x11, 0x01];

const GET_TEMPERATURE: [u16; 4] =     [0x01, 0x03, 0x55, 0x01];
const READ_FW_VERSION: [u16; 4] =     [0x01, 0x03, 0x00, 0x01];
const CUBE_POWER_RESET: [u16; 4] =    [0x01, 0x06, 0x1F, 0xAA55];

pub const HEARTBEAT: [u16; 4] =       [0x01, 0x06, 0x50, 0x00];

pub const BATTERY_INFO_REQUEST: &[[u16;4]] = &[READ_DC_STATUS, READ_CURRENT_VALUE, READ_VOLTAGE,
READ_BATTERY_STATUS, READ_SOC, READ_SOH, READ_REMAIN_TIME];

pub const CONNECT: &[[u16;4]] = &[READ_FW_VERSION];