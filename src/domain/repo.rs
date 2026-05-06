use async_trait::async_trait;
use crate::domain::entity::RpcPointer;
use crate::domain::entity::Validator;
use anyhow::Result;

#[async_trait]
pub trait ValidatorRepo {

    /// 添加一个验证器
    async fn add(&self, validator: Validator) -> Result<()>;

    /// 获取所有验证器
    async fn list(&self) -> Result<Vec<Validator>>;

    /// 删除一个验证器
    async fn delete(&self, validator: Validator) -> Result<()>;
}

#[async_trait]
pub trait RpcPointerRepo {

    /// 获取所有RpcPointer
    async fn list(&self) -> Result<Vec<RpcPointer>>;

    /// 添加一个RpcPointer
    async fn add(&self, rpc_pointer: &mut RpcPointer) -> Result<()>;

    /// 删除一个RpcPointer
    async fn delete(&self, id: &str) -> Result<()>;

    /// 更新一个RpcPointer
    async fn update(&self, rpc_pointer: &RpcPointer) -> Result<()>;

    /// 获取一个RpcPointer
    async fn get(&self, id: &str) -> Result<Option<RpcPointer>>;
}

