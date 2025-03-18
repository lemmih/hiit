use itertools::Itertools;
use leptos::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Settings {
    pub high_intensity_duration: Duration,
    pub rest_exercise_duration: Duration,
    pub rest_set_duration: Duration,
    pub sets: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            high_intensity_duration: Duration::from_secs(30),
            rest_exercise_duration: Duration::from_secs(15),
            rest_set_duration: Duration::from_secs(30),
            sets: 3,
        }
    }
}

#[derive(Clone)]
pub struct Stage {
    pub duration: Duration,
    pub is_high_intensity: bool,
    pub label: String,
}

#[derive(Clone)]
pub struct Routine {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target_area: String,
    pub intensity: String,
    pub settings: Settings,
    pub exercises: Vec<String>,
}

impl Routine {
    pub fn duration(&self) -> Duration {
        self.stages().iter().map(|stage| stage.duration).sum()
    }

    #[allow(unstable_name_collisions)]
    pub fn stages(&self) -> Vec<Stage> {
        // Prepare stage
        let prepare_stage = Stage {
            duration: Duration::from_secs(10),
            is_high_intensity: false,
            label: "Prepare".to_string(),
        };

        // Create iterator of exercise stages
        let exercise_stages = self.exercises.iter().map(|exercise| Stage {
            duration: self.settings.high_intensity_duration,
            is_high_intensity: true,
            label: exercise.clone(),
        });

        // Create rest stage
        let rest_stage = Stage {
            duration: self.settings.rest_exercise_duration,
            is_high_intensity: false,
            label: "Rest".to_string(),
        };

        // Create set break stage
        let set_break_stage = Stage {
            duration: self.settings.rest_set_duration,
            is_high_intensity: false,
            label: "Set Break".to_string(),
        };

        // Create a single set of exercise stages with rests
        let single_set: Vec<Stage> = exercise_stages.intersperse(rest_stage).collect();

        // Create iterator of sets and intersperse set breaks
        let all_stages = std::iter::repeat(single_set)
            .take(self.settings.sets as usize)
            .intersperse(vec![set_break_stage])
            .flatten();

        // Combine prepare stage with the rest of the stages
        std::iter::once(prepare_stage).chain(all_stages).collect()
    }

    pub fn stage_at_t(&self, t: f64) -> Option<(Stage, Option<Stage>, f64)> {
        let stages = self.stages();
        let mut cumulative_duration = 0.0;

        for i in 0..stages.len() {
            let current_stage = &stages[i];
            let stage_start = cumulative_duration;
            cumulative_duration += current_stage.duration.as_secs_f64();

            if t < cumulative_duration {
                let next_stage = if i + 1 < stages.len() {
                    Some(stages[i + 1].clone())
                } else {
                    None
                };
                let time_in_stage = t - stage_start;
                return Some((current_stage.clone(), next_stage, time_in_stage));
            }
        }

        None
    }
}

#[component]
pub fn RoutineCard(routine: Routine, #[prop(optional)] on_click: Option<Callback<Routine>>) -> impl IntoView {
    let routine_for_click = routine.clone();
    let handle_click = move |_| {
        if let Some(callback) = on_click {
            callback.run(routine_for_click.clone());
        }
    };

    view! {
        <div
            class="bg-white rounded-lg overflow-hidden shadow-md hover:shadow-lg transition-shadow duration-300 cursor-pointer transform hover:scale-102 transition-transform duration-300 h-full"
            on:click=handle_click
        >
            <div class="bg-gradient-to-r from-blue-500 to-indigo-600 py-3 px-4">
                <h3 class="text-xl font-bold text-white">{routine.name.clone()}</h3>
                <p class="text-blue-100 text-sm">{routine.target_area.clone()}- {routine.intensity.clone()}</p>
            </div>
            <div class="p-4">
                <p class="text-gray-700 mb-3">{routine.description.clone()}</p>
                <div class="flex items-center justify-between">
                    <span class="bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded">
                        {format!("{} seconds", routine.duration().as_secs())}
                    </span>
                    <span class="text-sm text-gray-500">Tap to start</span>
                </div>
            </div>
        </div>
    }
}
