use chrono::{DateTime, Duration as ChronoDuration, Utc};
use itertools::Itertools;
use leptos::prelude::*;
use std::time::Duration;

use super::settings::SettingsContext;

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
    pub exercises: Vec<String>,
}

impl Routine {
    pub fn description(&self) -> String {
        self.exercises.join(", ")
    }

    pub fn duration(&self) -> Duration {
        self.stages().iter().map(|stage| stage.duration).sum()
    }

    #[allow(unstable_name_collisions)]
    pub fn stages(&self) -> Vec<Stage> {
        // Get global settings
        let SettingsContext { settings, .. } = expect_context::<SettingsContext>();

        // Prepare stage
        let prepare_stage = Stage {
            duration: Duration::from_secs(10),
            is_high_intensity: false,
            label: "Prepare".to_string(),
        };

        // Create iterator of exercise stages
        let exercise_stages = self.exercises.iter().map(|exercise| Stage {
            duration: Duration::from_secs(settings.get().high_intensity_duration_secs as u64),
            is_high_intensity: true,
            label: exercise.clone(),
        });

        // Create rest stage
        let rest_stage = Stage {
            duration: Duration::from_secs(settings.get().rest_exercise_duration_secs as u64),
            is_high_intensity: false,
            label: "Rest".to_string(),
        };

        // Create set break stage
        let set_break_stage = Stage {
            duration: Duration::from_secs(settings.get().rest_set_duration_secs as u64),
            is_high_intensity: false,
            label: "Set Break".to_string(),
        };

        // Create a single set of exercise stages with rests
        let single_set: Vec<Stage> = exercise_stages.intersperse(rest_stage).collect();

        // Create iterator of sets and intersperse set breaks
        let all_stages = std::iter::repeat(single_set)
            .take(settings.get().sets as usize)
            .intersperse(vec![set_break_stage])
            .flatten();

        // Combine prepare stage with the rest of the stages
        std::iter::once(prepare_stage).chain(all_stages).collect()
    }

    pub fn stage_at_t(&self, t: f64) -> Option<(usize, Stage, Option<Stage>, f64)> {
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
                return Some((i, current_stage.clone(), next_stage, time_in_stage));
            }
        }

        None
    }
}

fn format_last_completion(last_completion: Option<DateTime<Utc>>) -> String {
    if let Some(completion_time) = last_completion {
        let now = Utc::now();
        let duration = now - completion_time;

        if duration < ChronoDuration::hours(24) {
            "done today".to_string()
        } else if duration < ChronoDuration::hours(48) {
            "done yesterday".to_string()
        } else {
            let days = duration.num_days();
            format!("done {} days ago", days)
        }
    } else {
        "".to_string()
    }
}

#[component]
pub fn RoutineCard(
    #[prop(into)] routine: StoredValue<Routine>,
    #[prop(optional)] on_click: Option<Callback<Routine>>,
) -> impl IntoView {
    let routine_for_click = routine.get_value();
    let handle_click = move |_| {
        if let Some(callback) = on_click {
            callback.run(routine_for_click.clone());
        }
    };

    let SettingsContext { settings, .. } = expect_context::<SettingsContext>();
    let last_completion = move || {
        settings
            .get()
            .routine_completions
            .get(&routine.get_value().name)
            .cloned()
    };

    view! {
      <div
        class="overflow-hidden h-full bg-white rounded-lg shadow-md transition-shadow transition-transform duration-300 duration-300 transform cursor-pointer hover:shadow-lg hover:scale-102"
        on:click=handle_click
      >
        <div class="py-3 px-4 bg-gradient-to-r from-blue-500 to-indigo-600">
          <h3 class="text-xl font-bold text-white">{routine.get_value().name.clone()}</h3>
        </div>
        <div class="p-4">
          <p class="mb-3 text-gray-700">{routine.get_value().description()}</p>
          <div class="flex justify-between items-center">
            <span class="py-0.5 px-2.5 text-xs font-semibold text-blue-800 bg-blue-100 rounded">
              {move || format!("{} seconds", routine.get_value().duration().as_secs())}
            </span>
            {move || {
              if let Some(completion_time) = last_completion() {
                view! {
                  <span class="text-sm text-gray-500">
                    {format_last_completion(Some(completion_time))}
                  </span>
                }
                  .into_any()
              } else {
                view! { <div></div> }.into_any()
              }
            }}
          </div>
        </div>
      </div>
    }
}
