use serde::{Deserialize, Serialize};
/// 验证者信息
pub struct Validator {
    /// 投票节点地址
    pub vote_pubkey: String,

    /// 佣金
    pub commission: u8,

    /// 最新五个纪元的积分，1纪元 约 432,000 slot、约2天 , 1 slot等于400ms
    pub epoch_credits: Vec<(u64, u64, u64)>,
}


impl Validator {
    /// 创建
    pub fn new(vote_pubkey: String, commission: u8, epoch_credits: Vec<(u64, u64, u64)>) -> Self {
        Validator {
            vote_pubkey,
            commission,
            epoch_credits,
        }
    }
}


/// rpc节点信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcPointer {
    /// 节点名称
    pub id : String,
    /// rpc节点地址
    pub url: String,
    /// 是否默认
    pub default: bool,
}

impl RpcPointer {
    /// 创建
    pub fn new(url: String, default: bool) -> Self {
        RpcPointer { id: String::from(""), url, default }
    }

    /// 设置默认
    pub fn set_default(&mut self, default: bool) {
        self.default = default;
    }
}
