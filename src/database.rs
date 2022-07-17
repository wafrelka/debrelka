use std::path::Path;

use rusqlite::{Connection, OpenFlags, Row};

use crate::record::{Record, RecordSet};

pub fn read_records<P: AsRef<Path>>(path: P) -> rusqlite::Result<RecordSet> {

    let conn = Connection::open_with_flags(path.as_ref(), OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let mut stmt = conn.prepare(r#"SELECT timestamp, value FROM "records" ORDER BY timestamp"#)?;
    let map = |row: &Row| { Ok(Record{timestamp: row.get(0)?, value: row.get(1)?}) };
    let resp = stmt.query_map([], map)?;
    resp.collect::<Result<_, _>>()
}
