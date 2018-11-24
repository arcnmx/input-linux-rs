#![allow(missing_docs)]

extern crate tokio_io;
extern crate bytes;

use std::{io, mem, ptr, slice};
use sys::input_event;
use self::tokio_io::codec::{Decoder, Encoder};
use self::bytes::{BytesMut, BufMut};
use InputEvent;

pub struct EventCodec {
    _dummy: (),
}

impl EventCodec {
    pub fn new() -> Self {
        EventCodec {
            _dummy: (),
        }
    }
}

impl Decoder for EventCodec {
    type Item = InputEvent;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() >= mem::size_of::<Self::Item>() {
            let src = src.split_to(mem::size_of::<Self::Item>());
            let event = unsafe {
                let mut event: input_event = mem::uninitialized();
                ptr::copy_nonoverlapping(src.as_ptr(), &mut event as *mut _ as *mut u8, mem::size_of::<Self::Item>());
                InputEvent::from_raw(&event).map(|e| *e).map_err(From::from)
            };

            event.map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Encoder for EventCodec {
    type Item = InputEvent;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(mem::size_of::<Self::Item>());
        unsafe {
            dst.put_slice(slice::from_raw_parts(&item as *const _ as *const u8, mem::size_of::<Self::Item>()));
        }
        Ok(())
    }
}
