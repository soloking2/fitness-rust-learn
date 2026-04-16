mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

pub fn ask_about_program() {
    println!("The nutritionist is {NUTRITIONIST}");
}
}
pub mod weightlifting;
pub mod cardio;

pub use cardio::{CardioTool, Exercise as CardioExercise};
pub use weightlifting::{Exercise as WeightligtingExercise};

#[derive(Debug)]
pub struct GymWorkout {
    pub weightlifting: WeightligtingExercise,
    pub cardio: CardioExercise,
}

impl GymWorkout {
    pub fn new(weightlifting: WeightligtingExercise, cardio: CardioExercise) -> Self {
        diet::ask_about_program();
        weightlifting::ask_about_program();
        cardio::ask_about_program();
        Self {weightlifting, cardio  }
    }
}