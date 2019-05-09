/************************************************************************************************/

use crate::error::DialogueError;
use crate::types::*;
use crate::util::parse_integer;

/************************************************************************************************/

#[derive(Debug)]
pub struct Group {
    pub group_id: GroupId,
    pub article_count: DbInteger,
    pub low_water_mark: DbInteger,
    pub high_water_mark: DbInteger,
}

/************************************************************************************************/

#[derive(Debug)]
pub struct Range {
    pub from: DbInteger,
    pub to: DbInteger,
}

/************************************************************************************************/

#[derive(Debug)]
pub struct ArticlePointer {
    pub group_id: GroupId,
    pub article_nr: DbInteger,
}

/************************************************************************************************/

#[derive(Debug, Clone)]
pub struct GroupId {
    parts: Vec<String>,
}

/************************************************************************************************/

impl Range {
    /*------------------------------------------------------------------------------------------*/

    pub fn parse(range_str: &str) -> Result<Range, DialogueError> {
        let v: Vec<&str> = range_str.split('-').collect();

        if v.is_empty() {
            Ok(Range { from: 0, to: 0 })
        } else if v.len() == 1 {
            let nr = parse_integer(v[0], 0)?;
            Ok(Range { from: nr, to: nr })
        } else {
            Ok(Range {
                from: parse_integer(v[0], 0)?,
                to: parse_integer(v[1], MAX_DB_INTEGER)?,
            })
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

impl GroupId {
    /*------------------------------------------------------------------------------------------*/

    pub fn from(group_id_str: &str) -> GroupId {
        if group_id_str.is_empty() {
            GroupId { parts: Vec::new() }
        } else {
            let parts_str: Vec<&str> = group_id_str.split('.').collect();
            let parts_string: Vec<String> = parts_str.iter().map(|s| s.to_string()).collect();

            // TODO validate each part with regex
            // ALPHA / DIGIT / "+" / "-" / "_"

            GroupId {
                parts: parts_string,
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn to_string(&self) -> String {
        let mut group_id_str = String::new();

        for (i, s) in self.parts.iter().enumerate() {
            if i > 0 {
                group_id_str.push('.')
            }
            group_id_str.push_str(&s)
        }

        group_id_str
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
