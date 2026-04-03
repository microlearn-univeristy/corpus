use serde::{Deserialize, Serialize};

// ── Threat / Node types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    Minimal  = 0,
    Low      = 1,
    Moderate = 2,
    High     = 3,
    Extreme  = 4,
}

impl ThreatLevel {
    pub fn label(self) -> &'static str {
        match self {
            ThreatLevel::Minimal  => "MINIMAL",
            ThreatLevel::Low      => "LOW",
            ThreatLevel::Moderate => "MODERATE",
            ThreatLevel::High     => "HIGH",
            ThreatLevel::Extreme  => "EXTREME",
        }
    }
    /// Base hull damage percentage per entry (0 = none).
    pub fn hull_damage_pct(self) -> f64 {
        match self {
            ThreatLevel::Minimal  => 0.0,
            ThreatLevel::Low      => 0.0,
            ThreatLevel::Moderate => 3.0,
            ThreatLevel::High     => 12.0,
            ThreatLevel::Extreme  => 30.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    HeartChamber,
    Artery,
    Vein,
    Organ,
}

impl NodeType {
    pub fn label(self) -> &'static str {
        match self {
            NodeType::HeartChamber => "Heart Chamber",
            NodeType::Artery       => "Artery",
            NodeType::Vein         => "Vein",
            NodeType::Organ        => "Organ",
        }
    }
}

// ── CircuitNode ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CircuitNode {
    pub id:            &'static str,
    pub name:          &'static str,
    pub node_type:     NodeType,
    /// One-line navigation flavor text shown when you arrive.
    pub description:   &'static str,
    /// Full educational biology fact shown on scan.
    pub biology:       &'static str,
    /// Adjacent node IDs (bidirectional).
    pub connections:   &'static [&'static str],
    pub threat_level:  ThreatLevel,
    /// ATP consumed to enter this node.
    pub atp_cost:      u32,
    /// Glucose units harvestable here (0 = none).
    pub glucose_yield: u32,
    /// Oxygen units harvestable here (0 = none).
    pub oxygen_yield:  u32,
    /// Legacy field — was used for Magellan circuit; campaigns now use active_objectives.
    #[allow(dead_code)]
    pub is_checkpoint: bool,
}

pub const START_NODE: &str = "left_ventricle";

// ── Lookup ────────────────────────────────────────────────────────────────────

pub fn find_node<'a>(circuit: &'a [CircuitNode], id: &str) -> Option<&'a CircuitNode> {
    circuit.iter().find(|n| n.id == id)
}

// ── The full cardiovascular circuit ──────────────────────────────────────────

pub fn build_circuit() -> Vec<CircuitNode> {
    vec![
        // ── Heart Chambers ─────────────────────────────────────────────────
        CircuitNode {
            id: "left_ventricle",
            name: "Left Ventricle",
            node_type: NodeType::HeartChamber,
            description: "The heart's most powerful chamber. Walls contract every 0.8 seconds \
                          with enough force to drive blood through the entire body. \
                          The rhythmic thud resonates through the hull.",
            biology: "The left ventricle generates pressures of 120 mmHg per contraction — the \
                      highest in the body. Its wall is 8–12 mm thick, three times that of the \
                      right ventricle. The mitral valve guards the inlet; the aortic valve the \
                      outlet. Each beat ejects roughly 70 mL of blood — about 5 liters per \
                      minute at rest. During exercise, this rises to 25 liters. The muscular \
                      wall (myocardium) is composed of cardiomyocytes that contract as a unit \
                      via electrical gap junctions.",
            connections: &["ascending_aorta", "left_atrium"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 3,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "left_atrium",
            name: "Left Atrium",
            node_type: NodeType::HeartChamber,
            description: "A thin-walled receiving chamber. Oxygenated blood arrives here \
                          from the lungs, funneled into the left ventricle below.",
            biology: "The left atrium receives oxygenated blood from four pulmonary veins — two \
                      from each lung. Its wall is only 3 mm thick: it collects rather than pumps. \
                      The pulmonary veins are anatomically anomalous: veins carrying oxygenated \
                      blood. The left atrium is prone to fibrillation — disorganized electrical \
                      activity that causes irregular heartbeat and stroke risk from clot formation \
                      in the left atrial appendage.",
            connections: &["left_ventricle", "pulmonary_vein"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "right_atrium",
            name: "Right Atrium",
            node_type: NodeType::HeartChamber,
            description: "The convergence point for deoxygenated blood from the entire body. \
                          Three tributaries drain here. The sinoatrial node — \
                          the heart's pacemaker — is embedded in this wall.",
            biology: "The right atrium receives blood from the superior vena cava (upper body), \
                      inferior vena cava (lower body), and coronary sinus (cardiac muscle). \
                      The sinoatrial (SA) node — a cluster of specialized pacemaker cells in \
                      the right atrial wall — fires electrical impulses spontaneously at 60–100 \
                      bpm, coordinating every heartbeat. Dysfunction here causes sick sinus \
                      syndrome and bradycardia.",
            connections: &["right_ventricle", "superior_vena_cava", "inferior_vena_cava", "coronary_sinus"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "right_ventricle",
            name: "Right Ventricle",
            node_type: NodeType::HeartChamber,
            description: "The right-side pump — less muscular, generating lower pressure \
                          for the short journey to the lungs.",
            biology: "The right ventricle produces only 25 mmHg of pressure — one-fifth that of \
                      the left. The pulmonary circuit is short and low-resistance, requiring far \
                      less force. The tricuspid valve (three leaflets) guards its inlet; the \
                      pulmonary valve its outlet. Chronic pulmonary hypertension strains the \
                      right ventricle, causing right heart failure (cor pulmonale) — the right \
                      side was not built for sustained high-pressure work.",
            connections: &["right_atrium", "pulmonary_artery"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 3,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Pulmonary Circuit ──────────────────────────────────────────────
        CircuitNode {
            id: "pulmonary_artery",
            name: "Pulmonary Artery",
            node_type: NodeType::Artery,
            description: "The only artery in the body carrying deoxygenated blood — \
                          dark, slower, striking. Two great branches diverge toward \
                          each lung.",
            biology: "The pulmonary arteries are anatomically arteries (thick, muscular walls) \
                      but carry deoxygenated blood — the inverse of every other artery. Normal \
                      pulmonary arterial pressure is 25/10 mmHg, far below systemic levels. \
                      Pulmonary embolism — a blood clot lodged here — obstructs flow and can \
                      cause sudden cardiac collapse. It kills 100,000 Americans annually.",
            connections: &["right_ventricle", "lungs"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "lungs",
            name: "Pulmonary Alveoli",
            node_type: NodeType::Organ,
            description: "Translucent honeycomb sacs extending in every direction. \
                          Oxygen readings are saturating your sensors. \
                          This is the refueling zone — harvest here.",
            biology: "Each lung contains 300–500 million alveoli — microscopic air sacs \
                      0.2 mm in diameter with walls a single cell layer thick (0.2 micrometers). \
                      Total gas-exchange surface: 70 square meters — the size of a tennis court. \
                      Oxygen dissolves into plasma and binds hemoglobin in 0.25 seconds. \
                      Type II pneumocytes lining the alveoli produce surfactant — a phospholipid \
                      film that reduces surface tension and prevents collapse. Without it, every \
                      breath would require enormous effort. Premature infants lack surfactant: \
                      respiratory distress syndrome.",
            connections: &["pulmonary_artery", "pulmonary_vein"],
            threat_level: ThreatLevel::Low,
            atp_cost: 4,
            glucose_yield: 0,
            oxygen_yield: 25,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "pulmonary_vein",
            name: "Pulmonary Vein",
            node_type: NodeType::Vein,
            description: "Bright arterial red — this vein carries the most oxygenated \
                          blood in the body back to the left heart.",
            biology: "Four pulmonary veins return oxygenated blood to the left atrium. \
                      They are the only veins (along with the umbilical vein in fetuses) that \
                      carry oxygenated blood. They have no valves — left atrial low pressure \
                      and continuous lung perfusion create unimpeded flow. Blood transits the \
                      pulmonary capillaries in 0.75 seconds at rest — barely enough for full \
                      O2 loading during maximal exercise.",
            connections: &["lungs", "left_atrium"],
            threat_level: ThreatLevel::Minimal,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 5,
            is_checkpoint: false,
        },

        // ── Systemic Arteries ──────────────────────────────────────────────
        CircuitNode {
            id: "ascending_aorta",
            name: "Ascending Aorta",
            node_type: NodeType::Artery,
            description: "The primary highway departs the heart. Thick elastic walls pulse \
                          visibly with each beat. Current is powerful — 1.5 m/s at peak.",
            biology: "The ascending aorta is 2.5 cm in diameter, rising 5 cm before arching. \
                      Its wall contains abundant elastin — a protein that stretches under systolic \
                      pressure and recoils in diastole. This Windkessel effect converts pulsatile \
                      pump output into near-continuous flow in peripheral vessels. Aortic \
                      aneurysm — pathological dilation — risks catastrophic rupture; a dissection \
                      (tear in the wall) carries 1–2% mortality per hour untreated.",
            connections: &["left_ventricle", "aortic_arch", "coronary_artery", "thymus"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "aortic_arch",
            name: "Aortic Arch",
            node_type: NodeType::Artery,
            description: "The aorta bends sharply over the heart. Three vessels branch \
                          upward toward the head and arms. The main trunk continues \
                          downward through the chest.",
            biology: "The aortic arch gives rise to three vessels: the brachiocephalic trunk \
                      (splitting into right subclavian and right common carotid), the left common \
                      carotid, and the left subclavian artery. Baroreceptors embedded in the arch \
                      continuously monitor blood pressure, firing signals to the brainstem's \
                      cardiovascular center to modulate heart rate and vascular resistance — \
                      a reflex loop completing in under one second.",
            connections: &["ascending_aorta", "carotid_artery", "descending_aorta"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "descending_aorta",
            name: "Descending Aorta",
            node_type: NodeType::Artery,
            description: "The main trunk descends through chest and abdomen, \
                          branching to every organ below the heart. \
                          A four-lane highway of oxygenated blood.",
            biology: "The descending aorta passes through the aortic hiatus in the diaphragm \
                      (becoming the abdominal aorta at T12) and travels along the vertebral \
                      column. It gives off intercostal arteries, the celiac trunk, superior and \
                      inferior mesenteric arteries, renal arteries, and gonadal arteries before \
                      bifurcating into the common iliac arteries at L4. The bifurcation is a \
                      prime site for atherosclerotic plaque.",
            connections: &["aortic_arch", "renal_artery", "mesenteric_artery", "iliac_artery", "spleen"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Coronary Circulation ───────────────────────────────────────────
        CircuitNode {
            id: "coronary_artery",
            name: "Coronary Arteries",
            node_type: NodeType::Artery,
            description: "Two arteries coil around the heart like a crown (corona), \
                          supplying the muscle that keeps everything moving.",
            biology: "The left and right coronary arteries arise from the sinuses of Valsalva — \
                      pockets behind the aortic valve cusps — filled only during diastole when \
                      backflow briefly holds the cusps open. The coronary system delivers 5% of \
                      cardiac output to an organ that never rests. Atherosclerosis here — \
                      plaque narrowing the lumen — is the most common cause of death globally. \
                      A complete blockage starves the downstream muscle in minutes: \
                      myocardial infarction.",
            connections: &["ascending_aorta", "heart_muscle"],
            threat_level: ThreatLevel::Low,
            atp_cost: 3,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "heart_muscle",
            name: "Myocardium",
            node_type: NodeType::Organ,
            description: "The walls of the heart itself. Tightly packed muscle fibers \
                          contract in perfect synchrony around you — every surface vibrates. \
                          HELIX: 'Cardiac macrophages detected. Elevated surveillance.'",
            biology: "Cardiac muscle (myocardium) is involuntary and striated. Unlike skeletal \
                      muscle, cardiomyocytes are coupled via gap junctions (intercalated discs), \
                      causing the whole heart to contract as an electrical syncytium. \
                      They cannot regenerate after infarction — replaced by scar tissue, which \
                      doesn't contract. Cardiomyocytes have the highest mitochondrial density of \
                      any cell type: 25–35% of cell volume. They extract 70% of delivered oxygen — \
                      more than any other tissue — leaving almost no reserve during ischemia.",
            connections: &["coronary_artery", "coronary_sinus"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 5,
            glucose_yield: 5,
            oxygen_yield: 0,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "coronary_sinus",
            name: "Coronary Sinus",
            node_type: NodeType::Vein,
            description: "A short venous collector on the posterior surface of the heart, \
                          draining spent blood from the myocardium back to the right atrium.",
            biology: "The coronary sinus collects from the great, middle, and small cardiac veins \
                      before opening into the right atrium between the inferior vena cava and the \
                      tricuspid valve. Because the myocardium extracts ~70% of delivered oxygen \
                      (vs. ~25% for most tissues), the blood here is profoundly deoxygenated — \
                      oxygen saturation around 25%. There is almost no reserve; any reduction in \
                      coronary flow rapidly starves the heart of oxygen.",
            connections: &["heart_muscle", "right_atrium"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Cerebral Circulation ───────────────────────────────────────────
        CircuitNode {
            id: "carotid_artery",
            name: "Internal Carotid Artery",
            node_type: NodeType::Artery,
            description: "The primary arterial supply to the brain. Strong pulse. \
                          HELIX is flagging elevated threat ahead: blood-brain barrier \
                          in range.",
            biology: "The internal carotid arteries supply the forebrain and eyes. The carotid \
                      body — a chemoreceptor cluster at the bifurcation of the common carotid — \
                      detects blood O2, CO2, and pH, relaying signals to the brainstem's \
                      respiratory center. Carotid sinus baroreceptors adjust heart rate within \
                      seconds of pressure changes. Carotid artery stenosis (narrowing by plaque) \
                      is a leading cause of ischemic stroke.",
            connections: &["aortic_arch", "brain"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "brain",
            name: "Cerebral Cortex",
            node_type: NodeType::Organ,
            description: "BLOOD-BRAIN BARRIER BREACH. Dense microglia activity on all \
                          sensors. Astrocyte end-feet press against vessel walls. \
                          Extraordinary neural density — 100 trillion connections — \
                          in every direction. This is the most dangerous location \
                          in the body.",
            biology: "The blood-brain barrier is formed by brain capillary endothelial cells \
                      connected by tight junctions, supported by pericytes and astrocyte end-feet. \
                      It blocks all but the smallest lipid-soluble molecules — drugs must be \
                      specially engineered to cross it. The brain constitutes 2% of body mass \
                      but consumes 20% of oxygen and glucose. It houses 86 billion neurons and \
                      roughly equal numbers of glial cells, with ~100 trillion synaptic \
                      connections. It cannot tolerate more than 4–6 minutes without oxygen \
                      before irreversible damage begins.",
            connections: &["carotid_artery", "jugular_vein"],
            threat_level: ThreatLevel::Extreme,
            atp_cost: 15,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "jugular_vein",
            name: "Internal Jugular Vein",
            node_type: NodeType::Vein,
            description: "Large venous trunk descending from the skull. The cranial \
                          pressure differential is palpable as neural tissue recedes.",
            biology: "The internal jugular veins are the primary venous drainage of the brain, \
                      receiving blood from the sigmoid sinus and other dural sinuses. Unlike most \
                      systemic veins, they receive mildly pulsatile flow transmitted via \
                      cerebrospinal fluid (CSF) pressure changes with each heartbeat. Physicians \
                      assess jugular venous distension (JVD) at the bedside as a measure of \
                      right heart filling pressure — a window into cardiovascular status \
                      without any equipment.",
            connections: &["brain", "superior_vena_cava", "lymph_nodes"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Hepatic / Digestive ────────────────────────────────────────────
        CircuitNode {
            id: "mesenteric_artery",
            name: "Superior Mesenteric Artery",
            node_type: NodeType::Artery,
            description: "Branches like a river delta into the intestinal walls. \
                          A chemically complex scent permeates — digestion at scale.",
            biology: "The superior mesenteric artery (SMA) supplies the small intestine, cecum, \
                      ascending colon, and part of the transverse colon. It receives 10% of \
                      cardiac output at rest — rising to 30% postprandially. SMA syndrome — \
                      compression of the duodenum between the SMA and aorta — causes obstruction. \
                      Mesenteric ischemia (sudden blockage) can necrotize the intestine within \
                      hours: a surgical catastrophe with 60–80% mortality.",
            connections: &["descending_aorta", "small_intestine"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "small_intestine",
            name: "Intestinal Villi",
            node_type: NodeType::Organ,
            description: "Finger-like projections carpeting every surface — each tipped \
                          with a brush of microvilli. The plasma is thick with glucose. \
                          Prime harvest zone.",
            biology: "The small intestine maximizes absorptive area via three amplifications: \
                      circular folds (plicae circulares), finger-like villi (0.5–1.6 mm tall), \
                      and microvilli on each enterocyte (the 'brush border'). Combined surface \
                      area: 250 square meters — the size of a doubles tennis court. Glucose is \
                      absorbed by SGLT1 sodium-glucose cotransporters (active transport, against \
                      gradient) and enters capillaries directly. After a carbohydrate meal, \
                      portal vein glucose rises 3-fold within 30 minutes. Lacteal vessels (lymph) \
                      absorb fats alongside the blood capillaries.",
            connections: &["mesenteric_artery", "portal_vein"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 5,
            glucose_yield: 20,
            oxygen_yield: 0,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "portal_vein",
            name: "Hepatic Portal Vein",
            node_type: NodeType::Vein,
            description: "A vein that starts in capillaries and ends in capillaries — \
                          the only portal system in the body. Rich with absorbed nutrients \
                          bound for the liver.",
            biology: "The hepatic portal system is unique: venous blood from the intestines \
                      passes through a second capillary bed (hepatic sinusoids) before reaching \
                      the heart. This delivers nutrient-rich blood directly to the liver for \
                      immediate processing. Portal hypertension — most often caused by cirrhosis \
                      (scarring of the liver) — backs pressure into the portal system, dilating \
                      esophageal varices that can rupture catastrophically.",
            connections: &["small_intestine", "liver", "spleen"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 5,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "liver",
            name: "Hepatic Sinusoids",
            node_type: NodeType::Organ,
            description: "HELIX: 'Multiple phagocytic contacts. Kupffer cells active — \
                          they're treating the vessel as a foreign particle.' \
                          Sinusoid walls are fenestrated (pored). You're being assessed.",
            biology: "Hepatic sinusoids are specialized capillaries with large fenestrations \
                      (gaps) and resident macrophages — Kupffer cells — that continuously \
                      phagocytose foreign particles and cellular debris. The liver's ~100 billion \
                      hepatocytes perform over 500 functions: detoxifying ammonia (to urea), \
                      synthesizing albumin and clotting factors, producing bile for fat digestion, \
                      storing glycogen, performing gluconeogenesis, and metabolizing drugs. \
                      It receives 25% of cardiac output via both the portal vein and \
                      hepatic artery.",
            connections: &["portal_vein", "hepatic_vein"],
            threat_level: ThreatLevel::High,
            atp_cost: 8,
            glucose_yield: 10,
            oxygen_yield: 0,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "hepatic_vein",
            name: "Hepatic Veins",
            node_type: NodeType::Vein,
            description: "Draining processed, detoxified blood from the liver into \
                          the inferior vena cava just below the diaphragm.",
            biology: "Three hepatic veins (right, middle, left) drain the liver into the \
                      inferior vena cava immediately below the diaphragm. Their short course \
                      means hepatic vein thrombosis — Budd-Chiari syndrome — rapidly causes \
                      hepatic congestion and failure. Blood leaving the liver has been filtered \
                      of toxins, enriched with plasma proteins (albumin, fibrinogen), and \
                      regulated for glucose content. First-pass metabolism here \
                      dramatically reduces bioavailability of many oral drugs.",
            connections: &["liver", "inferior_vena_cava"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Renal Circulation ──────────────────────────────────────────────
        CircuitNode {
            id: "renal_artery",
            name: "Renal Artery",
            node_type: NodeType::Artery,
            description: "Short, wide. The kidneys demand 20% of cardiac output — \
                          extraordinary for two fist-sized organs.",
            biology: "The renal arteries receive 1.2 liters of blood per minute — 20–25% of \
                      cardiac output — an enormous fraction for their size. Juxtaglomerular cells \
                      in the renal artery wall monitor pressure: a drop triggers renin secretion, \
                      initiating the renin-angiotensin-aldosterone cascade that raises systemic \
                      blood pressure. Renal artery stenosis (usually atherosclerotic) causes \
                      renovascular hypertension — a treatable but often-missed cause of \
                      high blood pressure.",
            connections: &["descending_aorta", "kidneys"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "kidneys",
            name: "Renal Glomeruli",
            node_type: NodeType::Organ,
            description: "Tiny knots of capillaries under pressure. Fluid is forced \
                          through the walls around you. HELIX: 'Filtration membrane \
                          detected. We are technically small enough to be filtered. \
                          Recommend maintaining hull integrity.'",
            biology: "Each kidney contains ~1 million nephrons — filtration units consisting of \
                      a glomerulus (a capillary tuft at 60 mmHg pressure) and Bowman's capsule. \
                      The kidneys filter 180 liters of blood per day — the entire blood volume \
                      40 times — producing 1–2 liters of urine. Podocytes, cells with \
                      interdigitating foot processes, form the filtration slit membrane. The \
                      kidneys also produce erythropoietin (stimulating red blood cell production), \
                      activate vitamin D (calcium metabolism), and regulate acid-base balance via \
                      bicarbonate reabsorption in the tubules.",
            connections: &["renal_artery", "renal_vein"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 6,
            glucose_yield: 3,
            oxygen_yield: 0,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "renal_vein",
            name: "Renal Veins",
            node_type: NodeType::Vein,
            description: "Clean, filtered blood. The kidneys have done their work. \
                          Waste products have been removed; plasma composition normalized.",
            biology: "The renal veins return filtered blood to the inferior vena cava. The left \
                      renal vein is three times longer than the right, crossing the aorta \
                      anteriorly — a quirk of embryological development. This makes the left \
                      kidney easier to harvest for transplant (longer vascular pedicle). \
                      The left renal vein can be compressed between the aorta and superior \
                      mesenteric artery (nutcracker syndrome), causing flank pain and \
                      hematuria.",
            connections: &["kidneys", "inferior_vena_cava"],
            threat_level: ThreatLevel::Minimal,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Peripheral / Musculoskeletal ───────────────────────────────────
        CircuitNode {
            id: "iliac_artery",
            name: "Common Iliac Artery",
            node_type: NodeType::Artery,
            description: "The aorta bifurcates at waist level into two great trunks \
                          supplying the pelvis and legs. The flow splits here.",
            biology: "The abdominal aorta bifurcates at L4 into the common iliac arteries — \
                      a location you can feel pulsating through the abdominal wall in lean \
                      individuals. This bifurcation is a prime site for atherosclerosis. \
                      Leriche syndrome — aortoiliac occlusive disease — causes bilateral \
                      buttock claudication (pain on walking), absent femoral pulses, and \
                      impotence in males from absent pelvic flow.",
            connections: &["descending_aorta", "skeletal_muscle", "bone_marrow"],
            threat_level: ThreatLevel::Low,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "skeletal_muscle",
            name: "Skeletal Muscle Capillaries",
            node_type: NodeType::Organ,
            description: "A dense mesh threading between red and white muscle fibers. \
                          The vessel is still. You can feel the host's resting metabolic \
                          rhythm — slow, steady, deep.",
            biology: "Skeletal muscle constitutes 40% of body mass and 20–25% of resting O2 \
                      consumption. No muscle fiber lies more than 50 micrometers from a capillary. \
                      Myoglobin — a monomeric hemoglobin analog storing O2 in muscle fibers — \
                      gives meat its red color. During maximal exercise, muscle blood flow \
                      increases 20-fold (from ~1 to 20 L/min) via vasodilatation and capillary \
                      recruitment. Type I (slow-twitch, oxidative) fibers are fatigue-resistant; \
                      Type II (fast-twitch, glycolytic) generate power but tire quickly.",
            connections: &["iliac_artery", "iliac_vein"],
            threat_level: ThreatLevel::Low,
            atp_cost: 4,
            glucose_yield: 5,
            oxygen_yield: 5,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "bone_marrow",
            name: "Red Bone Marrow",
            node_type: NodeType::Organ,
            description: "Crowded, dark red. Every direction packed with cells at all \
                          stages of development. HELIX: 'Immune progenitor density is \
                          extreme. These cells are being born here. Do not linger.'",
            biology: "Red bone marrow fills the cavities of flat bones (sternum, pelvis, ribs) \
                      and vertebrae. Hematopoietic stem cells here produce 200 billion red blood \
                      cells per day (2 million per second), plus all white blood cells and \
                      platelets — a process called hematopoiesis. Sinusoids (wide-gap capillaries) \
                      allow newly formed cells to squeeze into the bloodstream. Bone marrow is \
                      regulated by erythropoietin, thrombopoietin, and G-CSF. Failure causes \
                      aplastic anemia; cancerous transformation causes leukemia.",
            connections: &["iliac_artery", "iliac_vein"],
            threat_level: ThreatLevel::High,
            atp_cost: 10,
            glucose_yield: 8,
            oxygen_yield: 0,
            is_checkpoint: true,
        },
        CircuitNode {
            id: "iliac_vein",
            name: "Common Iliac Vein",
            node_type: NodeType::Vein,
            description: "Converging from the legs and pelvis. You can feel the \
                          relative quiet of venous return — slower, lower pressure.",
            biology: "The left common iliac vein crosses beneath the right common iliac artery \
                      before joining the IVC. This natural compression can cause May-Thurner \
                      syndrome — chronic left iliac vein narrowing predisposing to deep vein \
                      thrombosis (DVT). DVT, if dislodged, travels as a pulmonary embolism. \
                      The iliac veins are the primary venous drainage of the lower extremities \
                      and pelvis.",
            connections: &["skeletal_muscle", "bone_marrow", "inferior_vena_cava"],
            threat_level: ThreatLevel::Minimal,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Venous Return ──────────────────────────────────────────────────
        CircuitNode {
            id: "superior_vena_cava",
            name: "Superior Vena Cava",
            node_type: NodeType::Vein,
            description: "The great upper trunk. Continuous steady flow from the head, \
                          neck, and arms converging toward the right heart.",
            biology: "The SVC is 7 cm long and 2 cm in diameter, formed by union of the two \
                      brachiocephalic veins. It carries about one-third of total venous return. \
                      Unlike arteries, veins have thin walls and rely on external mechanisms: \
                      skeletal muscle contraction (muscle pump), respiratory pressure changes \
                      (respiratory pump), and venous valves to assist blood return against \
                      gravity. SVC syndrome — obstruction by tumor or clot — causes facial \
                      swelling and venous distension of the upper body.",
            connections: &["jugular_vein", "right_atrium", "thymus", "lymph_nodes"],
            threat_level: ThreatLevel::Minimal,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "inferior_vena_cava",
            name: "Inferior Vena Cava",
            node_type: NodeType::Vein,
            description: "The largest vein in the body. Deoxygenated blood from the \
                          entire lower half converges here. The right heart is close.",
            biology: "The IVC is the largest vein in the body, collecting blood from hepatic, \
                      renal, gonadal, and iliac veins below the diaphragm. In pregnancy, the \
                      gravid uterus compresses the IVC when lying flat — reducing venous return, \
                      dropping cardiac output, causing supine hypotension syndrome. This is why \
                      pregnant people are advised to sleep on their left side. IVC filters can \
                      be placed percutaneously to catch pulmonary emboli from DVTs.",
            connections: &["hepatic_vein", "renal_vein", "iliac_vein", "right_atrium"],
            threat_level: ThreatLevel::Minimal,
            atp_cost: 2,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },

        // ── Immune / Lymphoid ─────────────────────────────────────────────
        CircuitNode {
            id: "thymus",
            name: "Thymus",
            node_type: NodeType::Organ,
            description: "A bilobed organ in the anterior mediastinum. In health it is small, \
                          largely replaced by fat. In this host it is enlarged — dense with \
                          lymphoblastic infiltration on every sensor. T-cell precursors in \
                          every stage of development, and abnormal blast cells crowding them out.",
            biology: "The thymus reaches maximum relative size at birth and involutes after puberty, \
                      but remains critical for naive T-cell export throughout life. T-lymphocyte \
                      precursors migrate here from bone marrow; positive and negative selection \
                      eliminate 95% of candidates — only T cells that recognize self-MHC without \
                      attacking self-antigens survive. In T-cell ALL (T-ALL), malignant \
                      transformation of thymic precursors creates a mediastinal mass of \
                      lymphoblasts — often presenting in adolescent males with superior vena cava \
                      syndrome from compression. CAR-T cells targeting CD7 or CD5 are in clinical \
                      evaluation specifically for T-ALL. Even in B-ALL, the thymic microenvironment \
                      influences the behavior of co-injected therapeutic T cells.",
            connections: &["ascending_aorta", "superior_vena_cava"],
            threat_level: ThreatLevel::Low,
            atp_cost: 4,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "spleen",
            name: "Splenic Sinusoids",
            node_type: NodeType::Organ,
            description: "Soft, dark red, and enlarged beyond its normal 150 grams. \
                          Leukemic blast cells have infiltrated the white pulp. \
                          The sinusoids are dense with abnormal lymphocytes. \
                          HELIX: 'Splenomegaly confirmed. Macrophage activity elevated. \
                          CAR-T cells detected in the parenchyma.'",
            biology: "The spleen is the largest secondary lymphoid organ. Its red pulp filters \
                      300 mL of blood per minute, removing senescent erythrocytes via macrophage \
                      phagocytosis in splenic cords. Its white pulp — periarteriolar lymphoid \
                      sheaths and germinal centers — houses B and T cells for immune surveillance. \
                      In leukemia, the spleen becomes a site of extramedullary hematopoiesis \
                      (blood cell production outside the marrow) and leukemic blast infiltration. \
                      Massive splenomegaly is a hallmark of CML and advanced ALL. \
                      CAR-T cells trafficking through the spleen encounter large antigen depots — \
                      the spleen is a key site of early CAR-T expansion and activation. \
                      Splenic macrophages also clear exhausted blast cells post-therapy.",
            connections: &["descending_aorta", "portal_vein"],
            threat_level: ThreatLevel::Moderate,
            atp_cost: 6,
            glucose_yield: 4,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
        CircuitNode {
            id: "lymph_nodes",
            name: "Cervical Lymph Nodes",
            node_type: NodeType::Organ,
            description: "Clusters of bean-shaped nodes along the jugular chain. \
                          Normally 1 cm; now enlarged three to four times. \
                          Lymph flowing through carries circulating blast cells — \
                          the nodes are attempting filtration but are overwhelmed.",
            biology: "Lymph nodes are secondary lymphoid organs that filter lymph fluid before \
                      it rejoins the bloodstream. Each contains a cortex (B-cell follicles with \
                      germinal centers), paracortex (T-cell zone), and medulla (macrophages, \
                      plasma cells). In leukemia, lymphadenopathy is a hallmark — nodes enlarge \
                      as leukemic blasts infiltrate and proliferate. The cervical, axillary, and \
                      inguinal chains are most commonly palpated clinically. Lymph nodes also \
                      present tumor antigens to T cells — a step that CAR-T therapy circumvents \
                      by engineering T cells with artificial antigen receptors (CARs) targeting \
                      leukemia-specific surface markers: CD19 for B-ALL, CD7 for T-ALL. \
                      Residual disease in lymph nodes after induction chemotherapy (MRD-positive \
                      nodes) is a major predictor of relapse and a key target for CAR-T \
                      consolidation therapy.",
            connections: &["jugular_vein", "superior_vena_cava"],
            threat_level: ThreatLevel::Low,
            atp_cost: 4,
            glucose_yield: 0,
            oxygen_yield: 0,
            is_checkpoint: false,
        },
    ]
}
