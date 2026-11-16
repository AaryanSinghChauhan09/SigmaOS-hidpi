// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-hidpi: hidpi component model.
//!
//! Real implementation: hidpi_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidpiItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HidpiError {
    EmptyHidpiItemName,
    DuplicateHidpiItem(String),
    EmptyEventSubject,
}

impl fmt::Display for HidpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHidpiItemName => write!(f, "hidpi_item name must not be empty"),
            Self::DuplicateHidpiItem(id) => write!(f, "duplicate hidpi_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for HidpiError {}

pub struct Hidpi {
    pub name: String,
    hidpi_items: BTreeMap<String, HidpiItem>,
    events: BTreeMap<String, Event>,
}

impl Hidpi {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hidpi_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_hidpi_item(&mut self, item: HidpiItem) -> Result<(), HidpiError> {
        if item.name.trim().is_empty() {
            return Err(HidpiError::EmptyHidpiItemName);
        }
        let id = item.id.clone();
        if self.hidpi_items.contains_key(&id) {
            return Err(HidpiError::DuplicateHidpiItem(id.clone()));
        }
        self.hidpi_items.insert(id, item);
        Ok(())
    }

    pub fn hidpi_item_count(&self) -> usize {
        self.hidpi_items.len()
    }

    pub fn get_hidpi_item(&self, id: &str) -> Option<&HidpiItem> {
        self.hidpi_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), HidpiError> {
        if event.subject.trim().is_empty() {
            return Err(HidpiError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hidpi_item(id: &str, name: &str) -> HidpiItem {
        HidpiItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Hidpi::new("Hidpi");
        assert_eq!(m.name, "Hidpi");
        assert_eq!(m.hidpi_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_hidpi_item() {
        let mut m = Hidpi::new("Hidpi");
        m.add_hidpi_item(sample_hidpi_item("a1", "Alpha")).unwrap();
        assert_eq!(m.hidpi_item_count(), 1);
        assert!(m.get_hidpi_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Hidpi::new("Hidpi");
        assert!(m.add_hidpi_item(sample_hidpi_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Hidpi::new("Hidpi");
        m.add_hidpi_item(sample_hidpi_item("x", "One")).unwrap();
        assert!(m.add_hidpi_item(sample_hidpi_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Hidpi::new("Hidpi");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
