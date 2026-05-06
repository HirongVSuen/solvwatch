use crate::domain::entity::RpcPointer;
use crate::domain::repo::RpcPointerRepo;
use std::sync::Arc;
use anyhow::Result;
pub struct RpcService<R: RpcPointerRepo> {
    repo: Arc<R>,
    rpc_pointer_list: Vec<RpcPointer>,
}

impl<R: RpcPointerRepo> RpcService<R> {
    pub async fn new(repo: Arc<R>) -> Result<Self> {
        let rpc_pointer_list = repo.list().await?;
        Ok(RpcService { repo, rpc_pointer_list })
    }

    pub async fn add_rpc_pointer(&mut self, mut rpc_pointer: RpcPointer) -> Result<()> {
        self.repo.add(&mut rpc_pointer).await?;
        self.rpc_pointer_list.push(rpc_pointer);
        Ok(())
    }

    pub async fn save(&mut self, rpc_pointer: &RpcPointer) -> Result<()> {
        self.repo.update(rpc_pointer).await
    }

    pub fn list(&self) -> Result<&Vec<RpcPointer>> {
        Ok(&self.rpc_pointer_list)
    }

}