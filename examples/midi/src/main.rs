use std::{io, sync::mpsc};

#[cfg(target_os = "linux")]
use makepad_widgets::{alsa_midi::AlsaMidiAccess, SignalToUI};
#[cfg(target_os = "macos")]
use makepad_widgets::{core_midi::CoreMidiAccess, SignalToUI};
#[cfg(target_os = "windows")]
use makepad_widgets::{winrt_midi::WinRTMidiAccess, SignalToUI};

fn main() {
    let signal = SignalToUI::new();
    let ma;
    #[cfg(target_os = "windows")]
    {
        signal.set();
        ma = WinRTMidiAccess::new(signal);
    }

    #[cfg(target_os = "linux")]
    {
        signal.set();
        ma = AlsaMidiAccess::new(signal);
    }

    #[cfg(target_os = "macos")]
    {
        signal.set();
        ma = CoreMidiAccess::new(signal);
    }

    let ports = ma.lock().unwrap().get_updated_descs();
    for p in ports.iter() {
        println!("> {} {:?} {:?}", p.name, p.port_id, p.port_type);
    }

    let stdin = io::stdin();
    for line in stdin.lines() {
        println!("{}", line.unwrap());
    }
}
