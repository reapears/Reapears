use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Price of the harvest
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Price {
    pub amount: Decimal,
    pub unit: Unit,
}

impl Default for Price {
    fn default() -> Self {
        Self {
            amount: 0.into(),
            unit: Unit::Kg(0),
        }
    }
}

impl Price {
    /// Creates a new price
    #[must_use]
    pub const fn new(amount: Decimal, unit: Unit) -> Self {
        Self { amount, unit }
    }

    /// Creates a new `Price` from the database column
    #[must_use]
    pub fn from_row(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap_or_default()
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let amount = self.amount;
        let unit = self.unit.clone();
        write!(f, "N${amount} {unit}")
    }
}

/// A unit of the `Price`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Unit {
    Kg(u8),
    Crate,
    Bundle,
    Head,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Crate => write!(f, "crate"),
            Self::Bundle => write!(f, "bundle"),
            Self::Head => write!(f, "head"),
            Self::Kg(n) => write!(f, "{n}kg"),
        }
    }
}
