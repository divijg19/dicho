use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::daily;
use crate::storage::Store;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Daily,
    Save,
    Unsave,
}

impl EventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Save => "save",
            Self::Unsave => "unsave",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub time: String,
    pub kind: EventKind,
    pub id: String,
}

fn path() -> Result<PathBuf> {
    Ok(Store::data_dir()?.join("history.jsonl"))
}

fn read_all() -> Result<Vec<Event>> {
    let path = path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text =
        fs::read_to_string(&path).with_context(|| format!("cannot read {}", path.display()))?;
    let mut events = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let event: Event = serde_json::from_str(line)
            .with_context(|| format!("invalid history event on line {}", index + 1))?;
        events.push(event);
    }
    Ok(events)
}

pub fn append(kind: EventKind, id: &str) -> Result<()> {
    let path = path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let event = Event {
        time: daily::timestamp()?,
        kind,
        id: id.to_owned(),
    };
    let line = serde_json::to_string(&event)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("cannot open {}", path.display()))?;
    writeln!(file, "{line}")?;
    Ok(())
}

/// Record a daily encounter once per UTC date.
pub fn record_daily_if_new(date: &str, id: &str) -> Result<bool> {
    let events = read_all()?;
    if events
        .iter()
        .any(|event| event.kind == EventKind::Daily && event.time.starts_with(date))
    {
        return Ok(false);
    }
    append(EventKind::Daily, id)?;
    Ok(true)
}

pub fn recent(limit: usize) -> Result<Vec<Event>> {
    let events = read_all()?;
    let start = events.len().saturating_sub(limit);
    Ok(events[start..].to_vec())
}

pub fn lines(events: &[Event]) -> Vec<String> {
    events
        .iter()
        .map(|event| format!("{}  {}  {}", event.time, event.kind.as_str(), event.id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_kind_names_are_stable() {
        assert_eq!(EventKind::Daily.as_str(), "daily");
        assert_eq!(EventKind::Save.as_str(), "save");
        assert_eq!(EventKind::Unsave.as_str(), "unsave");
    }

    #[test]
    fn lines_render_events_compactly() {
        let events = vec![Event {
            time: "2026-11-26T09:30:00Z".to_owned(),
            kind: EventKind::Daily,
            id: "refran-001".to_owned(),
        }];
        assert_eq!(
            lines(&events),
            vec!["2026-11-26T09:30:00Z  daily  refran-001"]
        );
    }
}
