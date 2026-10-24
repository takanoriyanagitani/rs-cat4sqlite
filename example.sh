#!/bin/bash

wsm="./cat4sqlite.wasm"

hdir="$PWD"
gdir="/guest.d"
bname="example.sqlite.db3"
gpath="${gdir}/${bname}"
hpath="${hdir}/${bname}"

max_sql_size=1048576

run_wasmtime(){
  cat /dev/stdin |
    wasmtime \
      run \
      --env ENV_SQLITE_DB_FILE_PATH="${gpath}" \
      --env ENV_MAX_SQL_SIZE=$max_sql_size \
      --dir "${hdir}::${gdir}" \
      "${wsm}"
}

run_wazero(){
  cat /dev/stdin |
    wazero \
      run \
      -env ENV_SQLITE_DB_FILE_PATH="${gpath}" \
      -env ENV_MAX_SQL_SIZE=$max_sql_size \
      -mount "${hdir}:${gdir}" \
      "${wsm}"
}

run_wasi(){
  run_wasmtime
}

geninput(){
  echo creating example db...

  echo "
    CREATE TABLE IF NOT EXISTS tab1(
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      comment TEXT
    )
  " | sqlite3 "${hpath}"

  echo "
    INSERT INTO tab1(name, comment) VALUES (
      'j',
      'helo' || char(10) || 'wrld' || char(10)
    )
  " | sqlite3 "${hpath}"

  echo "
    INSERT INTO tab1(name, comment) VALUES (
      'k',
      'hello' || char(10) || 'world' || char(10)
    )
  " | sqlite3 "${hpath}"
}

test -f "${hpath}" || geninput

echo "
  SELECT comment FROM tab1
  ORDER BY id
" | run_wasi
