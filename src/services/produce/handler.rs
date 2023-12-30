//! Harvest feed impls

use std::future;

use axum::{extract::State, Json};
use axum_extra::extract::Query;
use futures_util::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
// use time::OffsetDateTime;

use crate::{
    endpoint::{validators::TransformString, EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::harvest::models::{Harvest, HarvestList};

/// Handles the `GET /harvests/feed` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_feed(
    filters: Query<HarvestFilter>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<HarvestFeed>> {
    let cultivars = filters.cultivars();
    let regions = filters.regions();
    let skip_id = filters.offset_id();

    let mut harvests: Vec<_> = Harvest::stream(&db)
        .await
        // Offset
        .try_skip_while(|harvest| {
            // Skips harvests until the skip_id.
            // if skip_id is None all harvests will be returned until the limit.
            // if skip_id is not a valid harvest_id no harvest will be returned.
            future::ready(skip_id.map_or_else(|| Ok(false), |id| Ok(harvest.id != id)))
        })
        // Filters
        .try_filter(|harvest| {
            // Filters harvests, if no filters are available
            // the all harvests will pass the filter, otherwise
            // only those matched the filters will pass.
            future::ready(match (cultivars.is_empty(), regions.is_empty()) {
                (true, true) => true,
                (false, false) => {
                    cultivars.contains(&harvest.name) && regions.contains(&harvest.region)
                }
                (false, true) => cultivars.contains(&harvest.name),
                (true, false) => regions.contains(&harvest.region),
            })
        })
        // Limit
        .take(filters.limit + 1)
        .try_collect()
        .await
        .map_err(|_err| EndpointRejection::internal_server_error())?;

    // Get the next skip_id
    let offset: Option<ModelID> = harvests.pop().map(|h| h.id);

    // // Sort Harvests. Harvests are sorted by boost_amount
    // // and then with available_at date relative to today's date.
    // let today = OffsetDateTime::now_utc().date();
    // harvests.sort_unstable_by(|a, b| {
    //     a.boost_amount
    //         .cmp(&b.boost_amount)
    //         .then_with(|| {
    //             ((today - a.available_at).abs())
    //                 .cmp(&(today - b.available_at).abs())
    //                 .reverse()
    //         })
    //         .reverse()
    // });

    Ok(Json(HarvestFeed { harvests, offset }))
}

/// Harvests feed
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestFeed {
    harvests: HarvestList,
    // Used for querying the next result set.
    // if `offset_id` is `None`, the result set has been exhausted;
    // there is no more harvests to be returned.
    offset: Option<ModelID>,
}

/// `harvests/feed` query parameters.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestFilter {
    /// filters for cultivar name
    #[serde(default)]
    pub cultivar: Vec<String>,
    /// filters for region name
    #[serde(default)]
    pub region: Vec<String>,

    /// `skip_id` - position in the result set.
    /// query's harvests starting from this harvest_id.
    #[serde(default)]
    pub offset: Option<String>,
    /// maximum number of harvest should be returned
    #[serde(default = " default_harvests_len_limit")]
    pub limit: usize,
}

/// Default maximum number of harvests returned `20`
const fn default_harvests_len_limit() -> usize {
    20
}

impl HarvestFilter {
    /// `skip_id` - position in the result set.
    pub fn offset_id(&self) -> Option<ModelID> {
        self.offset
            .as_ref()
            .and_then(|id| ModelID::try_from(id.as_str()).ok())
    }

    /// Cleaned cultivar name filters
    pub fn cultivars(&self) -> Vec<String> {
        self.cultivar.iter().map(|c| c.to_titlecase()).collect()
    }

    /// Cleaned region name filters
    pub fn regions(&self) -> Vec<Option<String>> {
        self.region.iter().map(|r| Some(r.to_titlecase())).collect()
    }
}
