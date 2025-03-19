use crate::components::routine_card::Routine;

pub fn get_routines() -> Vec<Routine> {
    vec![
        Routine {
            id: "2".to_string(),
            name: "Strength Training".to_string(),
            description: "Build muscle and increase strength with this routine.".to_string(),
            settings: Default::default(),
            exercises: vec![
                "Pushups".to_string(),
                "Overhead Triceps".to_string(),
                "Tricep Curl".to_string(),
            ],
        },
        Routine {
            id: "3".to_string(),
            name: "Core".to_string(),
            description: "Improve flexibility and reduce muscle tension.".to_string(),
            settings: Default::default(),
            exercises: vec![
                "Low Plank".to_string(),
                "Russian Twists".to_string(),
                "Leg Raises".to_string(),
            ],
        },
    ]
}
