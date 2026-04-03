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
