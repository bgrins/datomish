// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate clap;

extern crate iron;
extern crate staticfile;
extern crate mount;
use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;


use clap::{App, Arg, SubCommand, AppSettings};

fn main() {
    let app = App::new("Mentat").setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.subcommand(SubCommand::with_name("serve")
            .about("Starts a server")
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Print debugging info"))
            .arg(Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("FILE")
                .help("Path to the Mentat database to serve")
                .default_value("temp.db")
                .takes_value(true))
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("INTEGER")
                .help("Port to serve from, i.e. `localhost:PORT`")
                .default_value("3333")
                .takes_value(true)))
        .get_matches();
    if let Some(ref matches) = matches.subcommand_matches("serve") {
        let debug = matches.is_present("debug");
        println!("This doesn't work yet, but it will eventually serve the following database: {} \
                  on port: {}.  Debugging={}",
                 matches.value_of("database").unwrap(),
                 matches.value_of("port").unwrap(),
                 debug);

        let mut mount = Mount::new();

        // Serve the shared JS/CSS at /
        mount.mount("/", Static::new(Path::new("target/doc/")));
        // Serve the static file docs at /doc/
        mount.mount("/doc/", Static::new(Path::new("target/doc/mentat/")));
        // Serve the source code at /src/
        mount.mount("/src/", Static::new(Path::new("target/doc/src/staticfile/lib.rs.html")));

        println!("Doc server running on http://localhost:3000/doc/");

        Iron::new(mount).http("127.0.0.1:3000").unwrap();

    }
}
