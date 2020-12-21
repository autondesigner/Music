use hound;
use std::collections::LinkedList;

enum WaveFunction {
    Square,
    Sawtooth,
    Triangle,
}

struct MusicElement {
    function: WaveFunction,
    time: f32,
    duration: f32,
    note: f32,
}

impl MusicElement {
    fn new(function: WaveFunction, time: f32, duration: f32, note: f32) -> MusicElement {
        MusicElement {
            function,
            time,
            duration,
            note,
        }
    }
}

struct Generator {
    elements: LinkedList<MusicElement>,
}

fn apply_wave_function(function: &WaveFunction, t: f32, f: f32) -> f32 {
    match function {
        WaveFunction::Square => square(t, f),
        WaveFunction::Sawtooth => sawtooth(t, f),
        WaveFunction::Triangle => triangle(t, f),
    }
}

impl Generator {
    fn new() -> Generator {
        let elements = LinkedList::new();
        Generator { elements }
    }
    fn add_music_element(&mut self, function: WaveFunction, time: f32, duration: f32, note: f32) {
        let element = MusicElement::new(function, time, duration, note);
        &self.elements.push_back(element);
    }
    fn render_elements(&self, sample_rate: f32, wave: &mut Vec<f32>) {
        for element in self.elements.iter() {
            let first_sample = (sample_rate * element.time) as usize;
            let last_sample = first_sample + (sample_rate * element.duration) as usize;
            let frequency = get_frequency_from_note(element.note);
            let mut sample_time = 0f32;
            for sample in first_sample..last_sample {
                let t = sample_time / sample_rate;
                let level = apply_wave_function(&element.function, t, frequency);
                wave[sample] += level;
                sample_time += 1.0;
            }
        }
    }
    fn create_wave(&self, sample_rate: u32, silence_time: f32) -> Vec<f32> {
        let mut global_time = 0f32;
        for element in self.elements.iter() {
            let end_time = element.time + element.duration;
            if end_time > global_time {
                global_time = end_time;
            }
        }
        global_time += silence_time;
        let sample_rate_f32 = sample_rate as f32;
        let samples_count_f32 = global_time * sample_rate_f32;
        let samples_count = samples_count_f32 as usize;
        let mut wave: Vec<f32> = Vec::with_capacity(samples_count);
        for _i in 0..samples_count {
            wave.push(0.0);
        }
        wave
    }
    fn render(&self, sample_rate: u32, silence_time: f32, file_name: &str) {
        let mut wave = self.create_wave(sample_rate, silence_time);
        self.render_elements(sample_rate as f32, &mut wave);
        let wav_spec = hound::WavSpec {
            channels: 1,
            sample_rate: sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(file_name, wav_spec).unwrap();
        for sample in wave {
            writer.write_sample(sample).unwrap();
        }
    }
}

fn square(t: f32, f: f32) -> f32 {
    return 2.0 * (2.0 * (t * f).floor() - (2.0 * t * f).floor()) + 1.0;
}

fn sawtooth(t: f32, f: f32) -> f32 {
    return 2.0 * (t * f - (1.0 / 2.0 + t * f).floor());
}

fn triangle(t: f32, f: f32) -> f32 {
    return 2.0 * (2.0 * (t * f - (1.0 / 2.0 + t * f).floor())).abs() - 1.0;
}

fn get_frequency_from_note(note: f32) -> f32 {
    return 440.0 * 2.0f32.powf(note / 12.0);
}

fn main() {
    let mut generator = Generator::new();
    generator.add_music_element(WaveFunction::Square, 0.0, 1.0, 0.0);
    generator.add_music_element(WaveFunction::Square, 1.0, 1.0, -6.0);
    generator.add_music_element(WaveFunction::Square, 3.0, 1.0, 0.0);
    generator.add_music_element(WaveFunction::Square, 5.0, 1.0, -6.0);

    generator.render(48000, 0.0, "music.wav");
}
