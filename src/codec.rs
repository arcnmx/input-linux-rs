#![allow(missing_docs)]

use std::{io, mem, ptr};
use crate::sys::input_event;
#[cfg(feature = "bytes")]
use bytes::{BytesMut, BufMut};
use crate::InputEvent;

pub struct EventCodec {
    _dummy: (),
}

impl EventCodec {
    pub const fn new() -> Self {
        EventCodec {
            _dummy: (),
        }
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "bytes")))]
    pub fn decode_bytes(&mut self, src: &mut BytesMut) -> Result<Option<InputEvent>, io::Error> {
        if src.len() >= mem::size_of::<InputEvent>() {
            let src = src.split_to(mem::size_of::<InputEvent>());
            let event = unsafe {
                let mut event = mem::MaybeUninit::<input_event>::uninit();
                ptr::copy_nonoverlapping(src.as_ptr(), event.as_mut_ptr() as *mut u8, mem::size_of::<InputEvent>());
                InputEvent::from_raw(&*event.as_ptr()).map(|e| *e).map_err(From::from)
            };

            event.map(Some)
        } else {
            Ok(None)
        }
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "bytes")))]
    pub fn encode_bytes(&mut self, item: InputEvent, dst: &mut BytesMut) -> Result<(), io::Error> {
        dst.reserve(mem::size_of::<InputEvent>());
        dst.put_slice(item.as_bytes());
        Ok(())
    }
}

#[cfg(feature = "tokio-util-0_6")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "tokio-util-0_6")))]
mod tokio_util_impl_0_6 {
    use tokio_util_0_6::codec::{Encoder, Decoder};
    include!("tokio_impl.rs");
}

#[cfg(feature = "tokio-util-0_7")]
#[cfg_attr(feature = "dox", doc(cfg(feature = "tokio-util-0_7")))]
mod tokio_util_impl_0_7 {
    use tokio_util_0_7::codec::{Encoder, Decoder};
    include!("tokio_impl.rs");
}
