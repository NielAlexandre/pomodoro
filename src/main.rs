
use std::env;
use std::time::Duration;
use std::thread::sleep;
use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;
use chrono::Timelike;
use chrono::offset::Local;

fn log(mess: &str) {
    println!("{} - {}", now(), mess);
}

fn now() -> String {
    let now = Local::now();
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}


fn play_melody(sink: &Sink, melody: &Vec<(f32, f32)>) {
    for (note, dur) in melody {
        let source = SineWave::new(*note)
            .take_duration(Duration::from_secs_f32(*dur))
            .amplify(1.2);
        sink.append(source);
    }
    sink.sleep_until_end();
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let args: Vec<String> = env::args().collect();
    let work_duration: u64 = args[1].trim().parse().expect("Argument 1 was supposed to be an integer");
    let pause_duration: u64 = args[2].trim().parse().expect("Argument 2 was supposed to be an integer");
    let work_duration = Duration::from_secs(work_duration);
    let pause_duration = Duration::from_secs(pause_duration);

    let melody2 = vec![
        //(note, duration)
        (540.0, 0.2),
        (600.0, 0.1),
        (560.0, 0.1),
        (620.0, 0.2),
        (560.0, 0.1),
        (600.0, 0.1),
        (540.0, 0.2),
    ];

    let melody = vec![
        //(note, duration)
        (440.0, 0.1),
        (500.0, 0.2),
        (450.0, 0.2),
        (510.0, 0.1),
        (450.0, 0.2),
        (500.0, 0.2),
        (440.0, 0.1),
    ];

    let mut i = 0;
    log(&format!("[{:>3}] - Building Steam", i));
    play_melody(&sink, &vec![(700.0, 0.75)]);
    loop {
        i += 1;
        log(&format!("[{:>3}] - Work", i));
        play_melody(&sink, &melody);
        sleep(work_duration);
        log(&format!("[{:>3}] - Pause", i));
        play_melody(&sink, &melody2);
        sleep(pause_duration);
    }

}
