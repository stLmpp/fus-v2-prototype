use std::collections::HashSet;
use std::env;
use std::fs;

use sea_orm::{
    ActiveValue::Set, ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, EntityTrait,
    Order, QueryOrder, SelectColumns, Statement,
};

use crate::entity::{migrations, prelude::Migrations};

pub async fn run_database_migrations(database: DatabaseConnection) -> Result<(), DbErr> {
    let migrations = Migrations::find()
        .select_column(migrations::Column::Filename)
        .order_by(migrations::Column::CreatedAt, Order::Asc)
        .all(&database)
        .await?
        .iter()
        .map(|migration| migration.filename.clone())
        .collect::<Vec<String>>();
    let migrations_set: HashSet<String> = HashSet::from_iter(migrations);
    let current = env::current_dir().expect("Could not get current dir");
    let paths = fs::read_dir(current.join("migrations")).expect("Could not get migrations");
    for path_res in paths {
        let path = path_res.unwrap();
        let filename = path.file_name().into_string().unwrap();
        if migrations_set.contains(&filename) {
            continue;
        }
        let file_content = fs::read_to_string(path.path()).expect("Could not read file");
        database
            .execute(Statement::from_string(
                DatabaseBackend::Sqlite,
                file_content,
            ))
            .await?;
        Migrations::insert(migrations::ActiveModel {
            filename: Set(filename.to_owned()),
            created_at: Default::default(),
        })
        .exec(&database)
        .await?;
    }
    return Ok(());
}
