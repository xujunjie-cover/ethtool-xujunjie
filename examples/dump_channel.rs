// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;

// Once we find a way to load netsimdev kernel module in CI, we can convert this
// to a test
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(get_channel(None));

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(set_channel("eth0", 8));
}

async fn get_channel(iface_name: Option<&str>) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let mut channel_handle = handle.channel().get(iface_name).execute().await;

    let mut msgs = Vec::new();
    while let Some(msg) = channel_handle.try_next().await.unwrap() {
        msgs.push(msg);
    }
    assert!(!msgs.is_empty());
    for msg in msgs {
        println!("{msg:?}");
    }
}

async fn set_channel(iface_name: &str, channels: u32) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let mut channel_handle = handle.channel().set(iface_name).combined_count(channels).execute().await;
    let mut msgs = Vec::new();
    while let Some(msg) = channel_handle.try_next().await.unwrap() {
        msgs.push(msg);
    }
    for msg in msgs {
        println!("{msg:?}");
    }
}

