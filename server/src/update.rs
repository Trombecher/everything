use std::env::current_exe;
use tokio::process::Command;

pub async fn self_update() {
    let exe_path = current_exe().unwrap(); // TODO: remove this unwrap
    let has_target_in_path = exe_path.iter().find(|seg| *seg == "target").is_some();
    
    Command::new("rustup")
        .arg("update")
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
}