use itertools::Itertools;
use serenity::all::MessageBuilder;
use warframe::worldstate::items::Item;
use warframe::worldstate::queryable::VoidTrader;
use warframe::worldstate::{TimedEvent, VoidTraderInventoryItem};

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
        let tables = format_tables(&trader.inventory).await;

        // ...warning about truncation if necessary
        if tables.len() > max_tables {
            time_info = format!(
                "{time_info}\nNote: more than {max_tables} tables \
                    of content found. Truncating to reduce spam..."
            );
        }

        // Create our output buffer, and start pushing our tables into it
        let mut msgs = vec![time_info];
        for table in tables[0..max_tables].iter() {
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
async fn format_tables(items: &[VoidTraderInventoryItem]) -> Vec<String> {
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
