use sea_query::{Query, PostgresQueryBuilder};
use crate::schema::Shows;

fn main() {
    let q = Query::insert().into_table(Shows::Table).build_sqlx(PostgresQueryBuilder);
}
