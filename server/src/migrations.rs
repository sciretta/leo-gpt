use crate::db_collections::{MigrationRecord, Users};
use chrono;
use mongodb::Database;
use mongodb::bson::doc;
use std::fmt::Error;

pub enum Migration {
    M0,
    M1,
}

impl Migration {
    pub fn name(&self) -> &'static str {
        match self {
            Migration::M0 => "m0_create_users",
            Migration::M1 => "m1_update_users",
        }
    }

    pub async fn up(&self, db: &Database) -> Result<(), Error> {
        match self {
            Migration::M0 => {
                let collection = db.collection::<Users>("users");
                collection
                    .insert_many(vec![Users {
                        name: "Alice".to_string(),
                        email: "alice@example.com".to_string(),
                        created_at: chrono::Utc::now().to_rfc3339(),
                    }])
                    .await
                    .unwrap();
                Ok(())
            }
            Migration::M1 => {
                let collection = db.collection::<Users>("users");
                collection
                    .update_many(doc! {}, doc! { "$set": { "is_active": true } })
                    .await
                    .unwrap();
                Ok(())
            }
        }
    }
}

pub async fn ensure_migrations_collection(db: &Database) -> Result<(), Error> {
    let collection = db.collection::<MigrationRecord>("_migrations");

    let _ = collection
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "name": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
        )
        .await;

    Ok(())
}

pub async fn has_migration_run(db: &Database, name: &str) -> Result<bool, Error> {
    let collection = db.collection::<MigrationRecord>("_migrations");
    let result = collection.find_one(doc! { "name": name }).await.unwrap();
    Ok(result.is_some())
}

pub async fn record_migration(db: &Database, name: &str) -> Result<(), Error> {
    let collection = db.collection::<MigrationRecord>("_migrations");
    collection
        .insert_one(MigrationRecord {
            name: name.to_string(),
            applied_at: chrono::Utc::now().to_rfc3339(),
        })
        .await
        .unwrap();
    Ok(())
}

pub async fn run_migrations(db: &Database) -> Result<(), Error> {
    ensure_migrations_collection(db).await.unwrap();

    let migrations = vec![Migration::M0, Migration::M1];

    for mig in migrations {
        if !has_migration_run(db, mig.name()).await.unwrap() {
            mig.up(db).await.unwrap();
            record_migration(db, mig.name()).await.unwrap();
            println!("✓ Applied migration: {}", mig.name());
        }
    }

    Ok(())
}
