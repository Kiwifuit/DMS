use denji::{ServerSoftware, ServerSoftwareOptions};
use humantime::format_duration;
use log::{error, info};
use tempdir::TempDir;
use tokio::task::spawn;

use std::sync::mpsc::channel;
use std::time::Duration;

const CHANNEL_TIMEOUT: Duration = Duration::from_secs(90);

#[tokio::main]
async fn main() {
    env_logger::init();

    let root_dir = TempDir::new("test.denji.serverInstall")
        .unwrap()
        .into_path();
    let install_server_opts = ServerSoftwareOptions::with(
        ServerSoftware::Quilt,
        "0.9.2",
        "1.20.4",
        root_dir,
        "dockerfs",
    );
    let (tx, rx) = channel();
    let install_task = spawn(async move { install_server_opts.build(tx).await });

    info!(
        "started installer (timeout: {})",
        format_duration(CHANNEL_TIMEOUT)
    );
    loop {
        match rx.recv_timeout(CHANNEL_TIMEOUT) {
            Err(_e) => {
                break;
            }
            Ok(line) => {
                info!("{}", line)
            }
        }
    }

    if let Err(e) = install_task.await.unwrap() {
        error!("error while installing: {}", e);
        panic!("what");
    }

    println!("you may test the channel and close this program when finished");
    loop {}
}
