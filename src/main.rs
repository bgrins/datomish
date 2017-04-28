// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate clap;
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;

use clap::{App, Arg, SubCommand, AppSettings};
use slog::DrainExt;

use std::u16;
use std::str::FromStr;

extern crate time;

extern crate mentat;
extern crate mentat_db;

use mentat::{new_connection, conn};

use std::io::prelude::*;
use std::fs::{File};

// Steps for profiling:
// * cargo build --release
// * point profiler to target/release/mentat
fn main() {

    let mut sqlite = new_connection("").unwrap();
    let mut conn = conn::Conn::connect(&mut sqlite).unwrap();

    // Try to open relative to target/release/ for running profile, and also
    // from project root for cargo run
    let mut schema_file = match File::open("../../tests/music-schema.dtm") {
        Ok(s) => s,
        Err(why) => File::open("tests/music-schema.dtm").expect("Unable to open the file")
    };
    let mut schema_contents = String::new();
    schema_file.read_to_string(&mut schema_contents).expect("Unable to read the file.  TODO: Please download them at <URL>");
    let schema_transaction = conn.transact(&mut sqlite, schema_contents.as_str()).unwrap();

    // If you pull down the full dataset, you can replace the path here with
    // tests/music-data.dtm.
    let mut data_file = match File::open("../../tests/music-data-partial.dtm") {
        Ok(s) => s,
        Err(why) => File::open("tests/music-data-partial.dtm").expect("Unable to open the file")
    };
    let mut data_contents = String::new();
    data_file.read_to_string(&mut data_contents).expect("Unable to read the file.  TODO: Please download them at <URL>");
    let data_transaction = conn.transact(&mut sqlite, data_contents.as_str()).unwrap();

    let results = conn.q_once(&mut sqlite,
        r#"[:find ?name
        :where
        [?p :artist/name ?name]]"#, None)
    .expect("Query failed");
    println!("{:?}", results);
}
