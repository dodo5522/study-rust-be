use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct MeasurementRangeQuery {
    /// Date time measured from
    #[param(example = "2026-06-26T12:34:56Z")]
    pub from: DateTime<Utc>,
    /// Date time measured to
    #[param(example = "2026-06-26T21:34:56+09:00")]
    pub to: DateTime<Utc>,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct MeasurementItem {
    /// 物理量
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 計測日時
    pub monitored_at: DateTime<Utc>,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Response {
    /// 発電サブシステムの種類(e.g. 太陽光, 風力, ...)
    pub sub_system: String,
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
    /// 物理量の値と計測日時
    pub values: Vec<MeasurementItem>,
}
