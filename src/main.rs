extern crate serial;
extern crate time;

use std::env;
use std::io;

use time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    for arg in env::args_os().skip(1) {
        println!("opening port: {:?}", arg);
        let mut port = serial::open(&arg).unwrap();

        interact(&mut port).unwrap();
    }
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    try!(port.configure(|settings| {
        settings.set_baud_rate(serial::Baud9600);
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
    }));

    port.set_timeout(Duration::milliseconds(25000));

    let mut buf: Vec<u8> = (0..255).collect();


    //println!("{:?} --- {:?}", buf, outbuf);

    loop {
        //println!("writing bytes");
        //try!(port.write(&b));

        println!("reading bytes");
        try!(port.read(&mut buf[..]));

        println!("{:?}", buf);
    }
    Ok(())
}