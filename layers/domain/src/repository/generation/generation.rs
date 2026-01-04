use super::error::GenerationRepositoryError;
use crate::value_object::{EnergySource, SubSystem, Unit};
use chrono::{DateTime, Local};

/// 発電状況を記録するためのリポジトリインターフェース
pub trait GenerationRepository {
    /// 発電状況を記録する
    ///
    /// # Arguments
    /// * `value` - 発電量の値
    /// * `unit` - 発電量の単位
    /// * `sub_system` - 発電システムのサブシステム
    /// * `energy_source` - エネルギー源の種類
    /// * `monitored_at` - 発電状況が記録された日時
    /// # Returns
    /// * `Result<u64, GenerationRepositoryError>` - 成功時は記録IDを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    fn add(
        &self,
        value: f32,
        unit: &Unit,
        sub_system: &SubSystem,
        energy_source: &EnergySource,
        monitored_at: DateTime<Local>,
    ) -> Result<u64, GenerationRepositoryError>;

    /// 発電状況を取得する
    ///
    /// # Arguments
    /// * `id` - 取得する発電状況のID
    /// # Returns
    /// * `Result<String, GenerationRepositoryError>` - 成功時は発電状況の文字列を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    fn get(&self, id: u32) -> Result<String, GenerationRepositoryError>;

    /// 発電状況を削除する
    ///
    /// # Arguments
    /// * `id` - 削除する発電状況のID
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    fn delete(&self, id: u32) -> Result<(), GenerationRepositoryError>;
}
