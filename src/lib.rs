use mackerel_plugin::*;
use std::collections::HashMap;

pub struct DicePlugin {}

impl Plugin for DicePlugin {
    fn fetch_metrics(&self) -> Result<HashMap<String, f64>, String> {
        Ok(HashMap::from([
            ("dice.d6".to_owned(), (rand::random::<u64>() % 6 + 1) as f64),
            (
                "dice.d20".to_owned(),
                (rand::random::<u64>() % 20 + 1) as f64,
            ),
        ]))
    }

    fn graph_definition(&self) -> Vec<Graph> {
        vec![graph! {
            name: "dice",
            label: "My Dice",
            unit: "integer",
            metrics: [
                { name: "d6", label: "Die 6" },
                { name: "d20", label: "Die 20" },
            ],
        }]
    }
}
