extern crate tokio_io;
extern crate bytes;

use std::{io, mem, ptr};
use sys::input_event;
use self::tokio_io::codec::Decoder;
use self::bytes::BytesMut;

pub struct EventDecoder {
    _dummy: (),
}

impl EventDecoder {
    pub fn new() -> Self {
        EventDecoder {
            _dummy: (),
        }
    }
}

impl Decoder for EventDecoder {
    type Item = input_event;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() >= mem::size_of::<Self::Item>() {
            let src = src.split_to(mem::size_of::<Self::Item>());
            let event = unsafe {
                let mut event = mem::uninitialized();
                ptr::copy_nonoverlapping(src.as_ptr(), &mut event as *mut _ as *mut u8, mem::size_of::<Self::Item>());
                event
            };

            Ok(Some(event))
        } else {
            Ok(None)
        }
    }
}
