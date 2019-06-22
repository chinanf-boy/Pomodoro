
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

pub fn radio_play(){
    use std::fs::{self, File};
    use std::io::BufReader;
    use std::time::Duration;
    use std::thread::sleep;
    use std::io::{Read, Write};

    let device = rodio::default_output_device().expect("error 1");
    let shaolin = "shaolin.wav";
    let wav_cow = Asset::get(shaolin).expect("error 2");
    // let emb_buf = std::str::from_utf8(wav_cow.as_ref()).expect("error 3"); // embed struct

    let mut file = File::create(shaolin).expect("error 4");
    file.write_all(wav_cow.as_ref()).expect("error 5");

    let beep1 = rodio::play_once(&device, BufReader::new(File::open(shaolin).expect("open fail"))).expect("play fail");

    beep1.set_volume(0.2);
    // let _handle = thread::current();
    // println!("{:?}", _handle.name());
    sleep(Duration::from_millis(1000));
    fs::remove_file(shaolin).expect("fail remove");
}