#[macro_use]
mod debug;

use std::ffi::CStr;
use std::mem::{drop, transmute};
use std::net::UdpSocket;
use std::os::raw::c_char;
use std::ptr;
use std::result::Result;

pub struct Baton {
    socket: UdpSocket,
    server_url: String,
}

impl<'a> Baton {
    fn to_ptr(self) -> *mut Baton {
        unsafe { transmute(Box::new(self)) }
    }

    fn from_ptr(ptr: *mut Baton) -> &'a mut Baton {
        unsafe { &mut *ptr }
    }

    fn connect(url: &str) -> Result<Baton, String> {
        debug!("Connecting to {:}...", url);

        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => socket,
            Err(error) => return Err(format!("{:}", error)),
        };

        Ok(Baton { socket: socket, server_url: String::from(url) })
    }

    fn disconnect(ptr: *mut *mut Baton) {
        let baton: Box<Baton> = unsafe { transmute(*ptr) };

        drop(baton);
    }

    fn send_ding(&mut self) -> Result<(), String> {
        match self.socket.send_to(&[0; 10], &self.server_url) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("{:}", error)),
        }
    }
}

#[no_mangle]
pub extern "C" fn connect_to_server(ptr: *mut *const Baton, url: *const c_char) -> bool {
    let url = unsafe { CStr::from_ptr(url) };
    let url_str = match url.to_str() {
        Ok(slice) => slice,
        Err(_) => {
            debug!("Invalid UTF8 in URL.");
            return false;
        }
    };

    match Baton::connect(url_str) {
        Ok(baton) => {
            debug!("Connected.");

            unsafe {
                *ptr = baton.to_ptr();
            }

            true
        }
        Err(message) => {
            debug!("Failed to connect: {:}", message);

            unsafe {
                *ptr = ptr::null();
            }

            false
        }
    }
}

#[no_mangle]
pub extern "C" fn disconnect_from_server(ptr: *mut *mut Baton) {
    if !ptr.is_null() && unsafe { !(*ptr).is_null() } {
        Baton::disconnect(ptr);

        debug!("Disconnected.");

        unsafe {
            *ptr = ptr::null_mut();
        }
    }
}


#[no_mangle]
pub extern "C" fn send_ding(ptr: *mut Baton) -> bool {

	let mut num  ;

	let stringVar = "Hi";

	num = 5;

	num = 7;

	//stringVar = num;


    if !ptr.is_null() {
        match Baton::from_ptr(ptr).send_ding() {
            Ok(_) => true,
            Err(message) => {
                debug!("Error while sending: {:}", message);

                false
            }
        }
    } else {
        false
    }

	
}

#[no_mangle]
pub extern "C" fn processHeavy() -> f64{
   
	let mut total : f64 = 0.0;
	for elem in 0..1000000 {
		let dividend = 1.0 / elem as f64;

		total += dividend;
	}

	total
}

#[test]
pub fn TestThing()
{

}
