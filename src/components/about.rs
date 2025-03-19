use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8 max-w-2xl">
            <h1 class="text-2xl md:text-3xl font-bold text-center text-gray-800 mb-6">
                About HIIT Workout App
            </h1>

            <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                <p class="mb-4">
                    The HIIT Workout App is designed to help you perform effective high-intensity interval training
                    workouts from anywhere, with no equipment required.
                </p>

                <p class="mb-4">
                    High-Intensity Interval Training (HIIT) involves alternating between short, intense bursts of
                    exercise and periods of rest or lower-intensity activity. This approach has been shown to
                    improve cardiovascular health, increase metabolism, and burn calories efficiently.
                </p>

                <p class="mb-4">
                    Our app provides carefully designed workout routines with timed intervals,
                    clear instructions, and tracking to help you reach your fitness goals.
                </p>

                <p>
                    This app was created with Rust and Leptos, focusing on performance and usability
                    to provide you with the best possible workout experience.
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
