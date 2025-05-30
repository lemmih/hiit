use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;

// Import our components
use crate::components::about::AboutPage;
use crate::components::exercises::ExercisesPage;
use crate::components::home::HomePage;
use crate::components::science::SciencePage;
use crate::components::settings::{SettingsPage, SettingsProvider};
use crate::components::timer::TimerPage;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <title>HIIT</title>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />

          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
        </head>
        <body class="bg-gray-100">
          <App />
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Stylesheet href="/style.css" />
      <Link rel="icon" type_="image/x-icon" href="/favicon.ico" />

      <div class="min-h-screen bg-gray-100">
        <div class="mx-auto max-w-4xl bg-white shadow-sm">
          <SettingsProvider>
            <Router>
              <main>
                <Routes fallback=|| {
                  view! { <div class="p-4 text-center">Page Not Found</div> }
                }>
                  <Route path=path!("/") view=move || view! { <HomePage /> } />
                  <Route path=path!("/timer/:id") view=move || view! { <TimerPage /> } />
                  <Route path=path!("/about") view=move || view! { <AboutPage /> } />
                  <Route path=path!("/settings") view=move || view! { <SettingsPage /> } />
                  <Route path=path!("/science") view=move || view! { <SciencePage /> } />
                  <Route path=path!("/exercises") view=move || view! { <ExercisesPage /> } />
                </Routes>
              </main>
            </Router>
          </SettingsProvider>
        </div>
      </div>
    }
}
