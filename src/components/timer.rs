use crate::data::routines::get_routines;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_use::{use_interval_with_options, UseIntervalOptions, UseIntervalReturn};
use std::time::Duration;

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
            <div class="p-4 max-w-lg mx-auto">
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-xl font-bold text-center mb-4">Exercise Not Found</h2>
                    <div class="text-center">
                        <a href="/" class="text-blue-600 hover:text-blue-800">Return to Home</a>
                    </div>
                </div>
            </div>
        }
        .into_any();
    };
    let routine = StoredValue::new(r);

    // Initialize interval (1 second = 1000ms)
    let UseIntervalReturn {
        counter,
        pause,
        resume,
        is_active,
        reset,
        ..
    } = use_interval_with_options(1000, UseIntervalOptions::default().immediate(false));

    let time_left = move || routine.read_value().duration() - Duration::from_secs(counter.get());

    // Format time as MM:SS
    let format_time = move |seconds: u32| {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        format!("{:02}:{:02}", minutes, remaining_seconds)
    };

    // View
    view! {
        <div class="p-4 max-w-lg mx-auto">
            {move || {
                let r = routine.get_value();
                view! {
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <div class="flex justify-between items-center mb-6">
                            <a href="/" class="text-blue-600 hover:text-blue-800">
                                <span class="text-xl">&lt;</span>
                                Back
                            </a>
                            <h2 class="text-xl font-bold text-center flex-grow">
                                {r.name.clone()}
                            </h2>
                            <div class="w-6"></div>
                        </div>

                        <div class="mb-6">
                            <div class="text-gray-700 mb-2">{r.description.clone()}</div>
                            <div class="flex justify-between text-sm">
                                <span class="font-semibold">{r.target_area.clone()}</span>
                                <span class="bg-blue-100 text-blue-800 px-2 py-0.5 rounded">
                                    {r.intensity.clone()}Intensity
                                </span>
                            </div>
                        </div>

                        <div class="mb-8">
                            <div class="text-5xl font-bold text-center mb-2">
                                {move || format_time(time_left().as_secs() as u32)}
                            </div>
                            <div class="h-2 bg-gray-200 rounded-full overflow-hidden">
                                <div
                                    class="h-full bg-gradient-to-r from-blue-500 to-indigo-600 transition-all duration-1000"
                                    style:width=move || {
                                        let duration = routine.get_value().duration().as_secs();
                                        format!(
                                            "{}%",
                                            (time_left().as_secs() as f32 / duration as f32) * 100.0,
                                        )
                                    }
                                ></div>
                            </div>
                            {move || {
                                let routine = routine.get_value();
                                let elapsed = routine.duration().as_secs_f64() - time_left().as_secs_f64();
                                if let Some((current, next, time_in_stage)) = routine.stage_at_t(elapsed) {
                                    view! {
                                        <div class="mt-4 text-center">
                                            <div class="text-lg font-semibold">
                                                {current.label.clone()}
                                                {if current.is_high_intensity {
                                                    " (High Intensity)"
                                                } else {
                                                    ""
                                                }}
                                            </div>
                                            <div class="text-sm text-gray-600">
                                                {format!("Time in stage: {:.1}s", time_in_stage)}
                                            </div>
                                            {if let Some(next_stage) = next {
                                                view! {
                                                    <div class="text-sm text-gray-500 mt-1">
                                                        "Next: " {next_stage.label.clone()}
                                                    </div>
                                                }
                                                .into_any()
                                            } else {
                                                view! {
                                                    <div class="text-sm text-gray-500 mt-1">
                                                        "Final Stage"
                                                    </div>
                                                }
                                                .into_any()
                                            }}
                                        </div>
                                    }
                                    .into_any()
                                } else {
                                    view! {
                                        <div class="mt-4 text-center text-gray-500">
                                            "Workout Complete"
                                        </div>
                                    }
                                    .into_any()
                                }
                            }}
                        </div>

                        <div class="text-center">
                            <div class="flex justify-center space-x-4">
                                {if is_active.get() {
                                    view! {
                                        <button
                                            class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 transition-colors"
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
                                            class="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 transition-colors"
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
                                    class="bg-gray-500 text-white px-4 py-2 rounded hover:bg-gray-600 transition-colors"
                                    on:click={
                                        let reset = reset.clone();
                                        move |_| reset()
                                    }
                                >
                                    "Reset"
                                </button>
                            </div>
                        </div>
                    </div>
                }
                    .into_any()
            }}
        </div>
    }.into_any()
}
