use crate::{EventCodec, InputEvent};
use bytes::BytesMut;
use std::io::Error;

impl Decoder for EventCodec {
    type Item = InputEvent;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.decode_bytes(src)
    }
}

impl Encoder<InputEvent> for EventCodec {
    type Error = Error;

    fn encode(&mut self, item: InputEvent, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.encode_bytes(item, dst)
    }
}
