#![allow(dead_code)]

use std::num::NonZeroU32;

use crate::card_number::{CardNumber, SlotIndex};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct Binder {
    /// The number of rows on each page.
    rows: NonZeroU32,
    /// The number of columns on each page.
    cols: NonZeroU32,
    /// The number of pages in the binder.
    pages: NonZeroU32,
}

/// A struct to represent a card slot in the card binder.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct BinderSlot {
    /// The page number of the card slot.
    page: NonZeroU32,
    /// The row number on the page.
    row: NonZeroU32,
    /// The column number on the page.
    col: NonZeroU32,
    /// The index of the card slot.
    index: SlotIndex,
}

impl Binder {
    /// Create a new binder with the given number of rows and columns.
    pub fn new(rows: u32, cols: u32, pages: u32) -> Self {
        Self {
            rows: NonZeroU32::new(rows).expect("Rows should be non-zero"),
            cols: NonZeroU32::new(cols).expect("Columns should be non-zero"),
            pages: NonZeroU32::new(pages).expect("Pages should be non-zero"),
        }
    }

    /// Get the number of pages in the binder.
    pub fn pages(&self) -> u32 {
        self.pages.get()
    }

    /// Get the number of rows on a page.
    pub fn rows(&self) -> u32 {
        self.rows.get()
    }

    /// Get the number of columns on a page.
    pub fn cols(&self) -> u32 {
        self.cols.get()
    }

    /// Update the number of pages in the binder.
    pub fn set_pages(&mut self, pages: u32) -> Result<()> {
        self.pages = NonZeroU32::new(pages).context("Pages should be non-zero")?;
        Ok(())
    }

    /// Update the number of rows on a page.
    pub fn set_rows(&mut self, rows: u32) -> Result<()> {
        self.rows = NonZeroU32::new(rows).context("Rows should be non-zero")?;
        Ok(())
    }

    /// Update the number of columns on a page.
    pub fn set_cols(&mut self, cols: u32) -> Result<()> {
        self.cols = NonZeroU32::new(cols).context("Columns should be non-zero")?;
        Ok(())
    }

    /// Get the number of slots on a page.
    pub fn total_page_slots(&self) -> u32 {
        self.rows.saturating_mul(self.cols).get()
    }
}

impl BinderSlot {
    /// For a given index, return the corresponding card slot.
    pub fn from_index(binder: &Binder, index: SlotIndex) -> Self {
        let rows = binder.rows();
        let cols = binder.cols();

        let page = index.get() / (rows * cols);
        let row = (index.get() % (rows * cols)) / cols;
        let col = index.get() % cols;

        Self {
            page: NonZeroU32::new(page + 1).expect("Page number should be non-zero"),
            row: NonZeroU32::new(row + 1).expect("Row number should be non-zero"),
            col: NonZeroU32::new(col + 1).expect("Column number should be non-zero"),
            index,
        }
    }

    /// For a given card number, return the corresponding card slot.
    pub fn from_card_number(binder: &Binder, card_number: CardNumber) -> Self {
        let index = card_number.to_index();
        Self::from_index(binder, index)
    }

    /// Get the page number of the card slot.
    pub fn page(&self) -> u32 {
        self.page.get()
    }

    /// Get the row number of the card slot.
    pub fn row(&self) -> u32 {
        self.row.get()
    }

    /// Get the column number of the card slot.
    pub fn col(&self) -> u32 {
        self.col.get()
    }

    /// Convert a binder slot to an index.
    pub fn index(&self) -> SlotIndex {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binder_slot() {
        let binder = Binder::new(3, 3, 20);
        let slot = BinderSlot::from_index(&binder, SlotIndex::new(10));
        let index = slot.index();
        assert_eq!(index.get(), 11);
    }

    #[test]
    fn test_binder() {
        let binder = Binder::new(3, 3, 20);
        assert_eq!(binder.total_page_slots(), 9);
    }
}
