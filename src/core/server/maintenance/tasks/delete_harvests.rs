

pub async fn archive_farm_harvests(
    farm_id: ModelID,
    finished_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<u64> {
    let max_age = harvest_max_age(finished_at)?;
    match sqlx::query!(
        r#"
            UPDATE services.harvests harvest
            SET finished = true,
                images = NULL,
                finished_at = $1

            WHERE harvest.location_id IN (
                SELECT location_.id
                FROM services.active_locations location_
                WHERE location_.farm_id = $2
            )
            AND NOT(
                harvest.available_at > $1 OR
                harvest.created_at > $3
            )
        "#,
        finished_at.date(),
        farm_id.0,
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Farm active harvests archived, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive farm harvests");
            Err(err.into())
        }
    }
}
