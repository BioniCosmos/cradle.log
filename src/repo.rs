use anyhow::Result;
use jiff_sqlx::Timestamp;
use sqlx::{PgExecutor, query};

use crate::types::{MiscIn, SleepIn, SleepSource};

pub mod misc {
    use super::*;

    pub async fn create(db: impl PgExecutor<'_>, x: MiscIn<'_>) -> Result<()> {
        query!(
            "INSERT INTO misc (mood, behaviours, notes, time, tz) VALUES ($1, $2, $3, $4, $5)",
            x.mood,
            &x.behaviours,
            x.notes,
            x.time as Timestamp,
            x.tz,
        )
        .execute(db)
        .await?;
        Ok(())
    }
}

pub mod sleep {
    use super::*;

    pub async fn create(db: impl PgExecutor<'_>, x: SleepIn<'_>) -> Result<()> {
        query!(
            "INSERT INTO sleep (sleep_hours, sleep_quality, source, time, tz)
                VALUES ($1, $2, $3, $4, $5)",
            x.sleep_hours,
            x.sleep_quality,
            x.source as SleepSource,
            x.time as Timestamp,
            x.tz,
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
