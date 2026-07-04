use chrono::{DateTime, Utc};
use layer_domain::{entity::HistoryEntity, value_object::UnitError};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct HistoryItem {
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 発電サブシステムの種類(e.g. 太陽光, 風力, ...)
    pub sub_system: String,
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
}

#[derive(Deserialize, ToSchema)]
pub struct HistoryPostRequest {
    /// 発電状況の計測値
    pub data: Vec<HistoryItem>,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Utc>,
}

impl TryFrom<HistoryPostRequest> for Vec<HistoryEntity> {
    type Error = UnitError;

    fn try_from(input: HistoryPostRequest) -> Result<Self, Self::Error> {
        input
            .data
            .into_iter()
            .map(|item| {
                Ok(HistoryEntity {
                    value: item.value,
                    unit: item.unit.try_into()?,
                    sub_system: item.sub_system,
                    label: item.label,
                    monitored_at: input.monitored_at,
                })
            })
            .collect::<Result<Vec<HistoryEntity>, Self::Error>>()
    }
}
