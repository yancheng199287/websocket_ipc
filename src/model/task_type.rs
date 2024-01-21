use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::model::task_type::TaskType::Subscription;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FunctionPayload {
    // 定义函数任务的结构体字段
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ScriptPayload {
    // 定义脚本任务的结构体字段
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SubscriptionPayload {
    // 定义订阅任务的结构体字段
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Function,
    Script,
    Subscription,
}

impl TaskType {
/*    pub fn from_json(&self, json_str: &str) -> Result<Box<dyn TaskPayload>, Box<dyn std::error::Error>> {
        let payload = match self {
            Self::Function => FunctionPayload::from_str(json_str)?,
            Self::Script => ScriptPayload::from_str(json_str)?,
            Self::Subscription => SubscriptionPayload::from_str(json_str)?,
        };
        Ok(Box::new(payload) as Box<dyn TaskPayload>)
    }*/
}

// 抽象出一个 Trait 用于表示所有任务类型的 Payload
trait TaskPayload {}

impl TaskPayload for FunctionPayload {}

impl TaskPayload for ScriptPayload {}

impl TaskPayload for SubscriptionPayload {}

// 为了简化，这里使用了 `from_str()` 方法，实际上应使用 `serde_json::from_str()`
impl FromStr for FunctionPayload {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl FromStr for ScriptPayload {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl FromStr for SubscriptionPayload {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
