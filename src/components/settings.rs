use chrono::{DateTime, Utc};
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WorkoutSettings {
    pub high_intensity_duration_secs: u32,
    pub rest_exercise_duration_secs: u32,
    pub rest_set_duration_secs: u32,
    pub sets: u32,
    pub routine_completions: HashMap<String, DateTime<Utc>>,
}

impl Default for WorkoutSettings {
    fn default() -> Self {
        Self {
            high_intensity_duration_secs: 30,
            rest_exercise_duration_secs: 15,
            rest_set_duration_secs: 30,
            sets: 3,
            routine_completions: HashMap::new(),
        }
    }
}

impl WorkoutSettings {
    fn save_to_storage(&self) -> bool {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(self) {
                    return storage.set_item("hiit_settings", &json).is_ok();
                }
            }
        }
        false
    }
}

// Add serde support for WorkoutSettings
impl serde::Serialize for WorkoutSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("WorkoutSettings", 5)?;
        state.serialize_field("high_intensity_duration_secs", &self.high_intensity_duration_secs)?;
        state.serialize_field("rest_exercise_duration_secs", &self.rest_exercise_duration_secs)?;
        state.serialize_field("rest_set_duration_secs", &self.rest_set_duration_secs)?;
        state.serialize_field("sets", &self.sets)?;
        state.serialize_field("routine_completions", &self.routine_completions)?;
        state.end()
    }
}

// Add deserialization support for WorkoutSettings
impl<'de> serde::Deserialize<'de> for WorkoutSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct SettingsHelper {
            high_intensity_duration_secs: u32,
            rest_exercise_duration_secs: u32,
            rest_set_duration_secs: u32,
            sets: u32,
            routine_completions: HashMap<String, DateTime<Utc>>,
        }

        let helper = SettingsHelper::deserialize(deserializer)?;

        Ok(WorkoutSettings {
            high_intensity_duration_secs: helper.high_intensity_duration_secs,
            rest_exercise_duration_secs: helper.rest_exercise_duration_secs,
            rest_set_duration_secs: helper.rest_set_duration_secs,
            sets: helper.sets,
            routine_completions: helper.routine_completions,
        })
    }
}

// Range slider component for reusability
#[component]
fn RangeSlider(
    label: String,
    value: Signal<u32>,
    on_change: Callback<u32>,
    min: u32,
    max: u32,
    step: u32,
    unit: String,
) -> impl IntoView {
    let id = format!("slider-{}", label.to_lowercase().replace(" ", "-"));

    let on_input = move |ev| {
        let new_value = event_target_value(&ev).parse::<u32>().unwrap_or_default();
        on_change.run(new_value);
    };

    view! {
      <div class="mb-6">
        <label for=id.clone() class="block mb-2 text-sm font-medium text-gray-700">
          {label}
          {
            let unit = unit.clone();
            move || format!(" ({}{})", value.get(), unit)
          }
        </label>
        <input
          type="range"
          id=id
          min=min.to_string()
          max=max.to_string()
          step=step.to_string()
          value=move || value.get().to_string()
          on:input=on_input
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
        />
        <div class="flex justify-between mt-1 text-xs text-gray-500">
          <span>{min} {unit.clone()}</span>
          <span>{max} {unit.clone()}</span>
        </div>
      </div>
    }
}

// Create a context for the settings
#[derive(Clone)]
pub struct SettingsContext {
    pub settings: Signal<WorkoutSettings>,
    pub update_settings: Callback<WorkoutSettings>,
}

// Create a provider component for the settings context
#[component]
pub fn SettingsProvider(children: Children) -> impl IntoView {
    let settings = RwSignal::new(WorkoutSettings::default());
    let update_settings = Callback::new(move |new_settings: WorkoutSettings| {
        settings.set(new_settings.clone());
        new_settings.save_to_storage();
    });

    // Load settings from storage on the client side
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("hiit_settings") {
                    if let Ok(loaded_settings) = serde_json::from_str::<WorkoutSettings>(&json) {
                        settings.set(loaded_settings);
                    }
                }
            }
        }
    });

    provide_context(SettingsContext {
        settings: settings.into(),
        update_settings,
    });

    children()
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let SettingsContext {
        settings,
        update_settings,
    } = expect_context::<SettingsContext>();

    // Create preset settings functions
    let apply_preset = move |high: u32, rest: u32, set_rest: u32, sets_count: u32| {
        update_settings.run(WorkoutSettings {
            high_intensity_duration_secs: high,
            rest_exercise_duration_secs: rest,
            rest_set_duration_secs: set_rest,
            sets: sets_count,
            ..settings.get()
        });
    };

    // Functions to check if the current settings match a preset
    let is_low_preset = move || {
        let s = settings.get();
        s.high_intensity_duration_secs == 30
            && s.rest_exercise_duration_secs == 15
            && s.rest_set_duration_secs == 30
            && s.sets == 3
    };

    let is_mid_preset = move || {
        let s = settings.get();
        s.high_intensity_duration_secs == 45
            && s.rest_exercise_duration_secs == 10
            && s.rest_set_duration_secs == 15
            && s.sets == 4
    };

    let is_high_preset = move || {
        let s = settings.get();
        s.high_intensity_duration_secs == 60
            && s.rest_exercise_duration_secs == 0
            && s.rest_set_duration_secs == 15
            && s.sets == 6
    };

    // Set up handlers for the preset buttons
    let set_low_preset = move |_| apply_preset(30, 15, 30, 3);
    let set_mid_preset = move |_| apply_preset(45, 10, 15, 4);
    let set_high_preset = move |_| apply_preset(60, 0, 15, 6);

    view! {
      <div class="container py-8 px-4 mx-auto max-w-2xl">
        <h1 class="mb-6 text-2xl font-bold text-center text-gray-800 md:text-3xl">Settings</h1>

        <div class="p-6 mb-6 bg-white rounded-lg shadow-md">
          <p class="mb-4 text-sm text-center text-gray-600">
            Settings are automatically saved as you adjust them.
          </p>

          <div class="flex gap-4 justify-center mb-6">
            <button
              class=move || {
                if is_low_preset() {
                  "px-4 py-2 font-medium rounded-md bg-blue-600 text-white"
                } else {
                  "px-4 py-2 font-medium rounded-md bg-white border border-gray-300 text-gray-700 hover:bg-gray-50"
                }
              }
              on:click=set_low_preset
            >
              Low
            </button>
            <button
              class=move || {
                if is_mid_preset() {
                  "px-4 py-2 font-medium rounded-md bg-blue-600 text-white"
                } else {
                  "px-4 py-2 font-medium rounded-md bg-white border border-gray-300 text-gray-700 hover:bg-gray-50"
                }
              }
              on:click=set_mid_preset
            >
              Mid
            </button>
            <button
              class=move || {
                if is_high_preset() {
                  "px-4 py-2 font-medium rounded-md bg-blue-600 text-white"
                } else {
                  "px-4 py-2 font-medium rounded-md bg-white border border-gray-300 text-gray-700 hover:bg-gray-50"
                }
              }
              on:click=set_high_preset
            >
              High
            </button>
          </div>

          <RangeSlider
            label="High Intensity Duration".to_string()
            value=Signal::derive(move || settings.get().high_intensity_duration_secs)
            on_change=Callback::new(move |new_value| {
              update_settings
                .run(WorkoutSettings {
                  high_intensity_duration_secs: new_value,
                  ..settings.get()
                });
            })
            min=5
            max=300
            step=5
            unit="s".to_string()
          />

          <RangeSlider
            label="Exercise Rest Duration".to_string()
            value=Signal::derive(move || settings.get().rest_exercise_duration_secs)
            on_change=Callback::new(move |new_value| {
              update_settings
                .run(WorkoutSettings {
                  rest_exercise_duration_secs: new_value,
                  ..settings.get()
                });
            })
            min=0
            max=120
            step=5
            unit="s".to_string()
          />

          <RangeSlider
            label="Set Rest Duration".to_string()
            value=Signal::derive(move || settings.get().rest_set_duration_secs)
            on_change=Callback::new(move |new_value| {
              update_settings
                .run(WorkoutSettings {
                  rest_set_duration_secs: new_value,
                  ..settings.get()
                });
            })
            min=0
            max=120
            step=5
            unit="s".to_string()
          />

          <RangeSlider
            label="Number of Sets".to_string()
            value=Signal::derive(move || settings.get().sets)
            on_change=Callback::new(move |new_value| {
              update_settings
                .run(WorkoutSettings {
                  sets: new_value,
                  ..settings.get()
                });
            })
            min=1
            max=30
            step=1
            unit="".to_string()
          />
        </div>

        <div class="text-center">
          <a
            href="/"
            class="inline-block py-2 px-4 font-semibold text-white bg-blue-600 rounded-lg transition-colors hover:bg-blue-700"
          >
            Back to Workouts
          </a>
        </div>
      </div>
    }
}
