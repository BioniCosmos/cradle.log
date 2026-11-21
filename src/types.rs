use jiff::ZonedDateTime;
use jiff_sqlx::{Date, Timestamp};
use serde::Deserialize;
use sqlx::{PgPool, Type};
use uuid::Uuid;

pub struct Context {
    pub db: PgPool,
}

pub struct MiscIn<'a> {
    pub mood: i16,
    pub behaviours: Vec<String>,
    pub notes: String,
    pub time: Timestamp,
    pub tz: &'a str,
}

pub struct MiscOut {
    pub id: Uuid,
    pub mood: i16,
    pub behaviours: Vec<String>,
    pub notes: String,
    pub time: Timestamp,
    pub tz: String,
    pub date: Date,
}

pub struct SleepIn<'a> {
    pub sleep_hours: Option<f32>,
    pub sleep_quality: Option<i16>,
    pub source: SleepSource,
    pub time: Timestamp,
    pub tz: &'a str,
}

pub struct SleepOut {
    pub id: Uuid,
    pub sleep_hours: Option<f32>,
    pub sleep_quality: Option<i16>,
    pub source: SleepSource,
    pub time: Timestamp,
    pub tz: String,
    pub date: Date,
}

#[derive(Default, Type)]
#[sqlx(type_name = "text", rename_all = "kebab-case")]
pub enum SleepSource {
    #[default]
    Manual,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogInParams {
    pub time: ZonedDateTime,
    pub mood: i16,
    pub sleep_hours: Option<f32>,
    pub sleep_quality: Option<i16>,
    pub behaviors: Vec<String>,
    pub disease: Option<String>,
    pub symptoms: Option<String>,
    pub pain: Option<i16>,
    pub medication: Option<String>,
    pub notes: String,
}
