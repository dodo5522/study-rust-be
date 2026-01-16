use crate::value_object::{EnergySource, SubSystem, Unit};
use chrono::{DateTime, Local};

/// 発電状況の識別子
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct GenerationId(pub i64);

/// 発電状況エンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyRecord {
    /// 発電状況がシステムに登録済みであれば、そのIDが入る
    pub id: Option<GenerationId>,
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位
    pub unit: Unit,
    /// 発電サブシステムの種類
    pub sub_system: SubSystem,
    /// エネルギー源の種類
    pub energy_source: EnergySource,
    /// 発電状況のラベル
    pub label: String,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Local>,
}
