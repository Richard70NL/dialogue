/************************************************************************************************/

use crate::types::*;

/************************************************************************************************/

#[derive(Debug)]
pub struct Group {
    pub group_id: DbText,
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
    pub group_id: DbText,
    pub article_nr: DbInteger,
}

/************************************************************************************************/
