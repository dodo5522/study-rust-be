use super::errors::GenerationError;
use layer_domain::entity::SubSystemEntity;

/// グループ（サブシステム）を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait SubSystemRepositoryTrait<Tx> {
    /// グループ（サブシステム）を追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `e` - 新規登録するサブシステム
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, tx: &Tx, e: &SubSystemEntity) -> Result<(), GenerationError>;

    /// グループ（サブシステム）を取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `sub_system` - 情報取得する対象のサブシステム。指定なければ全て取得する。
    /// # Returns
    /// * `Result<Vec<GroupRecord>, GenerationRepositoryError>` - 成功時はグループ（サブシステム）のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(
        &self,
        tx: &Tx,
        sub_system: Option<impl AsRef<str> + Send>,
    ) -> Result<Vec<SubSystemEntity>, GenerationError>;

    /// グループ（サブシステム）を更新する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `e` - 更新する対象のサブシステム
    /// # Returns
    /// * `Result<UnitEntity, GenerationRepositoryError>` - 成功時は値を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn update(
        &self,
        tx: &Tx,
        e: &SubSystemEntity,
    ) -> Result<SubSystemEntity, GenerationError>;

    /// グループ（サブシステム）を削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `sub_system` - 削除するグループ（サブシステム）
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(
        &self,
        tx: &Tx,
        sub_system: impl AsRef<str> + Send,
    ) -> Result<(), GenerationError>;
}
