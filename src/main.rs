use debrelka::{create_router, read_records, RecordSet, ServerState, SharedServerState, watch};

use axum::Server;
use log::{info, warn};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use uuid::Uuid;

pub async fn update_state(state: &SharedServerState, records: RecordSet) -> bool {
    let mut state = state.write().await;
    if state.records != records {
        state.records = records;
        state.tag = Uuid::new_v4();
        true
    } else {
        false
    }
}

fn main() {

    env_logger::init();

    let records = Default::default();
    let tag = Uuid::new_v4();
    let state: SharedServerState = ServerState{records, tag}.into();

    let rt = Runtime::new().unwrap();

    let (tx, mut rx) = channel(1);

    rt.spawn_blocking(move || {
        loop {
            let result = watch("./records.db", |path| {
                match read_records(path) {
                    Ok(records) => tx.blocking_send(records).unwrap(),
                    Err(err) => warn!("cannot read database: {:?}", err),
                }
            });
            if let Err(err) = result {
                warn!("failed to initialize watcher: {:?}", err);
            }
        }
    });

    {
        let state = state.clone();
        rt.spawn(async move {
            loop {
                let records = rx.recv().await.unwrap();
                if update_state(&state, records).await {
                    info!("records updated");
                }
            }
        });
    }

    rt.block_on(async move {
        Server::bind(&"127.0.0.1:3000".parse().unwrap())
            .serve(create_router(state).into_make_service())
            .await
            .unwrap();
    });
}
