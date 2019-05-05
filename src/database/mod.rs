/************************************************************************************************/

use crate::error::DialogueError;
use crate::text::so;
use crate::text::Text::*;
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
            Ok(connection) => Ok(Database {
                connection: connection,
            }),
            Err(e) => Err(DialogueError::new(e.to_string()).add(so(ErrorConnectingDb))),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn install(&self) -> Result<(), DialogueError> {
        match self.get_schema_version() {
            0 => self.install_script(include_str!("install_1.sql")),
            _ => Ok(()),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn install_script(&self, script: &str) -> Result<(), DialogueError> {
        match self.connection.batch_execute(script) {
            Ok(_) => Ok(()),
            Err(e) => Err(DialogueError::new(e.to_string()).add(so(ErrorInstallingSchema))),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn get_schema_version(&self) -> i32 {
        match self
            .connection
            .query("select dialogue.schema_version();", &[])
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
}

/************************************************************************************************/
