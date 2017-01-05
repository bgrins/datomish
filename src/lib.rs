// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate datomish_query_parser;
extern crate rusqlite;

use rusqlite::Connection;

pub fn get_name() -> String {
  return String::from("datomish");
}

// Just an example of using a dependency
pub fn get_parser_name() -> String {
  return datomish_query_parser::get_name();
}

// Will ultimately not return the sqlite connection directly
pub fn get_connection() -> Connection {
    return Connection::open_in_memory().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_import_parser() {
        assert_eq!(String::from("datomish-query-parser"), get_parser_name());
    }
}