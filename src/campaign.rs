use crate::body::host::HostProfile;

#[derive(Clone)]
pub struct Campaign {
    pub id:           &'static str,
    pub name:         &'static str,
    pub tagline:      &'static str,
    pub description:  &'static str,
    pub objectives:   &'static [&'static str],
    pub win_node:     &'static str,
    pub host_profile: Option<HostProfile>,
    pub intro:        &'static [&'static str],
    pub win_text:     &'static [&'static str],
}

pub fn all_campaigns() -> Vec<Campaign> {
    vec![
        Campaign {
            id: "magellan",
            name: "The Magellan Circuit",
            tagline: "Circumnavigate the entire cardiovascular system.",
            description: "Complete one full circuit of the body — visiting all eight major organ \
                          systems and returning to the Left Ventricle. The first full voyage \
                          of its kind.",
            objectives: &[
                "heart_muscle", "brain", "lungs", "liver",
                "small_intestine", "kidneys", "bone_marrow", "skeletal_muscle",
            ],
            win_node: "left_ventricle",
            host_profile: None,
            intro: &[
                "Pilot. This is HELIX. Miniaturization confirmed. Injection complete.",
                "Objective: complete one full circuit of the cardiovascular system. Visit all \
                 eight major organ systems. Return to the Left Ventricle.",
                "Ferdinand Magellan never completed his own circumnavigation. \
                 Dr. Yun and Theo are online. Good luck.",
            ],
            win_text: &[
                "You have completed the circuit.",
                "You have navigated the entire cardiovascular system of a living human being. \
                 Heart. Lungs. Brain. Liver. Intestines. Kidneys. Marrow. Muscle. \
                 And back to where it started.",
                "Ferdinand Magellan never completed his own circumnavigation. You did.",
            ],
        },
        Campaign {
            id: "chimera",
            name: "Operation Chimera",
            tagline: "Leukemia. Verify CAR-T cell delivery at four disease sites.",
            description: "The host carries acute lymphoblastic leukemia. Leukemic blast cells \
                          have crowded out healthy marrow. CAR-T cells — engineered T cells \
                          targeting CD19-positive blasts — have been co-injected into the \
                          bloodstream. Navigate to the four primary disease sites, confirm \
                          therapeutic infiltration at each, and return to the left ventricle \
                          before the verification window closes.",
            objectives: &["thymus", "lymph_nodes", "spleen", "bone_marrow"],
            win_node: "left_ventricle",
            host_profile: Some(HostProfile::Leukemic),
            intro: &[
                "Pilot. This is HELIX. Injection confirmed. You are inside a leukemic host.",
                "Diagnosis: acute lymphoblastic leukemia. Blast cell count elevated throughout \
                 the bloodstream and marrow. CAR-T cells — CD19-targeted chimeric antigen \
                 receptor T cells — have been co-injected and are already dispersing.",
                "Your mission: reach the four primary disease sites — thymus, lymph nodes, \
                 spleen, and bone marrow — confirm CAR-T infiltration at each, then return \
                 to the left ventricle to complete the therapeutic verification protocol.",
                "Dr. Yun and Theo are online. They are not treating this as routine.",
            ],
            win_text: &[
                "Verification protocol complete.",
                "CAR-T infiltration confirmed at all four sites. Blast populations engaged. \
                 The immune reconstitution sequence has begun.",
                "What happens next is not instant. The CAR-T cells will expand, hunt, and kill. \
                 The marrow will slowly repopulate with healthy progenitors. The host will not \
                 know what happened here today — but something shifted inside them.",
            ],
        },
        Campaign {
            id: "sepsis",
            name: "Sepsis Protocol",
            tagline: "Systemic infection. Assess the cascade before multi-organ failure.",
            description: "Bacterial sepsis. Systemic inflammatory response syndrome is building. \
                          Cytokines are flooding the bloodstream and the immune system is beginning \
                          to attack the host's own tissues. Navigate to the three primary immune \
                          coordination centers — spleen, liver, and bone marrow — assess the \
                          inflammatory state, and return before the cascade becomes irreversible.",
            objectives: &["spleen", "liver", "bone_marrow"],
            win_node: "left_ventricle",
            host_profile: Some(HostProfile::Infected),
            intro: &[
                "Pilot. This is HELIX. Injection confirmed. Sepsis protocol is active.",
                "Blood cultures positive. Gram-negative bacteremia. Systemic inflammatory \
                 response is escalating — IL-6, TNF-alpha, and IL-1beta are all elevated. \
                 The immune system has activated and is beginning to lose precision.",
                "Objective: reach the spleen, liver, and bone marrow. Assess the cascade. \
                 Report back to the left ventricle. If the cytokine storm is not interrupted, \
                 multi-organ failure follows.",
                "Dr. Yun is already working the problem. Theo is quieter than usual.",
            ],
            win_text: &[
                "Assessment complete. Data transmitted.",
                "The inflammatory state at all three sites has been logged. \
                 The care team has what they need to calibrate the treatment protocol.",
                "Sepsis kills one in five. The host's odds just improved.",
            ],
        },
        Campaign {
            id: "peak_output",
            name: "Peak Output",
            tagline: "The host is mid-run. Confirm the aerobic cascade is holding.",
            description: "The host entered sustained aerobic exercise eleven minutes ago. \
                          Heart rate is 155 bpm and climbing. Skeletal muscle oxygen demand \
                          has tripled. The adrenal medulla is flooding the bloodstream with \
                          epinephrine. Navigate to the four sites driving the exercise response — \
                          adrenal glands, heart muscle, lungs, and skeletal muscle — confirm the \
                          aerobic cascade is intact, and return to the left ventricle.",
            objectives: &["adrenal_glands", "heart_muscle", "lungs", "skeletal_muscle"],
            win_node: "left_ventricle",
            host_profile: Some(HostProfile::Exercising),
            intro: &[
                "Pilot. This is HELIX. Injection confirmed. The host is in motion.",
                "Current telemetry: heart rate 155 bpm, systolic 168, respiratory rate elevated. \
                 Epinephrine detected in bloodstream. Blood flow has redistributed — \
                 gut perfusion down, skeletal muscle perfusion up threefold.",
                "Objective: verify the aerobic cascade at four primary sites. \
                 Adrenal glands, heart muscle, lungs, skeletal muscle. Then return. \
                 Note: glucose is scarce — muscle is consuming it faster than the liver \
                 can release it. Oxygen is abundant. Plan accordingly.",
                "Dr. Yun finds this one straightforward. Theo is excited about the mitochondria.",
            ],
            win_text: &[
                "Aerobic cascade confirmed. All four sites nominal.",
                "The adrenals fired. The heart adapted. The lungs delivered. \
                 The muscle burned clean. Every system performed exactly as designed.",
                "The host doesn't know you were here. They just know the run felt good.",
            ],
        },
        Campaign {
            id: "cortisol_storm",
            name: "Cortisol Storm",
            tagline: "Acute psychological stress. The HPA axis has taken over.",
            description: "The host is experiencing acute psychosocial stress — sustained, \
                          uncontrollable, and ongoing. The hypothalamic-pituitary-adrenal axis \
                          is fully activated. Cortisol is elevated and climbing. Epinephrine \
                          is surging. Blood glucose is high. Immune surveillance is suppressed. \
                          Navigate the stress circuit — brain, adrenal glands, heart muscle — \
                          map the hormonal cascade, and return to the left ventricle before \
                          the sustained cortisol load begins causing damage.",
            objectives: &["brain", "adrenal_glands", "heart_muscle"],
            win_node: "left_ventricle",
            host_profile: Some(HostProfile::Stressed),
            intro: &[
                "Pilot. This is HELIX. Injection confirmed. Biochemical environment is abnormal.",
                "Cortisol: elevated. Epinephrine: elevated. Blood glucose: high. \
                 Heart rate 94 bpm. Systolic 148. The HPA axis is running a full activation — \
                 hypothalamus signaled CRH, pituitary released ACTH, adrenals are responding.",
                "Objective: reach the brain, the adrenal glands, and the heart muscle. \
                 Map the cascade at each site. Then return. \
                 The stress response is not inherently pathological — but this one has been \
                 running for a while. Duration matters.",
                "Dr. Yun is not comfortable with how long this has been active. \
                 Theo is trying to stay focused.",
            ],
            win_text: &[
                "Stress circuit mapped. Data logged.",
                "The brain initiated it. The adrenals executed it. The heart bore the cost. \
                 The cascade is textbook — but sustained cortisol at this level suppresses \
                 immune function, degrades hippocampal neurons, and accelerates arterial aging.",
                "The host is not in danger today. But if this continues, they will be. \
                 That is not your problem to solve. It is theirs.",
            ],
        },
        Campaign {
            id: "coronary",
            name: "Coronary Crisis",
            tagline: "Myocardial infarction in progress. Time is muscle.",
            description: "A ruptured atherosclerotic plaque has formed a thrombus in the coronary \
                          arteries, blocking oxygen delivery to the myocardium. Cardiac muscle is \
                          dying. Navigate to the occlusion site, then confirm the state of the \
                          myocardium. Every minute of ischemia is irreversible.",
            objectives: &["coronary_artery", "heart_muscle"],
            win_node: "left_ventricle",
            host_profile: Some(HostProfile::Hypertensive),
            intro: &[
                "Pilot. This is HELIX. Emergency injection confirmed. Coronary protocol active.",
                "Troponin rising. ST-elevation on the ECG feed. A plaque in the left anterior \
                 descending artery has ruptured — thrombus forming, lumen occluding. \
                 The anterior wall of the left ventricle is ischemic.",
                "Reach the coronary arteries. Confirm the occlusion. Then reach the myocardium \
                 and assess ischemic damage. Every 60 seconds of delay equals more necrosis.",
                "Dr. Yun is not calm. Theo is not excited. Move.",
            ],
            win_text: &[
                "Coronary assessment complete.",
                "Occlusion site confirmed. Myocardial damage mapped. \
                 The interventional team has a target and a damage estimate.",
                "The stent goes in next. What you found here — the extent of the affected \
                 territory, the collateral flow status — changes the treatment decision. \
                 Some of that muscle will be saved because you were here.",
            ],
        },
    ]
}
