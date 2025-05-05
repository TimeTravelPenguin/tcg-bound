use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

/// A user-facing card number: always >=1 and <= capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CardNumber(NonZeroU32);

impl CardNumber {
    /// Try to create a new CardNumber in the range [1..=max].
    pub fn try_new(n: u32, max: u32) -> Option<Self> {
        NonZeroU32::new(n)
            .filter(|&nz| nz.get() <= max)
            .map(CardNumber)
    }

    /// Extract the raw 1-based value.
    pub fn get(self) -> u32 {
        self.0.get()
    }

    /// Convert to a 0-based slot index.
    pub fn to_index(self) -> SlotIndex {
        // subtract 1 is guaranteed safe
        SlotIndex(self.0.get() - 1)
    }
}

/// An internal, 0-based slot index: always >=0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SlotIndex(u32);

impl SlotIndex {
    /// Create a new SlotIndex from a 0-based value.
    pub fn new(n: u32) -> Self {
        SlotIndex(n)
    }

    /// Convert back to a 1-based CardNumber (for display).
    pub fn to_card_number(self) -> CardNumber {
        // unwrap is safe because slot < capacity => +1 <= capacity
        CardNumber(NonZeroU32::new(self.0 + 1).unwrap())
    }

    /// Extract the raw 0-based value.
    pub fn get(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_number() {
        assert_eq!(CardNumber::try_new(1, 10).unwrap().get(), 1);
        assert_eq!(CardNumber::try_new(10, 10).unwrap().get(), 10);
        assert_eq!(CardNumber::try_new(11, 10), None);
        assert_eq!(CardNumber::try_new(0, 10), None);
    }

    #[test]
    fn test_slot_index() {
        assert_eq!(SlotIndex(0).to_card_number().get(), 1);
        assert_eq!(SlotIndex(9).to_card_number().get(), 10);
    }

    #[test]
    fn test_zero_based_conversion() {
        let max = 10;
        for n in 1..=max {
            let cn = CardNumber::try_new(n, max).unwrap();
            assert_eq!(cn.to_index().get(), n - 1);
        }
    }

    #[test]
    fn test_round_trip() {
        let max = 10;
        for n in 1..=max {
            let cn = CardNumber::try_new(n, max).unwrap();
            let idx = cn.to_index();
            assert_eq!(idx.to_card_number().get(), n);
        }
    }
}
