use hex;
use std::str;
use winapi::um::winuser::{ 
    RegisterHotKey, 
    GetMessageW, 
    SendInput, 
    KEYBDINPUT, 
    INPUT_KEYBOARD, 
    INPUT, 
    INPUT_u, 
    KEYEVENTF_EXTENDEDKEY 
};

const PLAY_PAUSE: u16 = 0xB3;
const NEXT_MEDIA: u16 = 0xB0;
const PREV_MEDIA: u16 = 0xB1;

fn send(key: u16) {
    let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };

    unsafe {
        *input_u.ki_mut() = KEYBDINPUT {
            wVk: key,
            dwExtraInfo: 0,
            wScan: 0x45,
            time: 0,
            dwFlags: KEYEVENTF_EXTENDEDKEY
        }
    }

    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u
    };

    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe { 
        SendInput(1, &mut input, ipsize);
    };
}

pub fn init_hotkeys() {

    unsafe {
        RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("pp"), 16).unwrap(), winapi::um::winuser::MOD_CONTROL as u32, 0x20);
        RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("ns"), 16).unwrap(), winapi::um::winuser::MOD_CONTROL as u32, 0x27);
        RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("ps"), 16).unwrap(), winapi::um::winuser::MOD_CONTROL as u32, 0x25);
    }

    let mut result;
    let mut message : winapi::um::winuser::MSG = unsafe { std::mem::zeroed() };

    while { result = unsafe { GetMessageW( &mut message, std::ptr::null_mut() , 0, 0 )} ; result != 0 } {
        let decode = &hex::decode(format!("{:X}", message.wParam as i32)).unwrap();
        let key = str::from_utf8(decode).unwrap();

        match key {
            "pp" => {
                send(PLAY_PAUSE);
            },
            "ns" => { 
                send(NEXT_MEDIA);
            },
            "ps" => {
                send(PREV_MEDIA);
            }
            _ => {

            }
        }
    }
}
