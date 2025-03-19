use crate::components::routine_card::{Routine, RoutineCard};
use crate::data::routines::get_routines;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn HomePage() -> impl IntoView {
    let routines = get_routines();

    let navigate = use_navigate();

    let on_card_click = move |routine: Routine| {
        log::info!("Routine selected: {}", routine.name);
        // Navigate to the timer page for this routine
        navigate(&format!("/timer/{}", routine.id), Default::default());
    };

    // Create the callback once
    let card_callback = Callback::new(on_card_click);

    view! {
        <div class="pb-8">
            <header class="py-6">
                <h1 class="text-2xl md:text-3xl font-bold text-center text-gray-800">
                    HIIT Workout App
                </h1>
                <p class="text-center text-gray-600 mt-2">Choose a workout to begin</p>
            </header>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 px-4">
                {routines
                    .into_iter()
                    .map(|ex| {
                        view! {
                            <div class="h-full">
                                <RoutineCard routine=ex on_click=card_callback />
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <footer class="mt-8 pt-4 border-t border-gray-200">
                <div class="flex justify-center space-x-6 text-sm text-gray-600">
                    <a href="/about" class="hover:text-gray-900 transition-colors">
                        About
                    </a>
                    <a href="/settings" class="hover:text-gray-900 transition-colors">
                        Settings
                    </a>
                    <a href="/science" class="hover:text-gray-900 transition-colors">
                        Science
                    </a>
                    <a href="/exercises" class="hover:text-gray-900 transition-colors">
                        Exercises
                    </a>
                </div>
            </footer>
        </div>
    }
}
