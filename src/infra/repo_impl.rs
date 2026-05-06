use crate::domain::repo::RpcPointerRepo;
use crate::domain::entity::RpcPointer;
use anyhow::Result;
use async_trait::async_trait;
use polodb_core::Database;
use polodb_core::bson::doc;
use polodb_core::Collection;
use polodb_core::CollectionT;

struct ID_GENERATOR;
impl ID_GENERATOR {
    pub fn generate() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

pub struct RpcPointerRepoImpl {
    pub db: Database,
    pub collection: Collection<RpcPointer>
}

impl RpcPointerRepoImpl {
    pub fn new(db: Database) -> Self {
        let db = db;
        let collection = db.collection::<RpcPointer>("rpc_pointer");
        RpcPointerRepoImpl {
            db,
            collection
        }
    }
}

#[async_trait]
impl RpcPointerRepo for RpcPointerRepoImpl {
    /// 获取所有RpcPointer
    async fn list(&self) -> Result<Vec<RpcPointer>> {
        let rpc_pointers = self.collection.find(doc!{}).run()?;
        let vec:Vec<RpcPointer> = rpc_pointers.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(vec)
    }

    /// 添加一个RpcPointer
    async fn add(&self, rpc_pointer: &mut RpcPointer) -> Result<()> {
        if rpc_pointer.id.is_empty() {
            rpc_pointer.id = ID_GENERATOR::generate();
            self.collection.insert_one(rpc_pointer)?;
        } else {
            anyhow::bail!("id is not empty");
        }
        Ok(())
    }

    /// 删除一个RpcPointer
    async fn delete(&self, id: &str) -> Result<()> {
        self.collection.delete_one(doc!{"id": id})?;
        Ok(())
    }

    /// 更新一个RpcPointer
    async fn update(&self, rpc_pointer: &RpcPointer) -> Result<()> {
        self.collection.update_one(doc!{"id": rpc_pointer.id.clone()}, doc!{"$set": doc!{"url": rpc_pointer.url.clone(), "default": rpc_pointer.default}})?;
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<RpcPointer>> {
        let rpc_pointer = self.collection.find_one(doc!{"id": id})?;
        Ok(rpc_pointer)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::RpcPointer;

    #[tokio::test]
    async fn test_rpc_pointer_repo() -> Result<()> {
        // 初始化db
        let db = Database::open_path("./db").unwrap();
        // 创建repo
        let rpc_pointer_repo = RpcPointerRepoImpl::new(db);

        let rpc_pointers = rpc_pointer_repo.list().await?;
        let len = rpc_pointers.len();
        // 新增数据
        let mut rpc_pointer = RpcPointer::new("https://api.devnet.solana.com".to_string(), false);
        rpc_pointer_repo.add(&mut rpc_pointer).await?;
        assert_ne!(rpc_pointer.id, "");

        // 验证是否添加成功
        let rpc_pointers = rpc_pointer_repo.list().await?;
        assert_eq!(rpc_pointers.len(), len + 1);

        // 更新数据
        rpc_pointer.url = "https://api.testnet.solana.com".to_string();
        rpc_pointer_repo.update(&rpc_pointer).await?;
        let get_rpc_pointer = rpc_pointer_repo.get(&rpc_pointer.id).await?;
        assert_eq!(get_rpc_pointer.unwrap().url, rpc_pointer.url);

        // 删除数据
        rpc_pointer_repo.delete(&rpc_pointer.id).await?;
        let rpc_pointers = rpc_pointer_repo.list().await?;
        assert_eq!(rpc_pointers.len(), len);

        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<()> {
        let db = Database::open_path("./db").unwrap();
        let rpc_pointer_repo = RpcPointerRepoImpl::new(db);
        let rpc_pointers = rpc_pointer_repo.list().await?;
        println!("{:?}", rpc_pointers);
        Ok(())
    }
}

