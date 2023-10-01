use std::io;
pub struct UserPreset {
    pub name: String,
    pub work_time: i32,
    pub break_time: i32,
}
pub struct TimerPreset {
    pub index: i32,
    pub name: String,
    pub work_time: i32,
    pub break_time: i32,
}

pub fn build_preset(index: i32, name: String, work_time: i32, break_time: i32) -> TimerPreset {
    TimerPreset {
        index,
        name,
        work_time,
        break_time,
    }
}
pub fn user_build_presets(_prompt: i32) -> UserPreset {
    let mut user_preset_name = String::new();
    let mut user_work_time = String::new();
    let mut user_break_time = String::new();
    println!("Input a name for your preset");
    io::stdin()
        .read_line(&mut user_preset_name)
        .expect("please input a valid name");
    println!("Input how long you would like to work");
    io::stdin()
        .read_line(&mut user_work_time)
        .expect("please input a valid number");
    println!("Input your break time");
    io::stdin()
        .read_line(&mut user_break_time)
        .expect("please input a valid number");
    let user_preset_name = user_preset_name.trim().to_string();
    let user_work_time: i32 = user_work_time
        .trim()
        .parse()
        .expect("could not read number");
    let user_break_time: i32 = user_break_time
        .trim()
        .parse()
        .expect("could not read number");
    UserPreset {
        name: user_preset_name,
        work_time: user_work_time,
        break_time: user_break_time,
    }
}
