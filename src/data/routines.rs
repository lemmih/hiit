use crate::components::routine_card::Routine;

pub fn get_routines() -> Vec<Routine> {
    vec![
        Routine {
            id: "2".to_string(),
            name: "Upper Body".to_string(),
            exercises: vec![
                "Push ups".to_string(),
                "Overhead triceps".to_string(),
                "Hammer curls".to_string(),
                "Skull crushers".to_string(),
            ],
        },
        Routine {
            id: "3".to_string(),
            name: "Core".to_string(),
            exercises: vec![
                "Low plank".to_string(),
                "Crunches".to_string(),
                "Russian twists".to_string(),
                "Bicycle crunches".to_string(),
            ],
        },
        Routine {
            id: "4".to_string(),
            name: "Lower Body".to_string(),
            exercises: vec![
                "Squats".to_string(),
                "Lunges".to_string(),
                "Calf raises".to_string(),
                "Froggy glute lifts".to_string(),
            ],
        },
        Routine {
            id: "5".to_string(),
            name: "Full Body".to_string(),
            exercises: vec![
                "Mountain climber".to_string(),
                "Superman".to_string(),
                "Inchworm".to_string(),
                "Burpees".to_string(),
            ],
        },
        Routine {
            id: "6".to_string(),
            name: "Mobility".to_string(),
            exercises: vec![
                "Squat to Toe Touch".to_string(),
                "Downward to Upward Dog".to_string(),
                "Lunge and Reach".to_string(),
                "Modified Butterfly Sit".to_string(),
            ],
        },
    ]
}
