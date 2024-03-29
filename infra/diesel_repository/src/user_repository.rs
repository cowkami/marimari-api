use anyhow;
use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use diesel::{
    pg::{upsert::excluded, PgConnection},
    prelude::*,
    r2d2::ConnectionManager,
    Insertable, Queryable,
};
use r2d2::Pool;

use db_schema::users;
use domain::{User, UserId, UserRepository};
use error::AppError;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
struct UserRecord {
    pub id: String,
    pub name: String,
}

impl From<&User> for UserRecord {
    fn from(user: &User) -> Self {
        UserRecord {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}

impl TryFrom<UserRecord> for User {
    type Error = anyhow::Error;

    fn try_from(user: UserRecord) -> anyhow::Result<User> {
        let UserRecord { id, name } = user;

        User::reconstruct(id, name)
    }
}

#[derive(new, Clone)]
pub struct UserRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            let user = UserRecord::from(user);
            let mut conn = self
                .pool
                .get()
                .with_context(|| AppError::Internal("failed to connect DB server".to_string()))?;

            diesel::insert_into(users::table)
                .values(user)
                .on_conflict(users::id)
                .do_update()
                .set(users::name.eq(excluded(users::name)))
                .execute(&mut conn)
                .with_context(|| {
                    AppError::Internal("failed to insert or update user".to_string())
                })?;

            Ok(())
        })
    }

    async fn get_by_ids(&self, ids: &[UserId]) -> anyhow::Result<Vec<User>> {
        tokio::task::block_in_place(|| {
            let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
            let mut conn = self
                .pool
                .get()
                .with_context(|| AppError::Internal("failed to connect DB server".to_string()))?;

            users::table
                .filter(users::id.eq_any(ids))
                .load::<UserRecord>(&mut conn)?
                .into_iter()
                .map(TryInto::try_into)
                .collect()
        })
    }
}
