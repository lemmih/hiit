use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
      <div class="container py-8 px-4 mx-auto max-w-2xl">
        <h1 class="mb-6 text-2xl font-bold text-center text-gray-800 md:text-3xl">
          About HIIT Workout App
        </h1>

        <div class="p-6 mb-6 bg-white rounded-lg shadow-md">
          <p class="mb-4">
            {"This app is opinionated: High-intensity interval training is best, and exercises shouldn't require equipment."}
          </p>

          <p class="mb-4">
            {"Written in "}
            <a
              href="https://www.rust-lang.org"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              Rust
            </a>,
            built with {" "}
            <a
              href="https://nixos.org"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              Nix
            </a>, deployed with {" "}
            <a
              href="https://www.cloudflare.com"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              Cloudflare
            </a>, compiled to {" "}
            <a
              href="https://webassembly.org"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              WASM
            </a>, using the {" "}
            <a
              href="https://leptos.dev"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              Leptos
            </a> {" framework. Vibe coded using "}
            <a
              href="https://cursor.sh"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              Cursor
            </a>.
          </p>

          <p class="mb-4">
            Browse the source code at {" "}
            <a
              href="https://github.com/lemmih/hiit"
              class="font-medium text-blue-600 transition-colors hover:text-blue-800 hover:underline"
            >
              "github.com/lemmih/hiit"
            </a>.
          </p>
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
