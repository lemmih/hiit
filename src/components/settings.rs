use leptos::prelude::*;

#[derive(Clone)]
pub struct WorkoutSettings {
    pub high_intensity_duration_secs: u32,
    pub rest_exercise_duration_secs: u32,
    pub rest_set_duration_secs: u32,
    pub sets: u32,
}

impl Default for WorkoutSettings {
    fn default() -> Self {
        Self {
            high_intensity_duration_secs: 30,
            rest_exercise_duration_secs: 15,
            rest_set_duration_secs: 30,
            sets: 3,
        }
    }
}

impl WorkoutSettings {
    pub fn from_storage() -> Self {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("hiit_settings") {
                    if let Ok(settings) = serde_json::from_str::<WorkoutSettings>(&json) {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save_to_storage(&self) -> bool {
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
        let mut state = serializer.serialize_struct("WorkoutSettings", 4)?;
        state.serialize_field("high_intensity_duration_secs", &self.high_intensity_duration_secs)?;
        state.serialize_field("rest_exercise_duration_secs", &self.rest_exercise_duration_secs)?;
        state.serialize_field("rest_set_duration_secs", &self.rest_set_duration_secs)?;
        state.serialize_field("sets", &self.sets)?;
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
        }

        let helper = SettingsHelper::deserialize(deserializer)?;

        Ok(WorkoutSettings {
            high_intensity_duration_secs: helper.high_intensity_duration_secs,
            rest_exercise_duration_secs: helper.rest_exercise_duration_secs,
            rest_set_duration_secs: helper.rest_set_duration_secs,
            sets: helper.sets,
        })
    }
}

// Range slider component for reusability
#[component]
fn RangeSlider(
    label: String,
    #[prop(into)] value: RwSignal<u32>,
    min: u32,
    max: u32,
    step: u32,
    unit: String,
) -> impl IntoView {
    let id = format!("slider-{}", label.to_lowercase().replace(" ", "-"));

    let on_input = move |ev| {
        let new_value = event_target_value(&ev).parse::<u32>().unwrap_or_default();
        value.set(new_value);
    };

    view! {
        <div class="mb-6">
            <label for=id.clone() class="block text-sm font-medium text-gray-700 mb-2">
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
            <div class="flex justify-between text-xs text-gray-500 mt-1">
                <span>{min} {unit.clone()}</span>
                <span>{max} {unit.clone()}</span>
            </div>
        </div>
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    // Load saved settings or use defaults
    let initial_settings = WorkoutSettings::from_storage();

    // Create RwSignals for each setting
    let high_intensity_duration = RwSignal::new(initial_settings.high_intensity_duration_secs);
    let rest_exercise_duration = RwSignal::new(initial_settings.rest_exercise_duration_secs);
    let rest_set_duration = RwSignal::new(initial_settings.rest_set_duration_secs);
    let sets = RwSignal::new(initial_settings.sets);

    // Create a single effect that saves settings whenever any value changes
    Effect::new(move |_| {
        let new_settings = WorkoutSettings {
            high_intensity_duration_secs: high_intensity_duration.get(),
            rest_exercise_duration_secs: rest_exercise_duration.get(),
            rest_set_duration_secs: rest_set_duration.get(),
            sets: sets.get(),
        };
        new_settings.save_to_storage();
    });

    // Create preset settings functions
    let apply_preset = move |high: u32, rest: u32, set_rest: u32, sets_count: u32| {
        high_intensity_duration.set(high);
        rest_exercise_duration.set(rest);
        rest_set_duration.set(set_rest);
        sets.set(sets_count);
    };

    // Functions to check if the current settings match a preset
    let is_low_preset = move || {
        high_intensity_duration.get() == 30
            && rest_exercise_duration.get() == 15
            && rest_set_duration.get() == 30
            && sets.get() == 3
    };

    let is_mid_preset = move || {
        high_intensity_duration.get() == 45
            && rest_exercise_duration.get() == 10
            && rest_set_duration.get() == 15
            && sets.get() == 4
    };

    let is_high_preset = move || {
        high_intensity_duration.get() == 60
            && rest_exercise_duration.get() == 0
            && rest_set_duration.get() == 15
            && sets.get() == 6
    };

    // Set up handlers for the preset buttons
    let set_low_preset = move |_| apply_preset(30, 15, 30, 3);
    let set_mid_preset = move |_| apply_preset(45, 10, 15, 4);
    let set_high_preset = move |_| apply_preset(60, 0, 15, 6);

    view! {
        <div class="container mx-auto px-4 py-8 max-w-2xl">
            <h1 class="text-2xl md:text-3xl font-bold text-center text-gray-800 mb-6">Settings</h1>

            <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                <p class="text-sm text-gray-600 mb-4 text-center">
                    Settings are automatically saved as you adjust them.
                </p>

                <div class="flex justify-center gap-4 mb-6">
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
                    value=high_intensity_duration
                    min=5
                    max=300
                    step=5
                    unit="s".to_string()
                />

                <RangeSlider
                    label="Exercise Rest Duration".to_string()
                    value=rest_exercise_duration
                    min=0
                    max=300
                    step=5
                    unit="s".to_string()
                />

                <RangeSlider
                    label="Set Rest Duration".to_string()
                    value=rest_set_duration
                    min=0
                    max=300
                    step=5
                    unit="s".to_string()
                />

                <RangeSlider
                    label="Number of Sets".to_string()
                    value=sets
                    min=1
                    max=30
                    step=1
                    unit="".to_string()
                />
            </div>

            <div class="text-center mt-6">
                <a
                    href="/"
                    class="inline-block bg-gray-200 hover:bg-gray-300 text-gray-800 font-semibold py-2 px-4 rounded-lg transition-colors"
                >
                    Back to Workouts
                </a>
            </div>
        </div>
    }
}
