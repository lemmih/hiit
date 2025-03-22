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
            title=view! {
              "HIIT increases "
              <Vo2max />
            }
            elaboration=view! {
              "HIIT is more effective than endurance training at increasing "
              <Vo2max />
              ". Some caveats, see source."
            }
            doi="10.1007/s40279-015-0365-0"
            paper_title="Effectiveness of High-Intensity Interval Training and Continuous Endurance Training for VO2max Improvements: A Systematic Review and Meta-Analysis of Controlled Trials"
            paper_source="Sports Medicine"
            paper_year="2015"
          />

          <ScientificClaim
            title=view! {
              "More exercise ⇒ more "
              <Vo2max />
              " gains"
            }
            elaboration=view! {
              "Exercising twice per week yields better "
              <Vo2max />
              " gains than exercising once per week. The upper limit has yet to be found."
            }
            doi="10.3390/brainsci13040571"
            paper_title="Effects of High-Intensity Interval Training on Executive Functions in College Students: Evidence from Different Doses"
            paper_source="Brain Sciences"
            paper_year="2023"
          />

          <ScientificClaim
            title=view! { "HIIT is time-efficient" }
            elaboration=view! {
              "While endurance training yields similar "
              <Vo2max />
              " gains, HIIT is more time-efficient. Linked study shows 28 minutes of HIIT yielded similar results to 38 minutes of endurance training."
            }
            doi="10.3389/fphys.2018.01012"
            paper_title="High-Intensity Interval Training Performed by Young Athletes: A Systematic Review and Meta-Analysis"
            paper_source="Frontiers in Physiology"
            paper_year="2018"
          />

          <ScientificClaim
            title=view! { "HIIT increases HDL-C" }
            elaboration=view! {
              "Increase in HDL-C of 0.2 mmol/L after 8 weeks of HIIT. Some caveats, see source."
            }
            doi="10.1519/JSC.0b013e318198fd28"
            paper_title="The Effect of a High-Intensity Interval Training Program on High-Density Lipoprotein Cholesterol in Young Men"
            paper_source="Journal of Strength and Conditioning Research"
            paper_year="2009"
          />

          <ScientificClaim
            title=view! {
              <Vo2max />
              " is proportional to maximum heart rate over heart rate at rest"
            }
            elaboration=view! {
              "Measuing "
              <Vo2max />
              " is difficult, but HR_max/HR_rest can be used as a proxy. Some caveats, see source."
            }
            doi="10.1007/s00421-003-0988-y"
            paper_title="Estimation of VO2max from the ratio between HRmax and HRrest – the Heart Rate Ratio Method"
            paper_source="European Journal of Applied Physiology"
            paper_year="2004"
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
