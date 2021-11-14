use std::os::unix::fs::OpenOptionsExt;
use std::{fs::OpenOptions, io, thread, time::Duration};

use input_linux::{
    EventKind, EventTime, InputEvent, InputId, RelativeAxis, RelativeEvent,
    SynchronizeEvent, SynchronizeKind, UInputHandle,
};
use nix::libc::O_NONBLOCK;


// A rust translation of the uinput example available at
// https://docs.kernel.org/input/uinput.html#mouse-movements
// Creates a virtual mouse, moves it down and to the right 250 units
// in increments of 5 units
//
// This example requires either root (bad practice, too general) or
// the running user to be a member of the uinput group to actually
// make the mouse move
fn main() -> io::Result<()> {
    let uinput_file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(O_NONBLOCK)
        .open("/dev/uinput")?;
    let uhandle = UInputHandle::new(uinput_file);

    uhandle.set_evbit(EventKind::Key)?;
    uhandle.set_keybit(input_linux::Key::ButtonLeft)?;

    uhandle.set_evbit(EventKind::Relative)?;
    uhandle.set_relbit(RelativeAxis::X)?;
    uhandle.set_relbit(RelativeAxis::Y)?;

    let input_id = InputId {
        bustype: input_linux::sys::BUS_USB,
        vendor: 0x1234,
        product: 0x5678,
        version: 0,
    };
    let device_name = b"Example device";
    uhandle.create(&input_id, device_name, 0, &[])?;

    // This call to sleep was not necessary on my machine,
    // but this translation is meant to match exactly
    thread::sleep(Duration::from_secs(1));

    for _ in 0..50 {
        const ZERO: EventTime = EventTime::new(0, 0);
        let events = [
            *InputEvent::from(RelativeEvent::new(ZERO, RelativeAxis::X, 5)).as_raw(),
            *InputEvent::from(RelativeEvent::new(ZERO, RelativeAxis::Y, 5)).as_raw(),
            *InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw(),
        ];
        uhandle.write(&events)?;
        thread::sleep(Duration::from_micros(15_000));
    }

    // This call to sleep was not necessary on my machine,
    // but this translation is meant to match exactly
    thread::sleep(Duration::from_secs(1));
    uhandle.dev_destroy()?;

    Ok(())
}
