use db_core::db::DbState;

use std::path::Path;


pub struct Settings<'a> {
    pub db_path: &'a Path,
}

impl<'a> Settings<'a> {
    pub fn new() -> Box<Settings<'a>> {
        Box::new(Settings {
            db_path: Path::new("composedb_data"),
        })
    }
}

pub struct State<'a> {
    db: DbState,
    pub db_path: &'a Path,
}

impl<'a> State<'a> {
    pub fn new(s: &Settings<'a>) -> Box<State<'a>> {
        Box::new(State {
            db: None,
            db_path: s.db_path
        })
    }
}
