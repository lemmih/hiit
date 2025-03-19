use leptos::prelude::*;

#[component]
pub fn ExercisesPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-2xl font-bold text-center mb-6">HIIT Exercises</h1>
            <div class="bg-white p-6 rounded shadow mb-6">
                <p class="text-center mb-4">This page is coming soon.</p>
                <p class="text-center">
                    "We're developing a comprehensive library of HIIT exercises."
                </p>
            </div>
            <div class="text-center">
                <a
                    href="/"
                    class="inline-block bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                >
                    Back to Workouts
                </a>
            </div>
        </div>
    }
}
