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
                    {"This app is opinionated: High-intensity interval training is best, and exercises shouldn't require equipment."}
                </p>

                <p class="mb-4">
                    {"Written in "}
                    <a
                        href="https://www.rust-lang.org"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        Rust
                    </a>,
                    built with {" "}
                    <a
                        href="https://nixos.org"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        Nix
                    </a>, deployed with {" "}
                    <a
                        href="https://www.cloudflare.com"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        Cloudflare
                    </a>, compiled to {" "}
                    <a
                        href="https://webassembly.org"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        WASM
                    </a>, using the {" "}
                    <a
                        href="https://leptos.dev"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        Leptos
                    </a> {" framework. Vibe coded using "}
                    <a
                        href="https://cursor.sh"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        Cursor
                    </a>.
                </p>

                <p class="mb-4">
                    Browse the source code at {" "}
                    <a
                        href="https://github.com/lemmih/hiit"
                        class="text-blue-600 hover:text-blue-800 font-medium hover:underline transition-colors"
                    >
                        "github.com/lemmih/hiit"
                    </a>.
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
