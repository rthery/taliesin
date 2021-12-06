use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::thread;
use std::time::Instant;

use clap::{App, Arg, crate_authors, crate_version};
use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::{Decoder, OutputStream, Sink};

const DISABLED_TIMER: u128 = u128::MAX;

fn main() {
    let matches = App::new("taliesin")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Small application to play sound after pressing a key with an optional delay")
        .arg(Arg::new("KEYS").short('k').long("keys").about("Key(s) to trigger sound").required(true).multiple_values(true).takes_value(true))
        .arg(Arg::new("FILE").short('f').long("file").about("Path to sound file").required(true).takes_value(true))
        .arg(Arg::new("DELAY").short('d').long("delay").about("Delay in milliseconds before playing the sound").takes_value(true))
        .arg(Arg::new("IGNORE_DURATION").short('i').long("ignore-duration").about("Duration in milliseconds after pressing the key when another press will be ignored").takes_value(true))
        .arg(Arg::new("CANCEL_KEYS").short('c').long("cancel-keys").about("Key(s) that can cancel playing the sound").multiple_values(true).takes_value(true))
        .get_matches();

    let keys = matches.values_of("KEYS").unwrap();
    let mut keycodes: Vec<Keycode> = Vec::new();
    for key in keys {
        keycodes.push(Keycode::from_str(key).unwrap());
    }

    let sound_file_path = matches.value_of("FILE").unwrap();

    let delay = matches.value_of("DELAY").unwrap().parse().unwrap();

    let mut ignore_duration = 0;
    let ignore_duration_arg = matches.value_of("IGNORE_DURATION");
    if ignore_duration_arg.is_some() {
        ignore_duration = ignore_duration_arg.unwrap().parse().unwrap();
    }

    let cancel_keys = matches.values_of("CANCEL_KEYS").unwrap();
    let mut cancel_keycodes: Vec<Keycode> = Vec::new();
    for key in cancel_keys {
        cancel_keycodes.push(Keycode::from_str(key).unwrap());
    }

    let device_state = DeviceState::new();
    let mut previous_keys = vec![];
    let mut now = Instant::now();
    let mut timer = DISABLED_TIMER;
    loop {
        let delta_time = now.elapsed().as_millis();
        now = Instant::now();

        // println!("delta_time: {}ms, timer: {}ms", delta_time, timer);

        if timer != DISABLED_TIMER {
            timer += delta_time;
            if timer >= delay {
                play_notification_sound(sound_file_path.into());
                println!("Played {}!", sound_file_path);
                timer = DISABLED_TIMER;
            }
        }

        let keys = device_state.get_keys();
        if keys != previous_keys {
            if timer > ignore_duration && keys.iter().any(|key| keycodes.contains(key)) {
                timer = 0;
                println!("Timer (re)started, will play {} in {}ms...", sound_file_path, delay);
            }
            else if timer != DISABLED_TIMER && keys.iter().any(|key| cancel_keycodes.contains(key)) {
                timer = DISABLED_TIMER;
                println!("Timer cleared!");
            }
        }
        previous_keys = keys;

        thread::yield_now();
    }
}

fn play_notification_sound(sound_file_path: String) {
    thread::spawn(|| {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let file = File::open(sound_file_path).unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        sink.set_volume(0.5);
        sink.append(Decoder::new(BufReader::new(file)).unwrap());
        sink.sleep_until_end();
    });
}

