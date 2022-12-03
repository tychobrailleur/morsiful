use std::collections::HashMap;
use std::time::Duration;

use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

const FREQ_SOUND:f32 = 1_000.0;
const DAT_TIME:f32 = 370.0;
const DIT_TIME:f32 = 140.0;

const BETWEEN_LETTERS:f32 = 1_200.0;
const BETWEEN_WORDS:f32 = 3_300.0-BETWEEN_LETTERS;

fn play_word(word:&str, sink:&Sink) {
    let alphabet:HashMap<char,&str> = HashMap::from([
        ('A', ".-"),
        ('B', "-..."),
        ('C', "-.-."),
        ('D', "-.."),
        ('E', "."),
        ('F', "..-."),
        ('G', "--."),
        ('H', "...."),
        ('I', ".."),
        ('J', ".---"),
        ('K', "-.-"),
        ('L', ".-.."),
        ('M', "--"),
        ('N', "-."),
        ('O', "---"),
        ('P', ".--."),
        ('Q', "--.-"),
        ('R', ".-."),
        ('S', "..."),
        ('T', "-"),
        ('U', "..-"),
        ('V', "...-"),
        ('W', ".--"),
        ('X', "-..-"),
        ('Y', "-.--"),
        ('Z', "--.."),
        ('.', ".-.-.-"),
        (',', "--..--"),
        ('?', "..--.."),
        ('/', "-..-."),
        ('=', "-...-"),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----")
    ]);

    for c in word.to_uppercase().chars() {
        let code = *alphabet.get(&c).unwrap();
        for i in code.chars() {
            match i {
                '.' => { play_dit(sink); }
                '-' => { play_dat(sink); }
                _ => { panic!("Unknown char!"); }
            }
        }
        sink.append(SineWave::new(0.0).take_duration(Duration::from_secs_f32(BETWEEN_LETTERS/1_000.0))
            .fade_in(Duration::from_secs_f32(0.2)));
    }

    sink.append(SineWave::new(0.0).take_duration(Duration::from_secs_f32(BETWEEN_WORDS/1_000.0))
        .fade_in(Duration::from_secs_f32(0.2)));
}

fn play_dat(sink:&Sink) {
    let a_source = SineWave::new(FREQ_SOUND)
        .take_duration(Duration::from_secs_f32(DAT_TIME/1_000.0))
        .fade_in(Duration::from_millis(9))
        .amplify(0.6);
    sink.append(a_source);

    // Append silence
    sink.append(SineWave::new(0.0).take_duration(Duration::from_secs_f32(100.0/1_000.0))
        .fade_in(Duration::from_secs_f32(0.2)));
}

fn play_dit(sink:&Sink) {
    let a_source = SineWave::new(FREQ_SOUND)
        .take_duration(Duration::from_secs_f32(DIT_TIME/1_000.0))
        .fade_in(Duration::from_millis(9))
        .amplify(0.6);
    sink.append(a_source);

    // Append silence
    sink.append(SineWave::new(0.0).take_duration(Duration::from_secs_f32(100.0/1_000.0))
        .fade_in(Duration::from_secs_f32(0.2)));
}


fn main() {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // IRTS First Morse code test sample, https://www.irts.ie/cgi/st.cgi?morse_test
    let sentence = "The halfwave dipole, a resonant aerial, does not need an aerial tuning unit if it is cut. ? /";
    let words = sentence.split(" ");
    for word in words {
        play_word(word, &sink);
    }

    sink.sleep_until_end();
}
