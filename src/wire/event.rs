use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::error::ErrorPayload;

pub const TYPE_ERROR: &str = "error";
pub const TYPE_LOG: &str = "log";
pub const TYPE_USAGE: &str = "usage";
pub const DATA_KEY_ERROR: &str = "@error";
pub const DATA_KEY_LEVEL: &str = "@level";
pub const DATA_KEY_USER: &str = "@user";
pub const DATA_KEY_VERSION: &str = "@version";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub date: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub data: Map<String, Value>,
}

impl Event {
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            source: None,
            date: Utc::now(),
            tags: Vec::new(),
            message: None,
            data: Map::new(),
        }
    }

    pub fn error(error: ErrorPayload) -> Self {
        let mut event = Self::new(TYPE_ERROR);
        event.data.insert(
            DATA_KEY_ERROR.to_owned(),
            serde_json::to_value(error).unwrap_or(Value::Null),
        );
        event
    }

    pub fn log(message: impl Into<String>) -> Self {
        let mut event = Self::new(TYPE_LOG);
        event.message = Some(message.into());
        event
    }

    pub fn feature_usage(feature_name: impl Into<String>) -> Self {
        let mut event = Self::new(TYPE_USAGE);
        event.source = Some(feature_name.into());
        event
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        let tag = tag.into();
        if !tag.trim().is_empty() && !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, value: Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }

    pub fn with_level(mut self, level: impl AsRef<str>) -> Self {
        let level = level.as_ref().trim();
        if !level.is_empty() {
            self.data
                .insert(DATA_KEY_LEVEL.to_owned(), Value::String(level.to_owned()));
        }
        self
    }

    pub fn with_user_identity(mut self, identity: impl Into<String>) -> Self {
        self.data
            .insert(DATA_KEY_USER.to_owned(), Value::String(identity.into()));
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.data
            .insert(DATA_KEY_VERSION.to_owned(), Value::String(version.into()));
        self
    }
}
