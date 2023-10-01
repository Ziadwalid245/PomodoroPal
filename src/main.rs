use std::{
    io::{self, Write},
    thread,
    time::Duration,
    vec,
};

use create_presets::{TimerPreset, UserPreset};
mod create_presets;
mod playsound;
fn main() {
    let preset_1 = create_presets::build_preset(
        1,
        String::from("25min preset \n work time : 25mins ,  break time : 5mins"),
        1500,
        300,
    );
    let preset_2 = create_presets::build_preset(
        2,
        String::from("30min preset \n work time : 30mins ,  break time : 10mins"),
        1800,
        600,
    );
    let preset_3 = create_presets::build_preset(
        3,
        String::from("50min preset \n work time : 50mins ,  break time : 20mins"),
        3000,
        1200,
    );
    println!(" 0 -> Show available presets");
    println!(
        " 1 -> Work for {} minutes and have {} minute break",
        preset_1.work_time / 60,
        preset_1.break_time / 60
    );
    println!(
        " 2 -> Work for {} minutes and have {} minute break",
        preset_2.work_time / 60,
        preset_2.break_time / 60
    );
    println!(
        " 3 -> Work for {} minutes and have {} minute break",
        preset_3.work_time / 60,
        preset_3.break_time / 60
    );
    println!(" 4 -> Make your own preset");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    let user_choice: i32 = input
        .trim()
        .parse()
        .expect("you typed {}.your input must be a number like: 1 , 2 ,3.");
    let timer_presets = vec![preset_1, preset_2, preset_3];
    for i in &timer_presets {
        if user_choice == i.index {
            run_timer(i)
        } else if user_choice == 0 {
            println!("{}", i.name);
        } else if user_choice == 4 {
            let user_preset = create_presets::user_build_presets(4);
            run_user_timer(&user_preset);
        }
    }
}
fn countdown_timer(duration: i32, sound_file: &str) {
    for i in (0..duration).rev() {
        thread::sleep(Duration::from_secs(1));
        let minutes = i / 60;
        let seconds = i % 60;
        print!("\r00:{}:{}", minutes, seconds);
        io::stdout()
            .flush()
            .expect("could not render the time correctly");
    }
    playsound::play_sound(sound_file);
    println!("Time is up!");
}

fn run_timer(preset: &TimerPreset) {
    countdown_timer(preset.work_time, "./mixkit-bell-notification-933.wav");
    thread::sleep(Duration::from_secs(3));
    println!("Starting your break");
    countdown_timer(preset.break_time, "./mixkit-bell-notification-933.wav");
    println!("break time over!");
}
fn run_user_timer(preset: &UserPreset) {
    countdown_timer(preset.work_time, "./mixkit-bell-notification-933.wav");
    thread::sleep(Duration::from_secs(3));
    println!("Starting your break");
    countdown_timer(preset.break_time, "./mixkit-bell-notification-933.wav");
    println!("break time over!");
}
