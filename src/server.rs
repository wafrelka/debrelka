use std::sync::Arc;

use axum::{Extension, Json, Router, TypedHeader};
use axum::extract::Query;
use axum::headers::{ETag, IfNoneMatch};
use axum::http::StatusCode;
use axum::routing;
use serde_derive::Deserialize;
use tokio::sync::RwLock;
use tower_http::compression::CompressionLayer;
use uuid::Uuid;

use crate::record::{Record, RecordSet};

pub struct ServerState {
    pub records: RecordSet,
    pub tag: Uuid,
}
pub type SharedServerState = Arc<RwLock<ServerState>>;

impl From<ServerState> for SharedServerState {
    fn from(state: ServerState) -> Self {
        Arc::from(RwLock::from(state))
    }
}

#[derive(Deserialize, Debug)]
pub struct GetRecordsParams {
    from: Option<u64>,
    to: Option<u64>,
}

pub fn filter_records(records: &RecordSet, from: Option<u64>, to: Option<u64>) -> &[Record] {

    let from = from.unwrap_or(0);
    let to = to.unwrap_or(u64::MAX);

    let i = records.partition_point(|r| r.timestamp < from);
    let j = records.partition_point(|r| r.timestamp <= to);
    &records[i..j]
}

pub async fn get_records(
    params: Query<GetRecordsParams>,
    if_none_match: Option<TypedHeader<IfNoneMatch>>,
    state: Extension<SharedServerState>,
) -> (TypedHeader<ETag>, Result<Json<Vec<Record>>, StatusCode>) {

    let state = state.read().await;
    let etag: ETag = format!("\"{}\"", state.tag).parse().unwrap();

    if let Some(if_none_match) = if_none_match {
        if !if_none_match.precondition_passes(&etag) {
            return (TypedHeader(etag), Err(StatusCode::NOT_MODIFIED));
        }
    }

    let records: Vec<Record> = filter_records(&state.records, params.from, params.to).into();
    (TypedHeader(etag), Ok(records.into()))
}

pub fn create_router(state: SharedServerState) -> Router {
    Router::new()
        .route("/records", routing::get(get_records))
        .layer(CompressionLayer::new())
        .layer(Extension(state))
}

#[cfg(test)]
mod test {

    use crate::record::Record;
    use super::*;

    #[test]
    fn test_filter_records() {

        let records: RecordSet = [
            Record{timestamp: 1, value: 1.0},
            Record{timestamp: 2, value: 2.0},
            Record{timestamp: 3, value: 3.0},
            Record{timestamp: 4, value: 4.0},
            Record{timestamp: 5, value: 5.0},
        ].into_iter().collect();

        let expected = [
            Record{timestamp: 2, value: 2.0},
            Record{timestamp: 3, value: 3.0},
            Record{timestamp: 4, value: 4.0},
        ];
        let actual = filter_records(&records, Some(2), Some(4));

        assert_eq!(expected, actual);
    }
}
