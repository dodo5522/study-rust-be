mod error;
mod sub_system;

#[derive(Debug)]
pub enum SubSystemError {
    NotSupported(String),
}

/// 発電システム内のBattery, Arrayなどのサブシステムを表す値オブジェクト
#[derive(Clone, Eq, PartialEq)]
pub struct SubSystem(String);
