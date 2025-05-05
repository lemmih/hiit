use leptos::prelude::*;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash)]
pub struct Exercise {
    pub group: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

pub const EXERCISES: [Exercise; 22] = [
    Exercise {
        group: "Upper Body",
        name: "Push ups",
        description: "Start in a plank position with hands shoulder-width apart, \
             body straight from head to heels. Lower your body by bending \
             your elbows, keeping your core tight and back straight. Lower \
             until your chest nearly touches the ground, then push back up \
             to starting position. Keep your neck neutral by looking slightly \
             ahead rather than down.",
    },
    Exercise {
        group: "Upper Body",
        name: "Hammer curls",
        description: "Stand with feet hip-width apart, knees slightly bent, and core engaged. \
             Hold dumbbells or weighted objects in each hand with arms fully extended, \
             palms facing each other (neutral grip). Keep your elbows close to your sides \
             throughout the movement. Exhale as you curl the weights toward your shoulders \
             by bending at the elbows, maintaining the neutral grip. Focus on contracting \
             your biceps at the top of the movement. Pause briefly at the top, then inhale \
             as you slowly lower the weights back to the starting position. For proper form, \
             keep your upper arms stationary, back straight, and wrists neutral. \
             For increased difficulty: use heavier weights, slow down the lowering phase, \
             or perform alternating arms. For reduced intensity: use lighter weights, \
             perform seated hammer curls, or reduce the range of motion.",
    },
    Exercise {
        group: "Upper Body",
        name: "Skull crushers",
        description: "Lie on a bench or mat with knees bent and feet flat on the floor. \
             Hold dumbbells or a weighted object with both hands, arms extended above your chest, \
             palms facing each other. Keep your upper arms stationary and perpendicular to the floor. \
             Slowly bend your elbows to lower the weight behind your head until your forearms \
             are approximately parallel to the floor. Keep your elbows pointing forward and close \
             to your head. Pause briefly, then engage your triceps to extend your arms back to \
             the starting position. Exhale as you extend your arms, inhale as you lower the weight. \
             For increased difficulty: use heavier weights, slow down the lowering phase, \
             or perform single-arm skull crushers. For reduced intensity: use lighter weights, \
             perform seated skull crushers, or reduce the range of motion. \
             For a variation, try performing them with a resistance band or with your feet elevated.",
    },
    Exercise {
        group: "Upper Body",
        name: "Overhead triceps",
        description: "Stand with feet shoulder-width apart and core engaged. Hold a dumbbell, \
             water bottle, or any weighted object with both hands. \
             Raise your arms overhead, fully extending them with the weight positioned \
             behind your head. Your elbows should be close to your ears, pointing forward. \
             Slowly bend your elbows to lower the weight behind your head until your forearms \
             are approximately parallel to the floor or slightly lower. \
             Pause briefly, then engage your triceps to extend your arms back to the starting position. \
             Keep your upper arms stationary and close to your head throughout the movement. \
             Exhale as you extend your arms, inhale as you lower the weight. \
             For less intensity, use a lighter weight; for more challenge, increase the weight \
             or slow down the tempo, especially during the lowering phase.",
    },
    Exercise {
        group: "Upper Body",
        name: "Bicep curls",
        description: "Stand with feet hip-width apart, knees slightly bent, and core engaged. \
             Hold dumbbells, water bottles, or other weighted objects in each hand with arms fully extended, \
             palms facing forward (supinated grip). Keep your elbows close to your sides throughout the movement. \
             Exhale as you curl the weights toward your shoulders by bending at the elbows. \
             Focus on contracting your biceps at the top of the movement and avoid using momentum. \
             Pause briefly at the top, then inhale as you slowly lower the weights back to the starting position. \
             For proper form, keep your upper arms stationary, back straight, and wrists neutral. \
             To increase difficulty, use heavier weights or slow down the lowering phase. \
             For a variation, try alternating arms or rotating your wrists during the curl (hammer to supinated).",
    },
    Exercise {
        group: "Core",
        name: "Low plank",
        description: "Start by getting into a forearm plank position: forearms flat on the ground, \
             elbows directly under shoulders, body forming a straight line from head to heels. \
             Keep your core engaged, back flat (avoid sagging or lifting hips), and neck neutral \
             by looking at the floor slightly ahead of your hands. Squeeze your glutes and quads \
             to maintain proper form. Hold this position while breathing steadily.",
    },
    Exercise {
        group: "Core",
        name: "Russian twists",
        description: "Sit on the floor with knees bent and feet lifted slightly off the ground, \
             keeping your back at a 45-degree angle to the floor. Clasp your hands together \
             in front of your chest. Engage your core and lift your feet off the ground. \
             Keeping your back straight, rotate your torso from side to side, touching your \
             hands to the ground on each side. The movement should come from your core, not \
             your arms. Keep your feet steady and maintain balance throughout the exercise.",
    },
    Exercise {
        group: "Core",
        name: "Crunches",
        description: "Lie on your back with knees bent and feet flat on the floor, hip-width apart. \
             Place your hands behind your head with elbows pointed outward, or cross arms over your chest. \
             Engage your core by drawing your belly button toward your spine. Exhale as you lift your \
             upper back off the floor, keeping your neck neutral and chin slightly tucked. \
             Focus on using your abdominal muscles to perform the movement, not your neck. \
             Lift only until your shoulder blades clear the floor (about 30 degrees). \
             Hold briefly at the top, then inhale as you slowly lower back down with control. \
             For a greater challenge, lift your feet off the floor or extend your arms overhead.",
    },
    Exercise {
        group: "Core",
        name: "Bicycle crunches",
        description: "Lie on your back with knees bent at 90 degrees and feet lifted off the floor. \
             Place your hands behind your head with elbows wide, keeping your neck relaxed. \
             Engage your core by pressing your lower back into the floor. \
             Bring your right elbow toward your left knee while simultaneously extending your right leg. \
             Then switch sides, bringing your left elbow to your right knee while extending your left leg. \
             Continue alternating sides in a pedaling motion. Exhale as you rotate, inhale as you switch sides. \
             Focus on the rotation of your torso rather than just the elbow-to-knee connection. \
             Keep movements controlled rather than rushing through repetitions. \
             For less intensity, keep the movements smaller; for more challenge, extend the legs fully.",
    },
    Exercise {
        group: "Lower Body",
        name: "Squats",
        description: "Stand with feet shoulder-width apart, toes pointing slightly outward. \
             Keep your chest up, back straight, and core engaged. Begin the movement by \
             pushing your hips back as if sitting in a chair, while bending your knees. \
             Lower yourself until your thighs are parallel to the ground, keeping your \
             weight in your heels and knees tracking over (but not past) your toes. \
             Keep your back straight and head neutral throughout the movement. \
             Push through your heels to return to the starting position, squeezing \
             your glutes at the top.",
    },
    Exercise {
        group: "Lower Body",
        name: "Lunges",
        description: "Start standing with feet hip-width apart and hands on hips or by your sides. \
             Step forward with one leg, lowering your hips until both knees are bent at approximately 90 degrees. \
             Your front knee should be directly above your ankle, not pushed forward past your toes. \
             Your back knee should hover just above the floor without touching it. \
             Keep your weight in the heel of your front foot as you push back up to the starting position. \
             Maintain an upright posture throughout the movement, with shoulders back and core engaged. \
             Alternate legs, or complete all reps on one side before switching. \
             Exhale as you exert effort (pushing back up), inhale as you lower. \
             For less intensity, take a smaller step or don't lower as deep. \
             For more challenge, hold weights, perform walking lunges, or add a jump as you switch legs.",
    },
    Exercise {
        group: "Lower Body",
        name: "Calf raises",
        description: "Stand with feet hip-width apart, toes pointing forward, and core engaged. \
             For stability, hold onto a wall or chair if needed. Slowly rise up onto the balls \
             of your feet, lifting your heels as high as possible while keeping your body straight. \
             Hold at the top position for a moment, focusing on the contraction in your calves. \
             Then slowly lower your heels back to the ground in a controlled motion. \
             For added difficulty, perform on a step with heels hanging off the edge to increase range of motion, \
             or try single-leg calf raises for greater intensity.",
    },
    Exercise {
        group: "Lower Body",
        name: "Froggy glute lifts",
        description: "Lie face down on a mat with your forehead resting on your hands. \
             Bend your knees to about 90 degrees and turn them outward, with the soles of your feet \
             facing each other in a frog-like position. Keep your ankles touching or close together. \
             Engage your core and squeeze your glutes to lift both knees off the ground. \
             Lift only as high as you can while maintaining proper form - even a small lift is effective. \
             Hold the lifted position for 1-2 seconds at the top, focusing on the contraction in your glutes. \
             Lower your knees back to the starting position with control. \
             For increased difficulty, hold the top position longer or add small pulses. \
             For reduced intensity, perform smaller movements or take shorter breaks between repetitions.",
    },
    Exercise {
        group: "Full Body",
        name: "Mountain climber",
        description: "Start in a high plank position with hands directly under shoulders, \
             arms straight, and body forming a straight line from head to heels. \
             Engage your core and keep your shoulders stable. Rapidly drive one knee toward \
             your chest, then quickly switch legs in a running motion while maintaining the plank position. \
             Keep your hips low and avoid letting them rise too high. Breathe rhythmically \
             and maintain a steady pace. Focus on keeping your wrists aligned under your shoulders \
             to reduce strain. For increased intensity, speed up the pace while maintaining proper form.",
    },
    Exercise {
        group: "Full Body",
        name: "Superman",
        description: "Lie face down on a mat with arms extended forward and legs straight. \
             Keep your neck in a neutral position by looking down at the floor. \
             Simultaneously lift your arms, chest, and legs off the ground, engaging your lower back muscles. \
             Hold the raised position for 2-3 seconds, squeezing your back muscles at the top of the movement. \
             Lower back down with control and repeat. Breathe out as you lift and in as you lower. \
             For a modified version, lift only your arms or only your legs. For increased difficulty, \
             hold the raised position longer or add small pulses at the top of the movement.",
    },
    Exercise {
        group: "Full Body",
        name: "Inchworm",
        description: "Start standing with feet hip-width apart. Hinge at the hips and place your hands on \
             the floor in front of you while keeping your legs as straight as possible (slight bend is okay). \
             Walk your hands forward until you reach a high plank position with shoulders over wrists and body \
             forming a straight line. Engage your core throughout the movement. From the plank position, \
             slowly walk your hands back toward your feet, keeping your legs as straight as possible. \
             Return to the starting position by rolling up to standing one vertebra at a time. \
             Breathe naturally throughout the movement. For an easier version, bend your knees more. \
             For added difficulty, add a push-up when in the plank position or take smaller hand steps \
             to increase time under tension.",
    },
    Exercise {
        group: "Full Body",
        name: "Burpees",
        description: "Start standing with feet shoulder-width apart. Lower into a squat position \
             and place your hands on the ground in front of you. Kick your feet back to land in \
             a high plank position. Perform a push-up, keeping your body straight from head to heels. \
             Jump your feet back to your hands, landing in a squat position. Explosively jump up \
             from the squat, reaching your arms overhead. Land softly and immediately begin the next rep. \
             For a modified version, step back and forward instead of jumping, or skip the push-up. \
             For increased intensity, add a tuck jump at the top or perform the push-up with a clap.",
    },
    Exercise {
        group: "Mobility",
        name: "Squat to Toe Touch",
        description: "Start in a deep squat position with feet shoulder-width apart, heels flat on the ground, \
             and hands placed flat on the floor in front of you. Keep your chest up and back straight. \
             From this position, slowly straighten your legs as much as possible while keeping your hands \
             on the ground, feeling the stretch in your hamstrings and calves. Your body will form an inverted V shape. \
             Hold the stretched position for a moment, then bend your knees to return to the starting squat position. \
             Repeat the movement in a controlled manner. For increased difficulty, try to straighten your legs completely. \
             For reduced intensity, maintain a slight bend in the knees at the top position. \
             Focus on keeping your breathing steady throughout the exercise.",
    },
    Exercise {
        group: "Mobility",
        name: "Downward to Upward Dog",
        description: "Begin in a high plank position with wrists under shoulders and body forming a straight line. \
             Push your hips up and back while keeping arms and legs straight, forming an inverted V shape (Downward Dog). \
             Press heels toward the floor and relax your neck. Hold for a breath, then lower your hips toward the floor \
             while arching your back and pushing your chest forward (Upward Dog). Keep your shoulders away from your ears, \
             thighs lifted off the floor, and gaze forward. Flow between these two positions with controlled breathing. \
             For an easier version, perform Downward Dog with bent knees or substitute Cobra pose for Upward Dog by keeping \
             your hips on the ground. For increased difficulty, hold each position longer, add a plank between transitions, \
             or lift one leg during Downward Dog to challenge balance and engage your core more deeply.",
    },
    Exercise {
        group: "Mobility",
        name: "Lunge and Reach",
        description: "Start standing with feet hip-width apart. Step forward with one leg into a lunge position, \
             lowering your back knee toward the floor. Keep your front knee aligned with your ankle, not extending past your toes. \
             Once in the lunge, reach both arms overhead, extending through your spine and creating length. \
             Hold for a moment, feeling the stretch through your hip flexors, shoulders, and torso. \
             From the overhead position, slowly curl your torso down and place one palm on the floor beside your front foot for support. \
             Gently push your opposite elbow toward the floor, deepening the stretch through your shoulders and back. \
             Return to standing and repeat on the opposite side, alternating legs with each repetition. \
             Breathe deeply throughout the movement, exhaling as you reach and transition into the elbow stretch. \
             For an easier version, take a smaller step forward or don't lower as deeply into the lunge. \
             For increased difficulty, add a twist toward the front leg while in the lunge position, \
             or hold the position longer to increase the stretch and challenge your balance.",
    },
    Exercise {
        group: "Mobility",
        name: "Modified Butterfly Sit",
        description: "Sit on the floor with your back straight and bring the soles of your feet together in \
            front of you, allowing your knees to fall outward. Place your hands slightly behind your hips with \
            fingers pointing backward for support. Keeping your back straight, lift your hips slightly and slide \
            forward, creating a butterfly position with your feet together. Maintain your knees at a comfortable \
            height where you feel a gentle stretch in your inner thighs and hips. Breathe deeply and focus on \
            relaxing your hip muscles with each exhale. For increased comfort, start with a folded blanket or \
            cushion under your sitting bones. For a deeper stretch, gently press down on your inner thighs or \
            extend your torso forward while maintaining a straight back.",
    },
    Exercise {
        group: "Mobility",
        name: "90/90 Hip Stretch",
        description: "Sit on the floor with one leg bent in front of you at a 90-degree angle, \
             with your shin parallel to your hips. Position your other leg to the side, also bent \
             at 90 degrees with your shin parallel behind you. Keep your back straight and sit tall. \
             Your front foot should be flexed, and your back foot can be relaxed. Lean forward slightly \
             over your front leg while maintaining a straight back, feeling the stretch in your outer hip. \
             Hold this position for several breaths. To deepen the stretch, you can lean forward more or \
             gently press down on your front thigh. Switch sides by rotating your body and repositioning \
             your legs to stretch the opposite hip. For beginners, use your hands for support and don't \
             lean too far forward. For increased difficulty, try transitioning smoothly between sides or \
             hold the position longer.",
    }
];

#[component]
pub fn ExercisesPage() -> impl IntoView {
    let exercises_by_group = Memo::new(move |_| {
        EXERCISES
            .iter()
            .fold(BTreeMap::<&'static str, Vec<&Exercise>>::new(), |mut acc, exercise| {
                acc.entry(exercise.group).or_default().push(exercise);
                acc
            })
    });

    view! {
      <div class="container py-8 px-4 mx-auto">
        <h1 class="mb-6 text-2xl font-bold text-center">HIIT Exercises</h1>

        <div class="space-y-8">
          {move || {
            exercises_by_group
              .get()
              .iter()
              .map(|(group_name, exercises)| {
                view! {
                  <div class="p-6 mb-6 bg-white rounded shadow">
                    <h2 class="mb-4 text-xl font-bold">{*group_name}</h2>
                    <div class="space-y-4">
                      {exercises
                        .iter()
                        .map(|exercise| {
                          view! {
                            <div class="pb-4 border-b last:border-0">
                              <h3 class="text-lg font-semibold">{exercise.name}</h3>
                              <p class="mt-1 text-gray-600">{exercise.description}</p>
                            </div>
                          }
                        })
                        .collect::<Vec<_>>()}
                    </div>
                  </div>
                }
              })
              .collect::<Vec<_>>()
          }}
        </div>

        <div class="mt-8 text-center">
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
