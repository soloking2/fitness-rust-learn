const PERSONAL_TRAINER: &str = "Carl Cardio";
use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum CardioTool {
    Treadmill,
    Bike
}
#[derive(Debug, Dummy)]
pub struct Exercise {
    pub day: String,
    pub tool: CardioTool,
    pub minutes: u32,
}

impl Exercise {
   pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        Self { day, tool, minutes }
    }
}

pub fn ask_about_program() {
    println!("The cardio trainer is {}", PERSONAL_TRAINER);
}