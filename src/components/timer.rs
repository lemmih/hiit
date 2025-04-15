use crate::components::screen_wake_lock::ScreenWakeLock;
use crate::data::routines::get_routines;
use chrono::Utc;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_use::{use_interval_with_options, UseIntervalOptions, UseIntervalReturn};
use std::collections::HashSet;
use std::time::Duration;
use web_sys::SpeechSynthesisUtterance;

use super::settings::SettingsContext;

#[component]
pub fn TimerPage() -> impl IntoView {
    // Get exercise ID from the URL
    let params = use_params_map();
    let exercise_id = params.with(|p| p.get("id").unwrap_or_default());

    // Routine data from our data module
    let routines = get_routines();

    // Find the selected routine using StoredValue
    let Some(r) = routines.iter().find(|r| r.id == exercise_id).cloned() else {
        return view! {
          <div class="p-4 mx-auto max-w-lg">
            <div class="p-6 bg-white rounded-lg shadow-md">
              <h2 class="mb-4 text-xl font-bold text-center">Exercise Not Found</h2>
              <div class="text-center">
                <a href="/" class="text-blue-600 hover:text-blue-800">
                  Return to Home
                </a>
              </div>
            </div>
          </div>
        }
        .into_any();
    };
    let routine = StoredValue::new(r);

    // Get settings context
    let SettingsContext {
        settings,
        update_settings,
    } = expect_context::<SettingsContext>();

    // Initialize interval (1 second = 1000ms)
    let interval = 25;
    let UseIntervalReturn {
        counter,
        pause,
        resume,
        is_active,
        reset,
        ..
    } = use_interval_with_options(interval, UseIntervalOptions::default().immediate(false));

    // Store already spoken announcements to avoid duplicates
    let spoken_announcements = StoredValue::new(HashSet::<(usize, String)>::new());

    let time_left = move || {
        routine.read_value().duration() - Duration::from_secs_f64(counter.get() as f64 * interval as f64 / 1000.0)
    };

    // Format time as MM:SS
    let format_time = move |seconds: u32| {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        format!("{:02}:{:02}", minutes, remaining_seconds)
    };

    // Helper function to speak text using speech synthesis
    let speak = move |index: usize, text: &str| {
        // Create a key from index and text
        let announcement_key = (index, text.to_string());

        // Check if this announcement has already been spoken
        let mut spoken = spoken_announcements.get_value();
        if !spoken.contains(&announcement_key) {
            // Add it to the set of spoken announcements
            spoken.insert(announcement_key);
            spoken_announcements.set_value(spoken);

            // Speak the announcement
            if let Some(window) = web_sys::window() {
                if let Ok(speech) = window.speech_synthesis() {
                    // Cancel any ongoing speech
                    speech.cancel();

                    // Create and speak the new utterance
                    if let Ok(utterance) = SpeechSynthesisUtterance::new_with_text(text) {
                        speech.speak(&utterance);
                    }
                }
            }
        }
    };

    // Effect to handle stage announcements and countdowns
    Effect::new(move |_| {
        // Only process when the timer is active
        if !is_active.get() {
            return;
        }

        let routine_val = routine.get_value();
        let elapsed = routine_val.duration().as_secs_f64() - time_left().as_secs_f64();

        if let Some((stage_index, current, _next, time_in_stage)) = routine_val.stage_at_t(elapsed) {
            // Calculate remaining time in this stage
            let remaining = current.duration.as_secs_f64() - time_in_stage;

            // Announce new stage (only once when it changes)
            if time_in_stage < 1_f64 {
                // Announce the new stage name
                let announcement = current.label.clone();
                speak(stage_index, &announcement);
            }
            // Handle countdown when approaching the end of a stage
            if remaining <= 3.0 {
                speak(stage_index, "3");
            }
            if remaining < 2.0 {
                speak(stage_index, "2");
            }
            if remaining <= 1.0 {
                speak(stage_index, "1");
            }
        }
    });

    // Reset spoken announcements when timer is reset
    Effect::new(move |_| {
        if counter.get() == 0 {
            spoken_announcements.set_value(HashSet::new());
        }
    });

    // Effect to handle routine completion
    Effect::new(move |_| {
        if time_left().as_secs() == 0 && is_active.get() {
            // Record completion
            let mut new_settings = settings.get();
            let routine_name = routine.get_value().name.clone();
            let now = Utc::now();

            // Only update if there's no previous completion or it was more than 10 seconds ago
            let should_update = match new_settings.routine_completions.get(&routine_name) {
                Some(last_completion) => (now - *last_completion).num_seconds() > 10,
                None => true,
            };

            if should_update {
                new_settings.routine_completions.insert(routine_name, now);
                update_settings.run(new_settings);
            }
        }
    });

    // View
    view! {
      <div class="p-4 mx-auto max-w-lg">
        {move || {
          let r = routine.get_value();
          view! {
            <div class="p-6 bg-white rounded-lg shadow-md">
              // Only render the ScreenWakeLock component when the timer is active
              {move || {
                if is_active.get() {
                  view! { <ScreenWakeLock /> }.into_any()
                } else {
                  view! { <div></div> }.into_any()
                }
              }} <div class="flex mb-6">
                <a href="/" class="text-blue-600 hover:text-blue-800">
                  Back
                </a>
                <h2 class="flex-1 text-xl font-bold text-center">{r.name.clone()}</h2>
                <div class="invisible">Back</div>
              </div><div class="mb-6">
                <div class="mb-2 text-gray-700">{r.description()}</div>
              </div><div class="mb-8">
                <div class="mb-2 text-lg font-bold text-center">
                  {move || format_time(time_left().as_secs() as u32)}
                </div>
                <div class="overflow-hidden mb-4 h-2 bg-gray-200 rounded-full">
                  <div
                    class="h-full bg-gradient-to-r from-blue-500 to-indigo-600"
                    style:width=move || {
                      let duration = routine.get_value().duration().as_secs();
                      format!("{}%", (time_left().as_secs() as f32 / duration as f32) * 100.0)
                    }
                  ></div>
                </div>
                {move || {
                  let routine = routine.get_value();
                  let elapsed = routine.duration().as_secs_f64() - time_left().as_secs_f64();
                  if let Some((_stage_index, current, next, time_in_stage)) = routine
                    .stage_at_t(elapsed)
                  {
                    let stage_progress_pct = (time_in_stage / (current.duration.as_secs_f64()))
                      * 100.0;
                    view! {
                      <div class="mt-4 text-center">
                        <div class="text-5xl font-semibold">{current.label.clone()}</div>
                        <div class="overflow-hidden mb-2 h-2 bg-gray-200 rounded-full">
                          <div
                            class=move || {
                              if current.is_high_intensity {
                                "h-full bg-gradient-to-r from-red-500 to-orange-400"
                              } else {
                                "h-full bg-gradient-to-r from-green-400 to-teal-500"
                              }
                            }
                            style:width=move || {
                              if current.is_high_intensity {
                                format!("{}%", stage_progress_pct)
                              } else {
                                format!("{}%", 100.0 - stage_progress_pct)
                              }
                            }
                          ></div>
                        </div>

                        {if let Some(next_stage) = next {
                          view! {
                            <div class="mt-1 text-sm text-gray-500">
                              "Next: " {next_stage.label.clone()}
                            </div>
                          }
                            .into_any()
                        } else {
                          view! { <div class="mt-1 text-sm text-gray-500">"Final Stage"</div> }
                            .into_any()
                        }}
                      </div>
                    }
                      .into_any()
                  } else {
                    view! { <div class="mt-4 text-center text-gray-500">"Workout Complete"</div> }
                      .into_any()
                  }
                }}
              </div> <div class="text-center">
                <div class="flex justify-center space-x-4">
                  {if is_active.get() {
                    view! {
                      <button
                        class="py-2 px-4 text-white bg-red-500 rounded transition-colors hover:bg-red-600"
                        on:click={
                          let pause = pause.clone();
                          move |_| pause()
                        }
                      >
                        "Pause"
                      </button>
                    }
                      .into_any()
                  } else {
                    view! {
                      <button
                        class="py-2 px-4 text-white bg-green-500 rounded transition-colors hover:bg-green-600"
                        on:click={
                          let resume = resume.clone();
                          move |_| resume()
                        }
                      >
                        "Start Routine"
                      </button>
                    }
                      .into_any()
                  }}
                  <button
                    class="py-2 px-4 text-white bg-gray-500 rounded transition-colors hover:bg-gray-600"
                    on:click={
                      let reset = reset.clone();
                      move |_| reset()
                    }
                  >
                    "Reset"
                  </button>
                  <button
                    class="hidden py-2 px-4 text-white bg-purple-500 rounded transition-colors hover:bg-purple-600"
                    on:click={
                      move |_| {
                        let mut new_settings = settings.get();
                        new_settings
                          .routine_completions
                          .insert(routine.get_value().name.clone(), Utc::now());
                        update_settings.run(new_settings);
                      }
                    }
                  >
                    "Debug: Mark Complete"
                  </button>
                </div>
              </div>
            </div>
          }
            .into_any()
        }}
      </div>
    }
    .into_any()
}
