use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell {
    #[serde(rename = "Dauer")]
    spell_duration: String,
    #[serde(rename = "Optionen")]
    options: Vec<String>,
    #[serde(rename = "Reichweite")]
    reach: String,
    #[serde(rename = "Schwierigkeit")]
    difficulty: String,
    #[serde(rename = "Typus")]
    typus: String,
    #[serde(rename = "Verstärkt")]
    enforced: String,
    #[serde(rename = "Wirkung")]
    impact: String,
    #[serde(rename = "Wirkungsdauer")]
    cast_duration: String,
    #[serde(rename = "Zauberkosten")]
    cost: String,
    name: String,
    #[serde(rename = "schulen")]
    schools: Schools,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Schools {
    #[serde(rename = "Feuermagie")]
    feuermagie: Option<i64>,
    #[serde(rename = "Schutzmagie")]
    schutzmagie: Option<i64>,
    #[serde(rename = "Lichtmagie")]
    lichtmagie: Option<i64>,
    #[serde(rename = "Erkenntnismagie")]
    erkenntnismagie: Option<i64>,
    #[serde(rename = "Stärkungsmagie")]
    strkungsmagie: Option<i64>,
    #[serde(rename = "Verwandlungsmagie")]
    verwandlungsmagie: Option<i64>,
    #[serde(rename = "Bannmagie")]
    bannmagie: Option<i64>,
    #[serde(rename = "Illusionsmagie")]
    illusionsmagie: Option<i64>,
    #[serde(rename = "Bewegungsmagie")]
    bewegungsmagie: Option<i64>,
    #[serde(rename = "Windmagie")]
    windmagie: Option<i64>,
    #[serde(rename = "Heilungsmagie")]
    heilungsmagie: Option<i64>,
    #[serde(rename = "Todesmagie")]
    todesmagie: Option<i64>,
    #[serde(rename = "Schicksalsmagie")]
    schicksalsmagie: Option<i64>,
    #[serde(rename = "Naturmagie")]
    naturmagie: Option<i64>,
    #[serde(rename = "Beherrschungsmagie")]
    beherrschungsmagie: Option<i64>,
    #[serde(rename = "Kampfmagie")]
    kampfmagie: Option<i64>,
    #[serde(rename = "Wassermagie")]
    wassermagie: Option<i64>,
    #[serde(rename = "Schattenmagie")]
    schattenmagie: Option<i64>,
    #[serde(rename = "Felsmagie")]
    felsmagie: Option<i64>,
}
