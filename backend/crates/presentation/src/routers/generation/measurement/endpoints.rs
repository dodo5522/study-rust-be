use super::{
    get::{MeasurementItem, MeasurementRangeQuery, Response as GetResponse},
    post::PostMeasurementRequest,
};
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use layer_infra::{
    repository::measurement::MeasurementRepository, unit_of_work::UnitOfWorkFactory,
};
use layer_use_case::measurement::MeasurementUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    post,
    tag = "Generation - Measurement",
    description = "Create a new measurement record",
    path = "/generation/measurements",
    request_body = PostMeasurementRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn post_measurements(
    State(state): State<RouterState>,
    Json(body): Json<PostMeasurementRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let repo = MeasurementRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = MeasurementUseCase::new(repo, factory);
    let created = use_case
        .create(body.try_into().map_err(ErrorMapper::map_to_bad_request)?)
        .await;

    match created {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{error}"),
            }),
        )),
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Measurement",
    description = "Get a measurement record by id",
    path = "/generation/measurements/{id}",
    params(("id" = i64, Path, description = "Measurement id")),
    responses(
        (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurement(
    State(state): State<RouterState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<GetResponse>), (StatusCode, Json<ErrorResponse>)> {
    let repo = MeasurementRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = MeasurementUseCase::new(repo, factory);
    let measurement = use_case
        .get(id)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    match measurement {
        Some(measurement) => Ok((
            StatusCode::OK,
            Json(GetResponse {
                sub_system: measurement.sub_system,
                label: measurement.label,
                values: vec![MeasurementItem {
                    value: measurement.value,
                    unit: measurement.unit.to_string(),
                    monitored_at: measurement.monitored_at,
                }],
            }),
        )),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "Measurement record not found".to_string(),
            }),
        )),
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Measurement",
    description = "Get measurement records with date time range",
    path = "/generation/measurements",
    params(MeasurementRangeQuery),
    responses(
        (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements_with_range(
    State(state): State<RouterState>,
    Query(query): Query<MeasurementRangeQuery>,
) -> Result<(StatusCode, Json<GetResponse>), (StatusCode, Json<ErrorResponse>)> {
    let repo = MeasurementRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = MeasurementUseCase::new(repo, factory);
    // let measurement = use_case
    //     .get(id)
    //     .await
    //     .map_err(ErrorMapper::map_generation_error)?;

    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            message: "Measurement record not found".to_string(),
        }),
    ))
}
