mod energy_source;
mod error;

#[derive(Debug)]
pub enum EnergySourceError {
    NotSupported(String),
}

/// 発電システム内の発電元を表す値オブジェクト
#[derive(Clone, Eq, PartialEq)]
pub struct EnergySource(String);
