mod error;
mod unit;

/// 単位の生成に失敗したときのエラー
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnitError {
    Empty,
    Blank,
    Invalid(String),
}

/// 物理量の単位を表す値オブジェクト
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit(String);
