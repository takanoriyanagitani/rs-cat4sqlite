use std::io;
use std::process::ExitCode;

use rs_cat4sqlite::MAX_SQL_SIZE;
use rs_cat4sqlite::stdin2sql2path2sqlite2concat2stdout;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_path2sqlite() -> impl Fn() -> String {
    io_envkey2val("ENV_SQLITE_DB_FILE_PATH")
}

fn io_max_sql_size() -> impl Fn() -> u64 {
    || {
        let mx: String = io_envkey2val("ENV_MAX_SQL_SIZE")();
        let parsed: u64 = str::parse(&mx).ok().unwrap_or(MAX_SQL_SIZE);
        parsed
    }
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let path2sqlite: String = io_path2sqlite()();
        let max_sql_size: u64 = io_max_sql_size()();
        stdin2sql2path2sqlite2concat2stdout(path2sqlite, max_sql_size)
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");

        let path2sqlite: String = io_path2sqlite()();
        let max_sql_size: u64 = io_max_sql_size()();

        eprintln!("ENV_SQLITE_DB_FILE_PATH: {path2sqlite}");
        eprintln!("ENV_MAX_SQL_SIZE: {max_sql_size}");

        ExitCode::FAILURE
    })
}
