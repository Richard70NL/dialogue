/************************************************************************************************/

use crate::data::Group;
use crate::data::Range;
use crate::error::DialogueError;
use crate::error::DialogueErrorType::*;
use crate::text::so;
use crate::text::Text::*;
use crate::types::*;
use postgres::Connection;
use postgres::TlsMode;

/************************************************************************************************/

pub struct Database {
    connection: Connection,
}

/************************************************************************************************/

impl Database {
    /*------------------------------------------------------------------------------------------*/

    pub fn open(dburl: &str) -> Result<Database, DialogueError> {
        match Connection::connect(dburl, TlsMode::None) {
            Ok(connection) => Ok(Database { connection }),
            Err(e) => Err(DialogueError::new(e.to_string()).add(so(ErrorConnectingDb))),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn install(&self) -> Result<(), DialogueError> {
        match self.get_schema_version() {
            0 => self.install_script(include_str!("scripts/install_1.sql")),
            _ => Ok(()),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn install_test_data(&self) -> Result<(), DialogueError> {
        self.install_script(include_str!("scripts/test_data.sql"))
    }

    /*------------------------------------------------------------------------------------------*/

    fn install_script(&self, script: &str) -> Result<(), DialogueError> {
        match self.connection.batch_execute(script) {
            Ok(_) => Ok(()),
            Err(e) => Err(DialogueError::new(e.to_string()).add(so(ErrorInstallingSchema))),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn get_schema_version(&self) -> DbInteger {
        match self
            .connection
            .query("select dialogue.schema_version()::integer;", &[])
        {
            Ok(rows) => {
                if rows.is_empty() {
                    // This will probably never happen. The function exists or not but it will
                    // never return zero rows.
                    0
                } else {
                    // Return the first field of the first row as the current schema version.
                    rows.get(0).get(0)
                }
            }
            Err(_) => {
                // HACK: This is not strictly correct. In this case we assume there is non
                // installed so a fresh install is needed. But an error can even be raised when
                // there is a schema already installed. But for now this will do so we just
                // return 0 as the version. We might need to change this so it actually checks
                // if the function dialogue.schema_version exists. If not it can return 0.
                // If it does however exist but still raises an error then there is something
                // wrong and this Rust function should fail!
                0
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn get_group(&self, group_str: &str) -> Result<Group, DialogueError> {
        match self
            .connection
            .query(include_str!("queries/get_group.sql"), &[&group_str])
        {
            Ok(rows) => {
                if rows.is_empty() {
                    Err(DialogueError::new(so(ErrorNoSuchGroup)).set_type(NoSuchGroup))
                } else {
                    let row = rows.get(0);

                    Ok(Group {
                        group_id: row.get("f_group_id"),
                        article_count: row.get("f_article_count"),
                        low_water_mark: row.get("f_low_water_mark"),
                        high_water_mark: row.get("f_high_water_mark"),
                    })
                }
            }
            Err(e) => Err(DialogueError::new(format!("{:?}", e)).add(so(ErrorSQL))),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn list_article_numbers(
        &self,
        group_id: &str,
        range: &Range,
    ) -> Result<Vec<DbInteger>, DialogueError> {
        match self.connection.query(
            include_str!("queries/list_article_numbers.sql"),
            &[&group_id, &range.from, &range.to],
        ) {
            Ok(rows) => {
                let article_numbers: Vec<DbInteger> =
                    rows.iter().map(|row| row.get("f_number")).collect();
                Ok(article_numbers)
            }
            Err(e) => Err(DialogueError::new(format!("{:?}", e)).add(so(ErrorSQL))),
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
