use std::io;
use std::path::Path;

use io::BufWriter;
use io::Write;

use io::BufRead;
use io::Read;

use rusqlite::Connection;
use rusqlite::OpenFlags;
use rusqlite::Row;
use rusqlite::Rows;
use rusqlite::Statement;
use rusqlite::types::ValueRef;

pub fn data2writer<W>(data: &[u8], wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    wtr.write_all(data)
}

pub fn value2writer_str<W>(val: &ValueRef, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    match val {
        ValueRef::Null => Ok(()),
        ValueRef::Text(dat) => data2writer(dat, wtr),
        ValueRef::Blob(dat) => data2writer(dat, wtr),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unexpected value type",
        )),
    }
}

pub fn row2writer_str<W>(row: &Row, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    let val: ValueRef = row.get_ref(0).map_err(io::Error::other)?;
    value2writer_str(&val, wtr)
}

pub fn rows2writer_str<W>(rows: &mut Rows, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    loop {
        let onext: Option<&Row> = rows.next().map_err(io::Error::other)?;
        let Some(next) = onext else {
            return wtr.flush();
        };

        row2writer_str(next, wtr)?
    }
}

pub fn stmt2writer_str<W>(stmt: &mut Statement, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    let mut rows: Rows = stmt.query(()).map_err(io::Error::other)?;
    rows2writer_str(&mut rows, wtr)
}

pub fn query2writer_str<W>(conn: &Connection, query: &str, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    let mut stmt: Statement = conn.prepare(query).map_err(io::Error::other)?;
    stmt2writer_str(&mut stmt, wtr)
}

pub fn brdr2sql2path2sqlite2concat2wtr<R, W, P>(
    brdr: R,
    mut wtr: W,
    path2sqlite: P,
    max_sql_size: u64,
) -> Result<(), io::Error>
where
    R: BufRead,
    W: Write,
    P: AsRef<Path>,
{
    let mut taken = brdr.take(max_sql_size);
    let mut buf: String = String::default();
    taken.read_to_string(&mut buf)?;

    let con: Connection =
        Connection::open_with_flags(path2sqlite, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(io::Error::other)?;
    query2writer_str(&con, &buf, &mut wtr)
}

pub const MAX_SQL_SIZE: u64 = 1048576;

pub fn stdin2sql2path2sqlite2concat2stdout<P>(
    path2sqlite: P,
    max_sql_size: u64,
) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let o = io::stdout();
    let mut ol = o.lock();
    brdr2sql2path2sqlite2concat2wtr(
        io::stdin().lock(),
        BufWriter::new(&mut ol),
        path2sqlite,
        max_sql_size,
    )?;
    ol.flush()
}
