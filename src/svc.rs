use anyhow::{Context as _, Result};
use jiff_sqlx::ToSqlx as _;
use tracing::instrument;

use crate::{
    repo,
    types::{Context, LogInParams, MiscIn, SleepIn},
};

pub mod log {
    use super::*;

    #[instrument(err, skip(ctx))]
    pub async fn create(ctx: &Context, params: LogInParams) -> Result<()> {
        let time = params.time.timestamp().to_sqlx();
        let tz = params
            .time
            .time_zone()
            .iana_name()
            .context("invalid time format")?;

        let mut tx = ctx.db.begin().await?;

        repo::misc::create(
            tx.as_mut(),
            MiscIn {
                mood: params.mood,
                behaviours: params.behaviors,
                notes: params.notes,
                time,
                tz,
            },
        )
        .await?;
        if params.sleep_hours.is_some() || params.sleep_quality.is_some() {
            repo::sleep::create(
                tx.as_mut(),
                SleepIn {
                    sleep_hours: params.sleep_hours,
                    sleep_quality: params.sleep_quality,
                    source: Default::default(),
                    time,
                    tz,
                },
            )
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
