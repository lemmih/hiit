use chrono::{DateTime, Utc};
use itertools::Itertools;
use leptos::prelude::*;
use std::time::Duration;

use super::settings::SettingsContext;

#[derive(Clone)]
pub struct Stage {
    pub duration: Duration,
    pub is_high_intensity: bool,
    pub label: String,
}

#[derive(Clone)]
pub struct Routine {
    pub id: String,
    pub name: String,
    pub exercises: Vec<String>,
}

impl Routine {
    pub fn description(&self) -> String {
        self.exercises.join(", ")
    }

    pub fn duration(&self) -> Duration {
        self.stages().iter().map(|stage| stage.duration).sum()
    }

    #[allow(unstable_name_collisions)]
    pub fn stages(&self) -> Vec<Stage> {
        // Get global settings
        let SettingsContext { settings, .. } = expect_context::<SettingsContext>();

        // Prepare stage
        let prepare_stage = Stage {
            duration: Duration::from_secs(10),
            is_high_intensity: false,
            label: "Prepare".to_string(),
        };

        // Create iterator of exercise stages
        let exercise_stages = self.exercises.iter().map(|exercise| Stage {
            duration: Duration::from_secs(settings.get().high_intensity_duration_secs as u64),
            is_high_intensity: true,
            label: exercise.clone(),
        });

        // Create rest stage
        let rest_stage = Stage {
            duration: Duration::from_secs(settings.get().rest_exercise_duration_secs as u64),
            is_high_intensity: false,
            label: "Rest".to_string(),
        };

        // Create set break stage
        let set_break_stage = Stage {
            duration: Duration::from_secs(settings.get().rest_set_duration_secs as u64),
            is_high_intensity: false,
            label: "Set Break".to_string(),
        };

        // Create a single set of exercise stages with rests
        let single_set: Vec<Stage> = exercise_stages.intersperse(rest_stage).collect();

        // Create iterator of sets and intersperse set breaks
        let all_stages = std::iter::repeat(single_set)
            .take(settings.get().sets as usize)
            .intersperse(vec![set_break_stage])
            .flatten();

        // Combine prepare stage with the rest of the stages
        std::iter::once(prepare_stage).chain(all_stages).collect()
    }

    pub fn stage_at_t(&self, t: f64) -> Option<(usize, Stage, Option<Stage>, f64)> {
        let stages = self.stages();
        let mut cumulative_duration = 0.0;

        for i in 0..stages.len() {
            let current_stage = &stages[i];
            let stage_start = cumulative_duration;
            cumulative_duration += current_stage.duration.as_secs_f64();

            if t < cumulative_duration {
                let next_stage = if i + 1 < stages.len() {
                    Some(stages[i + 1].clone())
                } else {
                    None
                };
                let time_in_stage = t - stage_start;
                return Some((i, current_stage.clone(), next_stage, time_in_stage));
            }
        }

        None
    }
}

fn format_last_completion(last_completion: Option<DateTime<Utc>>, current_time: DateTime<chrono::Local>) -> String {
    if let Some(completion_time) = last_completion {
        // Convert UTC time to local time
        let local_completion = completion_time.with_timezone(current_time.offset());

        // Get the date part only (year, month, day)
        let today = current_time.date_naive();
        let completion_date = local_completion.date_naive();

        // Calculate days between dates
        let days_diff = today.signed_duration_since(completion_date).num_days();

        match days_diff {
            0 => "done today".to_string(),
            1 => "done yesterday".to_string(),
            n if n > 1 => format!("done {} days ago", n),
            _ => "".to_string(), // Handles future dates, though this shouldn't happen
        }
    } else {
        "".to_string()
    }
}

// Add this helper function above the RoutineCard component
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

#[component]
pub fn RoutineCard(
    #[prop(into)] routine: StoredValue<Routine>,
    #[prop(optional)] on_click: Option<Callback<Routine>>,
) -> impl IntoView {
    let routine_for_click = routine.get_value();
    let handle_click = move |_| {
        if let Some(callback) = on_click {
            callback.run(routine_for_click.clone());
        }
    };

    let SettingsContext { settings, .. } = expect_context::<SettingsContext>();
    let last_completion = move || {
        settings
            .get()
            .routine_completions
            .get(&routine.get_value().name)
            .cloned()
    };

    // Get current time once for the component
    let current_time = chrono::Local::now();

    view! {
      <div
        class="overflow-hidden h-full bg-white rounded-lg shadow-md transition-shadow transition-transform duration-300 duration-300 transform cursor-pointer hover:shadow-lg hover:scale-102"
        on:click=handle_click
      >
        <div class="py-3 px-4 bg-gradient-to-r from-blue-500 to-indigo-600">
          <h3 class="text-xl font-bold text-white">{routine.get_value().name.clone()}</h3>
        </div>
        <div class="p-4">
          <p class="mb-3 text-gray-700">{routine.get_value().description()}</p>
          <div class="flex justify-between items-center">
            <span class="py-0.5 px-2.5 text-xs font-semibold text-blue-800 bg-blue-100 rounded">
              {move || format_duration(routine.get_value().duration())}
            </span>
            {move || {
              if let Some(completion_time) = last_completion() {
                view! {
                  <span class="text-sm text-gray-500">
                    {format_last_completion(Some(completion_time), current_time)}
                  </span>
                }
                  .into_any()
              } else {
                view! { <div></div> }.into_any()
              }
            }}
          </div>
        </div>
      </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration as ChronoDuration, TimeZone};

    #[test]
    fn test_format_last_completion_none() {
        // Test with None input
        let current_time = chrono::Local::now();
        let result = format_last_completion(None, current_time);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_last_completion_today() {
        // Create a fixed "now" time for testing
        let current_time = chrono::Local::now();

        // Create a completion time on the same day
        let completion_time = chrono::Utc::now();

        let result = format_last_completion(Some(completion_time), current_time);
        assert_eq!(result, "done today");
    }

    #[test]
    fn test_format_last_completion_yesterday() {
        // Create a fixed "now" time for testing
        let current_time = chrono::Local::now();

        // Create a completion time from yesterday (24 hours ago)
        let completion_time = chrono::Utc::now() - ChronoDuration::days(1);

        let result = format_last_completion(Some(completion_time), current_time);
        assert_eq!(result, "done yesterday");
    }

    #[test]
    fn test_format_last_completion_days_ago() {
        // Create a fixed "now" time for testing
        let current_time = chrono::Local::now();

        // Create a completion time from 3 days ago
        let completion_time = chrono::Utc::now() - ChronoDuration::days(3);

        let result = format_last_completion(Some(completion_time), current_time);
        assert_eq!(result, "done 3 days ago");
    }

    #[test]
    fn test_format_last_completion_future() {
        // Create a fixed "now" time for testing
        let current_time = chrono::Local::now();

        // Create a completion time in the future (tomorrow)
        let completion_time = chrono::Utc::now() + ChronoDuration::days(1);

        let result = format_last_completion(Some(completion_time), current_time);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_last_completion_exact_dates() {
        // Test with exact dates to ensure consistent behavior
        // Use a specific timezone for consistent testing
        let tz = chrono::FixedOffset::east_opt(0).unwrap(); // UTC

        // Create a fixed current time: 2023-06-15 12:00:00 UTC
        let current_time_fixed = tz.with_ymd_and_hms(2023, 6, 15, 12, 0, 0).unwrap();
        // Convert to Local for the function call
        let current_time = current_time_fixed.with_timezone(&chrono::Local::now().timezone());

        // Test same day
        let same_day = chrono::Utc.with_ymd_and_hms(2023, 6, 15, 8, 0, 0).unwrap();
        assert_eq!(format_last_completion(Some(same_day), current_time), "done today");

        // Test yesterday
        let yesterday = chrono::Utc.with_ymd_and_hms(2023, 6, 14, 8, 0, 0).unwrap();
        assert_eq!(format_last_completion(Some(yesterday), current_time), "done yesterday");

        // Test 5 days ago
        let days_ago = chrono::Utc.with_ymd_and_hms(2023, 6, 10, 8, 0, 0).unwrap();
        assert_eq!(format_last_completion(Some(days_ago), current_time), "done 5 days ago");

        // Test future date
        let future = chrono::Utc.with_ymd_and_hms(2023, 6, 16, 8, 0, 0).unwrap();
        assert_eq!(format_last_completion(Some(future), current_time), "");
    }

    #[test]
    fn test_format_last_completion_yesterday_near_midnight() {
        // Test the edge case where completion time was yesterday but very close to midnight
        // Use a specific timezone for consistent testing
        let tz = chrono::FixedOffset::east_opt(0).unwrap(); // UTC

        // Today at 1am
        let current_time_fixed = tz.with_ymd_and_hms(2023, 6, 15, 1, 0, 0).unwrap();
        // Convert to Local for the function call
        let current_time = current_time_fixed.with_timezone(&chrono::Local::now().timezone());

        // Yesterday at 11pm (only 2 hours before "now", but still "yesterday" by date)
        let completion_time = chrono::Utc.with_ymd_and_hms(2023, 6, 14, 23, 0, 0).unwrap();

        let result = format_last_completion(Some(completion_time), current_time);
        assert_eq!(result, "done yesterday");
    }

    use quickcheck::{Arbitrary, Gen, TestResult};
    use quickcheck_macros::quickcheck;

    // Generate arbitrary UTC DateTime values within a reasonable range
    #[derive(Clone, Debug)]
    struct ArbitraryDateTime(DateTime<Utc>);

    impl Arbitrary for ArbitraryDateTime {
        fn arbitrary(g: &mut Gen) -> Self {
            // Generate a timestamp between 1990-01-01 and 2050-01-01
            // This gives a reasonable range without going into extreme values
            let min_seconds = 631152000; // 1990-01-01
            let max_seconds = 2524608000; // 2050-01-01

            let seconds = u32::arbitrary(g) % (max_seconds - min_seconds) + min_seconds;

            // Create a DateTime from this timestamp
            let dt = chrono::Utc.timestamp_opt(seconds as i64, 0).unwrap();
            ArbitraryDateTime(dt)
        }
    }

    // Arbitrary DateTime for Local time
    #[derive(Clone, Debug)]
    struct ArbitraryLocalDateTime(DateTime<chrono::Local>);

    impl Arbitrary for ArbitraryLocalDateTime {
        fn arbitrary(g: &mut Gen) -> Self {
            // Generate an arbitrary UTC DateTime first
            let arbitrary_utc = ArbitraryDateTime::arbitrary(g);

            // Convert to local time
            let local_dt = arbitrary_utc.0.with_timezone(&chrono::Local::now().timezone());
            ArbitraryLocalDateTime(local_dt)
        }
    }

    #[quickcheck]
    fn test_format_last_completion_doesnt_panic(
        maybe_completion: Option<ArbitraryDateTime>,
        current_time: ArbitraryLocalDateTime,
    ) -> TestResult {
        // Convert Option<ArbitraryDateTime> to Option<DateTime<Utc>>
        let completion_time = maybe_completion.map(|dt| dt.0);

        // Use the arbitrary current time
        let current_time = current_time.0;

        // Just call the function and verify it doesn't panic
        let _ = format_last_completion(completion_time, current_time);

        // If we got here, the function didn't panic
        TestResult::passed()
    }
}
