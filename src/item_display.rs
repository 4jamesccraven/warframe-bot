use itertools::Itertools;
use poise::serenity_prelude::MessageBuilder;
use warframe::worldstate::items::Item;
use warframe::worldstate::queryable::VoidTrader;
use warframe::worldstate::{TimedEvent, VoidTraderInventoryItem};

use crate::circuit::{circuit, sp_circuit};

pub async fn calculate_baro_string(trader: &VoidTrader) -> Vec<String> {
    let max_tables = 2;
    if trader.active() {
        // Generate overview message
        let mut time_info = format!(
            "Baro Ki'Teer is at {} until {}.",
            trader.location,
            trader.expiry().format("%a, %b %d at %-I:%M %p")
        );

        // Calculate the tables...
        let tables = format_baro_inventory(&trader.inventory).await;

        // ...warning about truncation if necessary
        if tables.len() > max_tables {
            time_info = format!(
                "{time_info}\nNote: more than {max_tables} tables \
                    of content found. Truncating to reduce spam..."
            );
        }

        // Create our output buffer, and start pushing our tables into it
        let mut msgs = vec![time_info];
        for table in tables[0..tables.len().min(max_tables)].iter() {
            let msg = MessageBuilder::new()
                // Sanitise our output and make it a codeblock.
                // It would probably be fine not to sanitise it, but why not?
                .push_codeblock_safe(table, None)
                .build();

            msgs.push(msg);
        }

        msgs
    } else {
        // Without inventory to share, a simple timer will suffice.
        let msg = format!(
            "Baro Ki'Teer will be at {} on {}.",
            trader.location,
            trader.activation().format("%a, %b %d"),
        );

        vec![msg]
    }
}

/// Sorts and formats the the Void Trader's inventory for display.
async fn format_baro_inventory(items: &[VoidTraderInventoryItem]) -> Vec<String> {
    // Sort the items and format them as strings for displaying
    let data = items
        .iter()
        .sorted_by(|item_a, item_b| {
            item_b
                .ducats
                .cmp(&item_a.ducats)
                .then_with(|| item_a.item.inner().cmp(item_b.item.inner()))
                .then_with(|| item_b.credits.cmp(&item_a.credits))
        })
        .map(|item| {
            vec![
                item.item.inner().to_string(),
                item.ducats.to_string(),
                format_thousands(item.credits),
            ]
        })
        .collect::<Vec<_>>();

    // Chunk the items to fit the approximate message size limit, and convert them into tables to
    // be displayed by the bot.
    data.into_iter()
        .chunks(25)
        .into_iter()
        .map(|chunk| {
            use ascii_table::{Align::*, AsciiTable};

            let d_i = chunk.into_iter().collect::<Vec<_>>();

            let mut table = AsciiTable::default();
            table.column(0).set_header("Item").set_align(Left);
            table.column(1).set_header("Ducats").set_align(Right);
            table.column(2).set_header("Credits").set_align(Right);

            table.format(d_i)
        })
        .collect::<Vec<_>>()
}

/// Generate the name of the archon shard from the boss's name.
pub fn format_archon(boss: &str) -> String {
    match boss.to_lowercase() {
        s if s.contains("amar") => "Crimson Archon Shard".to_string(),
        s if s.contains("nira") => "Amber Archon Shard".to_string(),
        s if s.contains("boreal") => "Azure Archon Shard".to_string(),
        s => format!("Unknown Archon Boss: {s}"),
    }
}

pub struct WeeklyInfo {
    pub archon_shard: String,
    pub normal_circuit: [&'static str; 3],
    pub sp_circuit: [&'static str; 5],
}

impl WeeklyInfo {
    pub fn new(boss: &str) -> Self {
        Self {
            archon_shard: format_archon(boss),
            normal_circuit: circuit(),
            sp_circuit: sp_circuit(),
        }
    }

    /// Convert the weekly info struct into a Discord message.
    pub fn as_message(&self) -> String {
        use ascii_table::{Align::*, AsciiTable};
        let mut table = AsciiTable::default();
        table.column(0).set_header("The Circit").set_align(Center);
        #[rustfmt::skip]
        table.column(1).set_header("The Circit (Steel Path)").set_align(Center);
        table.column(2).set_header("Archon Hunt").set_align(Center);

        let circ = self.normal_circuit;
        let spc = self.sp_circuit;

        let data = vec![
            vec![circ[0], spc[0], &self.archon_shard],
            vec![circ[1], spc[1]],
            vec![circ[2], spc[2]],
            vec!["", spc[3]],
            vec!["", spc[4]],
        ];

        let table = table.format(data);

        let line_length = table
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        let title = format!("{:^width$}", "Weekly Info", width = line_length);

        MessageBuilder::new()
            .push_codeblock_safe(format!("{}\n{}", title, table), None)
            .build()
    }
}

/// Formats large numbers using k (thousands) and M (millions) where appropriate.
///
/// Primarily used to display credits.
fn format_thousands(n: i32) -> String {
    match n {
        0..=999 => n.to_string(),
        1_000..=9_949 => format!("{:.1}k", n as f64 / 1_000.0),
        9_950..=999_499 => format!("{:.0}k", n as f64 / 1_000.0),
        _ => format!("{:.1}M", n as f64 / 1_000_000.0),
    }
}

// TODO: Everything below this point will hopefully be re-incorporated later, but there is currently no
// way to use them efficiently.

#[allow(unused)]
fn variant_name(e: &Item) -> &'static str {
    use Item::*;
    match e {
        Arcane(_) => "arcane",
        Archwing(_) => "archwing",
        Fish(_) => "fish",
        Gear(_) => "gear",
        Glyph(_) => "glyph",
        Misc(_) => "misc",
        Mod(_) => "mod",
        Node(_) => "node",
        Pet(_) => "pet",
        Quest(_) => "quest",
        Relic(_) => "relic",
        Resource(_) => "resource",
        Sentinel(_) => "sentinel",
        Sigil(_) => "sigil",
        Skin(_) => "skin",
        Warframe(_) => "warframe",
        Weapon(_) => "weapon",
    }
}

#[allow(unused)]
fn variant_group(e: &Item) -> usize {
    use Item::*;
    match e {
        Arcane(_) | Mod(_) | Quest(_) | Relic(_) | Weapon(_) | Sentinel(_) | Archwing(_)
        | Gear(_) => 0,
        Glyph(_) | Resource(_) | Sigil(_) | Warframe(_) | Pet(_) | Skin(_) => 1,
        Fish(_) | Node(_) | Misc(_) => 3,
    }
}
