use chrono::{DateTime, TimeZone, Utc};
use once_cell::sync::Lazy;

// Most recent time of first rotation as of writing.
#[rustfmt::skip]
const EPOCH: Lazy<DateTime<Utc>> =
    Lazy::new(|| Utc.with_ymd_and_hms(2025, 6, 9, 0, 0, 0).unwrap());

// Most recent time of first rotation for the Steel Path as of writing.
const SP_EPOCH: Lazy<DateTime<Utc>> =
    Lazy::new(|| Utc.with_ymd_and_hms(2025, 7, 21, 0, 0, 0).unwrap());

#[rustfmt::skip]
const REWARDS: [[&str; 3]; 11] = [
    ["Excalibur", "Trinity", "Ember"   ],
    ["Loki",      "Mag",     "Rhino"   ],
    ["Ash",       "Frost",   "Nyx"     ],
    ["Saryn",     "Vauban",  "Nova"    ],
    ["Nekros",    "Valkyr",  "Oberon"  ],
    ["Hydroid",   "Mirage",  "Limbo"   ],
    ["Mesa",      "Chroma",  "Atlas"   ],
    ["Ivara",     "Inaros",  "Titania" ],
    ["Nidus",     "Octavia", "Harrow"  ],
    ["Gara",      "Khora",   "Revenant"],
    ["Garuda",    "Baruuk",  "Hildryn" ],
];

#[rustfmt::skip]
const SP_REWARDS: [[&str; 5]; 8] = [
    ["Braton",      "Lato",          "Skana",      "Paris",     "Kunai"         ],
    ["Boar",        "Gammacor",      "Angstrum",   "Gorgon",    "Anku"          ],
    ["Bo",          "Latron",        "Furis",      "Furax",     "Strun"         ],
    ["Lex",         "Magistar",      "Boltor",     "Bronco",    "Ceramic Dagger"],
    ["Torid",       "Dual Toxocyst", "Dual Ichor", "Miter",     "Atomos"        ],
    ["Ack & Brunt", "Soma",          "Vasto",      "Nami Solo", "Burston"       ],
    ["Zylok",       "Sibear",        "Dread",      "Despair",   "Hate"          ],
    ["Dera",        "Sybaris",       "Cestra",     "Sicarus",   "Okina"         ],
];

/// Get the Steel Path offerings from the Circuit.
pub fn sp_circuit() -> [&'static str; 5] {
    let now = Utc::now();
    let week = (now - *SP_EPOCH).num_weeks() % 8;

    SP_REWARDS[week as usize]
}

/// Get the offerings from the Circuit.
pub fn circuit() -> [&'static str; 3] {
    let now = Utc::now();
    let week = (now - *EPOCH).num_weeks() % 11;

    REWARDS[week as usize]
}
