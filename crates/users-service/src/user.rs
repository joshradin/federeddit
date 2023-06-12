//! Internal user

use common::repo::Repository;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{insert_into, select};
use r2d2::PooledConnection;

use crate::schema::user::dsl::user;
use users_api::auth::{PasswordAuth, PasswordError};
use users_api::{EmailAddress, User as UserTrait};

/// User with only public info exposed
#[derive(Debug, Clone)]
pub struct PublicUser {
    id: i64,
    email: EmailAddress,
    username: String,
}

impl PublicUser {
    pub fn create_new_user(
        conn: &mut MysqlConnection,
        email: &str,
        username: &str,
        password: &str,
    ) -> QueryResult<PublicUser> {
        use crate::schema::user::dsl;

        if user.select(InternalUser::as_select()).filter(dsl::email.eq(email)).first(conn).is_ok() {
            return Err(diesel::result::Error::NotFound)
        }

        insert_into(dsl::user)
            .values((
                dsl::username.eq(username),
                dsl::email.eq(email),
                dsl::password_hash.eq(password),
            ))
            .execute(conn)?;

        Ok(Self::get_user(conn, email).expect("user should have been created"))
    }

    pub fn get_user(conn: &mut MysqlConnection, email: &str) -> Option<PublicUser> {
        use crate::schema::user::dsl;

        let internal = user
            .filter(dsl::email.eq(email))
            .first::<InternalUser>(conn)
            .ok()?;

        Some(PublicUser {
            id: internal.id,
            email: EmailAddress::new_unchecked(internal.email),
            username: internal.username,
        })
    }

    pub fn verify_password(
        &self,
        conn: &mut MysqlConnection,
        auth: &PasswordAuth,
        pass: &str,
    ) -> Result<(), PasswordError> {
        use crate::schema::user::dsl;
        let hashed: String = user
            .select(dsl::password_hash)
            .find(self.id)
            .first(conn)
            .map_err(|_| PasswordError::NoPasswordFound)?;

        auth.verify_password(pass.as_bytes(), &hashed)
    }
}

impl PublicUser {
    pub fn new(email: EmailAddress, username: String) -> Self {
        Self {
            id: 0,
            email,
            username,
        }
    }
}

impl UserTrait for PublicUser {
    fn username(&self) -> &str {
        &self.username
    }

    fn set_username(&mut self, name: &str) {
        self.username = name.to_string();
    }

    fn email(&self) -> EmailAddress {
        self.email.clone()
    }
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::user)]
struct InternalUser {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
}
