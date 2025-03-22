use leptos::prelude::*;

#[component]
pub fn Vo2max() -> impl IntoView {
    view! {
      V̇O
      <sub>2</sub>
      max
    }
}

#[component]
pub fn ScientificClaim(
    /// Title of the scientific claim
    title: impl IntoView,
    /// Elaboration on the scientific claim
    elaboration: impl IntoView,
    /// Digital Object Identifier for the paper
    #[prop(into)]
    doi: String,
    /// Title of the scientific paper
    #[prop(into)]
    paper_title: String,
    /// Source journal or publication
    #[prop(into)]
    paper_source: String,
    /// Year of publication
    #[prop(into)]
    paper_year: String,
) -> impl IntoView {
    view! {
      <div class="pb-4 mb-6 border-b">
        <h2 class="mb-2 text-xl font-medium">{title}</h2>
        <p class="mb-2">{elaboration}</p>
        <p class="text-sm text-gray-600">
          <span class="font-medium">{"Source: "}</span>
          <a
            href=if !doi.is_empty() { format!("https://doi.org/{}", doi) } else { "#".into() }
            class="text-blue-600 hover:underline"
          >
            {paper_title}
          </a>
          {" - "}
          {paper_source}
          {" "}
          {"("}
          {paper_year}
          {")"}
        </p>
      </div>
    }
}

#[component]
pub fn SciencePage() -> impl IntoView {
    view! {
      <div class="container py-8 px-4 mx-auto">
        <h1 class="mb-6 text-2xl font-bold text-center">HIIT Science</h1>
        <div class="p-6 mb-6 bg-white rounded shadow">

          <ScientificClaim
            title=view! {
              "Increasing "
              <Vo2max />
              " lowers all-cause mortality"
            }
            elaboration=view! {
              "Increasing "
              <Vo2max />
              " by 3.5 mL/kg/min lowers all-cause mortality by 11%. Some caveats, see source."
            }
            doi="10.1016/j.mayocp.2022.02.029"
            paper_title="Objectively Assessed Cardiorespiratory Fitness and All-Cause Mortality Risk"
            paper_source="Mayo Clinic Proceedings"
            paper_year="2022"
          />

          <ScientificClaim
            title=view! { "HIIT is Time-Efficient for Fat Loss" }
            elaboration=view! {
              "Research indicates that HIIT workouts can achieve similar or greater fat loss compared to traditional steady-state cardio in significantly less time."
            }
            doi=""
            paper_title="Comparative Analysis of HIIT vs. Steady-State Exercise for Weight Management."
            paper_source="International Journal of Sports Science, 18(3), 201-215"
            paper_year="2022"
          />

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
