use std::collections::HashMap;
use std::fmt;

pub enum Rarity {
    R,
    SR,
    SSR,
    UNKNOWN
}

pub enum Type {
    SPD,
    STA,
    POW,
    GUT,
    WIT,
    FRD,
    GRP,
    UNKNOWN
}

pub struct Progress {
    current: i64,
    max: i64
}

pub struct ChainData {
    name: String,
    position: i64,
    rarity: Rarity,
    r#type: Type,
    progress: Progress
}

pub struct UI {
    chains: HashMap<i64, ChainData>
}

impl UI {
    pub fn init() -> Self {
        let chains = HashMap::new();

        Self { chains }
    }

    pub fn set(&mut self, cards: HashMap<i64, (i64, String, Rarity, Type)>) {
        self.chains.clear();

        for (id, (position, name, rarity, r#type)) in cards {
            self.chains.insert(id, ChainData {
                position,
                name,
                rarity,
                r#type,
                progress:Progress { current: 0, max: 0 }
            });
        }
    }

    pub fn update(&mut self, id: i64, current: i64, max: i64) {
        self.chains.entry(id)
            .and_modify(|chain_data| {
                chain_data.progress.current = current;
                chain_data.progress.max = max;
            });
    }
}

impl Rarity {
    pub fn from(rarity: i32) -> Self {
        match rarity {
            1 => Self::R,
            2 => Self::SR,
            3 => Self::SSR,
            _ => Self::UNKNOWN
        }
    }
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rarity = match self {
            Self::R => "R  ",
            Self::SR => "SR ",
            Self::SSR => "SSR",
            _ => "???"
        };

        write!(f, "{}", rarity)
    }
}

impl Type {
    pub fn from(r#type: i32) -> Self {
        match r#type {
            1 => Self::SPD,
            2 => Self::STA,
            3 => Self::POW,
            4 => Self::GUT,
            5 => Self::WIT,
            6 => Self::FRD,
            7 => Self::GRP,
            _ => Self::UNKNOWN
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r#type = match self {
            Self::SPD => "SPD",
            Self::STA => "STA",
            Self::POW => "POW",
            Self::GUT => "GUT",
            Self::WIT => "WIT",
            Self::FRD => "FRD",
            Self::GRP => "GRP",
            Self::UNKNOWN => "???"
        };

        write!(f, "{}", r#type)
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let blue = "\x1b[94m";
        let gray = "\x1b[37m";
        let reset = "\x1b[0m";

        // Start with gray brackets
        write!(f, "{}[", gray)?;

        // Get the base string pattern
        let pattern = match self.max {
            5 => ">>>>>",
            3 => "> > >",
            2 => " > > ",
            1 => "  >  ",
            _ => "  ?  ",
        };

        let mut blue_count = 0;
        for c in pattern.chars() {
            if c == '>' && blue_count < self.current {
                write!(f, "{}{}", blue, c)?;
                blue_count += 1;
            } else {
                write!(f, "{}{}", gray, c)?;
            }
        }

        write!(f, "]{}", reset)
    }
}

impl fmt::Display for UI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut values: Vec<&ChainData> = self.chains.values().collect();
        values.sort_by_key(|data| data.position);

        for data in values  {
            write!(f, "{} {} {} {}\n", data.progress, data.rarity, data.r#type, data.name)?;
        }

        Ok(())
    }
}