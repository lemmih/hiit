use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;

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
            <body class="bg-sky-100">
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

        <div class="bg-white" style:box-shadow="0 0px 5px rgba(0, 0, 0, 0.4)">
            <div class="max-w-4xl mx-auto p-4">
                <Router>
                    <main>
                        <Routes fallback=|| "Not found">
                            <Route
                                path=path!("/")
                                view=move || {
                                    view! {}
                                }
                            />
                        </Routes>
                    </main>
                </Router>
            </div>
        </div>
    }
}
