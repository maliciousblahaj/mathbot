use mathbot::{command::CommandParams, Result};
use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub async fn slots(params: CommandParams) -> Result<()> {

    Ok(())
}

struct Slots {
    row_1: [SlotItem;5],
    row_2: [SlotItem;5],
    row_3: [SlotItem;5],
}

impl Slots {
    pub fn new() -> Self {
        let items = SlotItem::iter().collect::<Vec<_>>();
        let mut rng = thread_rng();
        let rows = Vec::new();
        for _ in 0..3 {
            let mut row = [SlotItem::Malako; 5];
            for i in 0..5 {
                row[i] = items.choose(&mut rng).unwrap().clone();
            }
        }

        Self {
            row_1: rows[0],
            row_2: rows[1],
            row_3: rows[2],
        }
    }
}

#[derive(strum_macros::EnumIter, Clone, Copy)]
enum SlotItem {
    Planet,
    Mathtopia,
    Ferris,
    Victor,
    Malako
}

impl SlotItem {
    pub fn emoji_str(&self) -> &str {
        match self {
            Self::Planet => "<:slot1:1260985506020200548>",
            Self::Mathtopia => "<:slot2:1260985504472629419>",
            Self::Ferris => "<:slot3:1260985501704388798>",
            Self::Victor => "<:slot4:1260985503038312549>",
            Self::Malako => "<:slot5:1260985500483850300>",
        }
    }

    pub fn multiplier(&self) -> usize {
        match self {
            Self::Planet => 100,
            Self::Mathtopia => 25,
            Self::Ferris => 10,
            Self::Victor => 5,
            Self::Malako => 2,
        }
    }
}