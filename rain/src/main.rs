extern crate term;
use rand::Rng;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let mut t = term::stdout().unwrap();

    let ground_type = vec!['_', '_', '_', '_', '_', '_', '_', '_', ',', ','];
    // ground vector will never be modified after it initialization
    let mut ground = Vec::<char>::new();
    let mut rain = Vec::<char>::new();

    let cloud: vec![];

    for i in 0..25 {
        for j in 0..50 {
            if i == 12 && j >= 23 && j <= 26 {
                match j {
                    23 => ground.push('R'),
                    24 => ground.push('A'),
                    25 => ground.push('I'),
                    26 => ground.push('N'),
                    _ => (),
                }
            } else {
                ground.push(ground_type[rand::thread_rng().gen_range(0, 10)]);
            }
            rain.push('.');
        }
        ground.push('\n');
        rain.push('\n');
    }

    let mut timer = SystemTime::now();

    loop {
        if match timer.elapsed() {
            Ok(elapsed) => {
                (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64 >= 100
            }
            Err(_) => false,
        } {
            t.fg(term::color::WHITE).unwrap();

            println!("{}[2J", 27 as char);
            let mut final_scene = Vec::<char>::new();

            for i in 0..25*50+25 {
                match rain[i] {
                    'o' => {
                        final_scene.push(rain[i]);
                        rain[i - 1] = '(';
                    },
                    '(' | ')' => {
                        final_scene.push(rain[i]);
                        rain[i] = '.';
                    },
                    '.' => {
                        final_scene.push(ground[i]);
                        if i > 0 && rain[i-1] == 'o' {
                            rain[i-1] = '.';
                            rain[i] = ')';
                        }
                    },
                    '\n' => final_scene.push(rain[i]),
                    _ => (),
                }
            }

            let s: String = final_scene.into_iter().collect();
            writeln!(t, "{}", s).unwrap();

            let mut x = Vec::<usize>::new();
            let mut y = Vec::<usize>::new();
            for _ in 0..10 {
                x.push(rand::thread_rng().gen_range(1, 49));
                y.push(rand::thread_rng().gen_range(0, 25));

                let index: usize = x.remove(0) + 51 * y.remove(0);
                if rain[index] != '('
                    && rain[index] != ')'
                    && rain[index] != 'R'
                    && rain[index] != 'A'
                    && rain[index] != 'I'
                    && rain[index] != 'N'
                    && rain[index - 1] != '('
                    && rain[index + 1] != ')'
                {
                    rain[index] = 'o';
                }
            }

            t.reset().unwrap();

            timer = SystemTime::now();
        }

        sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
