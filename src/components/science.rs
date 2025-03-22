use leptos::prelude::*;

#[component]
pub fn SciencePage() -> impl IntoView {
    view! {
      <div class="container py-8 px-4 mx-auto">
        <h1 class="mb-6 text-2xl font-bold text-center">HIIT Science</h1>
        <div class="p-6 mb-6 bg-white rounded shadow">
          <p class="mb-4 text-center">This page is coming soon.</p>
          <p class="text-center">"We're developing content about the science of HIIT workouts."</p>
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
