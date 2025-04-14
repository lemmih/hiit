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
          <h1 class="text-2xl font-bold text-center text-gray-800 md:text-3xl">HIIT Workout App</h1>
          <p class="mt-2 text-center text-gray-600">Choose a workout to begin</p>
        </header>

        <div class="grid grid-cols-1 gap-4 px-4 sm:grid-cols-2 lg:grid-cols-3">
          {routines
            .into_iter()
            .map(|ex| {
              view! {
                <div class="h-full">
                  <RoutineCard routine=StoredValue::new(ex) on_click=card_callback />
                </div>
              }
            })
            .collect::<Vec<_>>()}
        </div>

        <footer class="pt-4 mt-8 border-t border-gray-200">
          <div class="flex justify-center space-x-6 text-sm text-gray-600">
            <a href="/about" class="transition-colors hover:text-gray-900">
              About
            </a>
            <a href="/settings" class="transition-colors hover:text-gray-900">
              Settings
            </a>
            <a href="/science" class="transition-colors hover:text-gray-900">
              Science
            </a>
            <a href="/exercises" class="transition-colors hover:text-gray-900">
              Exercises
            </a>
          </div>
        </footer>
      </div>
    }
}
