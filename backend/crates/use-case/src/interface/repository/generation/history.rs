use super::errors::GenerationError;
use layer_domain::entity::HistoryEntity;

/// 発電状況を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait HistoryRepositoryTrait<Tx> {
    /// 発電状況を記録する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `histories` - 新規登録する発電状況
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空タプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, tx: &Tx, histories: Vec<HistoryEntity>) -> Result<(), GenerationError>;

    /// 発電状況を取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `id` - 取得する発電状況のID
    /// # Returns
    /// * `Result<Option<HistoryEntity>, GenerationRepositoryError>` - 成功時は発電状況のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self, tx: &Tx, id: i64) -> Result<Option<HistoryEntity>, GenerationError>;

    /// 発電状況を削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `id` - 削除する発電状況のID
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, tx: &Tx, id: i64) -> Result<(), GenerationError>;
}
