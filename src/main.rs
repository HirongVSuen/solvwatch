// use solana_client::rpc_client::RpcClient;
// use solana_commitment_config::CommitmentConfig;
// use std::env;
// use anyhow::{Context, Result,Ok};
// #[tokio::main]
// async fn main() -> Result<()>{
//     let rpc_url = env::var("SOLANA_RPC_POINT").unwrap_or("https://api.devnet.solana.com".to_string());
//     println!("rpc_url: {}", rpc_url);
//     let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
//
//     let vote_accounts = rpc_client.get_vote_accounts().context("get vote accounts failed")?;
//
//     println!("总共有 {} 个活跃验证者\n", vote_accounts.current.len());
//
//     // 2. 打印前 3 个，看字段
//     for (i, v) in vote_accounts.current.iter().take(3).enumerate() {
//         println!("--- 验证者 #{} ---", i + 1);
//         println!("投票账户地址: {}", v.vote_pubkey);
//         println!("节点身份: {}", v.node_pubkey);
//         println!("激活质押: {} SOL)",
//                  v.activated_stake as f64 / 1_000_000_000.0
//         );
//         println!("佣金: {}%", v.commission);
//         println!("最后投票 slot: {:?}", v.last_vote);
//         println!("根 slot: {:?}", v.root_slot);
//         println!("epoch 积分记录数: {}", v.epoch_credits.len());
//         if let Some(last) = v.epoch_credits.last() {
//             println!("  最新: epoch={}, credits={}, prev={}", last.0, last.1, last.2);
//         }
//         println!();
//
//     }
//
//     // 3. 取当前 slot 和 epoch
//     let slot = rpc_client.get_slot()?;
//     // 4. 取 epoch 信息 1epoch = 432000 slots
//     let epoch_info = rpc_client.get_epoch_info()?;
//     println!("当前确认 slot: {}", slot);
//     println!("当前确认 epoch: {}", epoch_info.epoch);
//     println!("本 epoch 进度: {}/{} slots",
//              epoch_info.slot_index,
//              epoch_info.slots_in_epoch
//     );
//
//     for (i, v) in vote_accounts.current.iter().take(3).enumerate() {
//         println!("--- 验证者 #{} ---", i + 1);
//         println!("投票账户地址: {}", v.vote_pubkey);
//         let current_credits = v.epoch_credits.iter().find(|(c, _, _)| *c == epoch_info.epoch);
//         match current_credits {
//             Some(c) => {
//                 let current_credits = c.1 - c.2;
//                 println!("  本 epoch 积分: {}", current_credits);
//                 println!("  本 epoch 出块数: {}", epoch_info.slot_index);
//                 let rate = current_credits as f64 / epoch_info.slot_index as f64;
//                 println!("  本 epoch 投票平均积分: {:.2}", rate);
//                 let health = if rate > 95.0 {
//                     "🟢 优秀"
//                 } else if rate > 80.0 {
//                     "🟡 正常"
//                 } else {
//                     "🔴 预警"
//                 };
//                 println!("  健康状态: {}", health);
//             }
//             None => {
//                 println!("  本 epoch 还未参与");
//             }
//         }
//
//     }
//     Ok(())
// }

use tokio;
use anyhow::Result;
use solvwatch::infra::ratatui;

#[tokio::main]
async fn main() -> Result<()>{
    ratatui::run();
    Ok(())
}


