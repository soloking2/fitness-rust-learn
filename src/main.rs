use fake::{Fake, Faker};
use fitness::{GymWorkout, CardioExercise, WeightligtingExercise};

fn main() {
    let cardio: CardioExercise = Faker.fake();
    let weightlifting: WeightligtingExercise = Faker.fake();

    println!("{:?}", GymWorkout::new(weightlifting, cardio));

}
