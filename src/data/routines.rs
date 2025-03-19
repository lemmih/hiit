use crate::components::routine_card::Routine;

pub fn get_routines() -> Vec<Routine> {
    vec![
        Routine {
            id: "2".to_string(),
            name: "Upper Body".to_string(),
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
        Routine {
            id: "4".to_string(),
            name: "Lower Body".to_string(),
            description: "Build strength and endurance in your lower body.".to_string(),
            settings: Default::default(),
            exercises: vec![
                "Squats".to_string(),
                "Lunges".to_string(),
                "Glute Bridges".to_string(),
                "Calf Raises".to_string(),
            ],
        },
        Routine {
            id: "5".to_string(),
            name: "Full Body".to_string(),
            description: "Build strength and endurance in your full body.".to_string(),
            settings: Default::default(),
            exercises: vec![
                "Mountain Climbers".to_string(),
                "Superman".to_string(),
                "Inchworms".to_string(),
            ],
        },
    ]
}
