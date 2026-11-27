use sqlx::{Row, postgres::PgRow};

use crate::domain::models::entity::{Location, LocationKind};

pub(crate) fn decode(row: &PgRow) -> Result<Location, sqlx::Error> {
    let kind: String = row.try_get("kind")?;

    Ok(Location {
        id: row.try_get("id")?,
        kind: kind
            .parse::<LocationKind>()
            .map_err(|_| sqlx::Error::Decode("unknown location kind".into()))?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        location_lat: row.try_get("location_lat")?,
        location_lng: row.try_get("location_lng")?,
        is_active: row.try_get("is_active")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        updated_by: row.try_get("updated_by")?,
    })
}
