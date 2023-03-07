//! Services routers impls

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post, put},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::{
    files::IMAGE_MAX_SIZE,
    server::state::ServerState,
    services::{
        farmers::{
            farm::handlers::{
                farm_create, farm_delete, farm_detail, farm_list, farm_location_index, farm_update,
            },
            location::{
                country::handlers::{country_create, country_delete, country_list, country_update},
                handlers::{
                    location_create, location_delete, location_detail, location_list,
                    location_update,
                },
                region::handlers::{region_create, region_delete, region_list, region_update},
            },
            rating::handlers::{
                farm_rating_create, farm_rating_delete, farm_rating_detail, farm_rating_list,
                farm_rating_update, farm_ratings,
            },
        },
        produce::{
            cultivar::{
                category::handlers::{
                    cultivar_category_create, cultivar_category_delete, cultivar_category_list,
                    cultivar_category_update,
                },
                handlers::{
                    cultivar_create, cultivar_delete, cultivar_detail, cultivar_image_delete,
                    cultivar_image_upload, cultivar_index, cultivar_list, cultivar_update,
                },
            },
            harvest::{
                handlers::{
                    harvest_create, harvest_delete, harvest_detail, harvest_image_delete,
                    harvest_image_uploads, harvest_list, harvest_update,
                },
                HARVEST_MAX_IMAGE,
            },
        },
    },
};

/// Services routers
pub fn routers() -> Router<ServerState> {
    Router::new()
        // Cultivar
        .route("/cultivars", get(cultivar_list).post(cultivar_create))
        .route(
            "/cultivars/:cultivar_id",
            get(cultivar_detail)
                .put(cultivar_update)
                .delete(cultivar_delete),
        )
        .route("/cultivars/index", get(cultivar_index))
        .route(
            "/cultivars/categories",
            get(cultivar_category_list).post(cultivar_category_create),
        )
        .route(
            "/cultivars/categories/:category_id",
            put(cultivar_category_update).delete(cultivar_category_delete),
        )
        .route(
            "/cultivars/:cultivar_id/photo",
            post(cultivar_image_upload).delete(cultivar_image_delete),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(IMAGE_MAX_SIZE))
        // Harvest
        .route("/harvests", get(harvest_list).post(harvest_create))
        .route(
            "/harvests/:harvest_id",
            get(harvest_detail)
                .put(harvest_update)
                .delete(harvest_delete),
        )
        .route(
            "/harvests/:harvest_id/photos",
            post(harvest_image_uploads).delete(harvest_image_delete),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            IMAGE_MAX_SIZE * HARVEST_MAX_IMAGE,
        ))
        // Farms
        .route("/farms", get(farm_list).post(farm_create))
        .route(
            "/farms/:farm_id",
            get(farm_detail).put(farm_update).delete(farm_delete),
        )
        .route(
            "/farms/:farm_id/locations",
            post(location_create).get(farm_location_index),
        )
        .route(
            "/farms/:farm_id/ratings",
            get(farm_ratings).post(farm_rating_create),
        )
        .route(
            "/farms/ratings/:rating_id",
            get(farm_rating_detail)
                .put(farm_rating_update)
                .delete(farm_rating_delete),
        )
        .route("/farms/ratings", get(farm_rating_list))
        // Locations
        .route("/locations", get(location_list))
        .route(
            "/locations/:location_id",
            get(location_detail)
                .put(location_update)
                .delete(location_delete),
        )
        .route(
            "/locations/countries",
            get(country_list).post(country_create),
        )
        .route(
            "/locations/countries/:country_id",
            put(country_update).delete(country_delete),
        )
        .route(
            "/locations/countries/:country_id/regions",
            get(region_list).post(region_create),
        )
        .route(
            "/locations/countries/regions/:region_id",
            put(region_update).delete(region_delete),
        )
}
