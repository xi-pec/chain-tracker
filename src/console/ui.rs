use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

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
    chains: Mutex<HashMap<i64, ChainData>>
}

impl UI {
    pub fn init() -> Self {
        let chains = Mutex::new(HashMap::new());

        Self { chains }
    }

    pub fn set(&self, cards: HashMap<i64, (i64, String, Rarity, Type)>) {
        let Ok(mut chains) = self.chains.lock()
        else { return };

        chains.clear();

        for (id, (position, name, rarity, r#type)) in cards {
            chains.insert(id, ChainData {
                position,
                name,
                rarity,
                r#type,
                progress: Progress { current: 0, max: 0 }
            });
        }
    }

    pub fn update(&self, id: i64, current: i64, max: i64) {
        let Ok(mut chains) = self.chains.lock()
        else { return };

        chains.entry(id)
            .and_modify(|chain_data| {
                chain_data.progress.current = current;
                chain_data.progress.max = max;
            });
    }
}

impl Rarity {
    pub fn from(rarity: i64) -> Self {
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
        let reset = "\x1b[0m";

        match self {
            Self::R => {
                let gray = "\x1b[37m";
                write!(f, "{}R  {}", gray, reset)
            }

            Self::SR => {
                let gold = "\x1b[38;5;214m";
                write!(f, "{}SR {}", gold, reset)
            }

            Self::SSR => {
                let lavender = "\x1b[38;5;141m";
                write!(f, "{}SSR{}", lavender, reset)
            }

            _ => {
                let gray = "\x1b[90m";
                write!(f, "{}???{}", gray, reset)
            }
        }
    }
}

impl Type {
    pub fn from(r#type: i64) -> Self {
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
        let reset = "\x1b[0m";
        
        match self {
            Self::SPD => write!(f, "\x1b[94mSPD{}", reset),
            Self::STA => write!(f, "\x1b[38;5;203mSTA{}", reset),
            Self::POW => write!(f, "\x1b[38;5;208mPOW{}", reset),
            Self::GUT => write!(f, "\x1b[38;5;213mGUT{}", reset),
            Self::WIT => write!(f, "\x1b[38;5;120mWIT{}", reset),
            Self::FRD => write!(f, "\x1b[38;5;227mFRD{}", reset),
            Self::GRP => write!(f, "\x1b[38;5;121mGRP{}", reset),
            Self::UNKNOWN => write!(f, "\x1b[90m???{}", reset),
        }
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gray = "\x1b[90m";
    let reset = "\x1b[0m";

    let fill_color = if self.current >= self.max && self.max > 0 {
        "\x1b[92m" // green
    } else {
        "\x1b[94m" // blue
    };

    write!(f, "{}[", gray)?;

    let pattern = match self.max {
        5 => " >>>>> ",
        3 => " > > > ",
        2 => "  > >  ",
        1 => "   >   ",
        _ => "   ?   ",
    };

    let mut count = 0;
    for c in pattern.chars() {
        if c == '>' && count < self.current {
            write!(f, "{}{}", fill_color, c)?;
            count += 1;
        } else {
            write!(f, "{}{}", gray, c)?;
        }
    }

    write!(f, "]{}", reset)
    }
}

impl fmt::Display for UI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Ok(chains) = self.chains.lock()
        else { return Ok(()) };

        let mut values: Vec<&ChainData> = chains.values().collect();
        values.sort_by_key(|data| data.position);

        for data in values  {
            write!(f, "{} {} {} {}\n", data.progress, data.rarity, data.r#type, data.name)?;
        }

        Ok(())
    }
}