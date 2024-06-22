fn main() {
    println!("|==================================|");
    println!("| Welcome to Data Structure in Rust! |");

    let mut person = Person::new("Moaz", 100.0);

    person.status();

    // person.walk();
    person.perform_activity(Activity::Walk);
    person.status();

    // person.run();
    person.perform_activity(Activity::Run);

    println!("\n|> At end of the day:");
    person.status();
}

struct Person {
    name: String,
    health: f32,
    steps: i32,
}

impl Person {
    // Associated function
    fn new(name: &str, health: f32) -> Self {
        Person {
            name: name.to_string(),
            health,
            steps: 0,
        }
    }

    // Methods

    fn status(&self) {
        println!(
            "╭──────────────────────────────\n\
            │  🙋 Name: {}          \n\
            │  🚶 Steps: {}                 \n\
            │  ❤️  Health: {}                \n\
            ╰──────────────────────────────",
            self.name, self.steps, self.health
        );
    }

    fn refresh_health(&mut self) {
        self.health -= (self.steps / 10) as f32;
    }

    // fn walk(&mut self) {
    //     let walk_step = 10;
    //     println!("\n🚶=> Walking {} steps", walk_step);
    //     self.steps += walk_step;
    //     self.refresh_health();
    // }

    // fn run(&mut self) {
    //     let run_step = 20;
    //     println!("\n🏃=> Running {} steps", run_step);
    //     self.steps += run_step;
    //     self.refresh_health();
    // }

    // =====
    fn perform_activity(&mut self, activity: Activity) {
        let steps = activity.activity_steps();
        match activity {
            Activity::Walk => println!("\n🚶=> Walking {} steps", steps),
            Activity::Run => println!("\n🏃=> Running {} steps", steps),
        }
        self.steps += steps;
        self.refresh_health();
    }
}

// Enum
enum Activity {
    Walk,
    Run,
}

impl Activity {
    fn activity_steps(&self) -> i32 {
        match self {
            Activity::Walk => 10,
            Activity::Run => 50,
        }
    }
}
