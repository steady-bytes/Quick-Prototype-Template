use uuid::Uuid;
use sqlx::postgres::PgPool;
use crate::common::crypto;

#[derive(Debug)]
pub enum UsersError{
    FailedCount,
    FailedRoleNameLookup,
    FailedUserInsert,
    FailedUserInsertUniqueEmail,
    FailedUserRoleInsert,
    FailedUserTransactionCommit,
    FailedLogin,
}

/// Count the number of users that are in the system
pub async fn count_users(pool: &PgPool) -> Result<i64, UsersError> {
    match sqlx::query!("SELECT COUNT(*) FROM users").fetch_all(pool).await {
        Ok(v) => {
            match v[0].count {
                Some(count) => Ok(count),
                None => Err(UsersError::FailedCount)
            }
        },
        Err(_e) => Err(UsersError::FailedCount),
    }
}

/// Insert a new user into the system, email is considered a unique value
/// A role will also be added for the user using a database transaction
/// If one insert fails they both rollback
pub async fn insert_user(pool: &PgPool, params: &InsertUserParams) -> Result<Uuid, UsersError> {
    println!("insert_user");

    let id = Uuid::new_v4(); 
    let mut tx = pool.begin().await.unwrap();

    match save_user_tx(&mut tx, id, params).await {
        Ok(_v) => println!("user inserted {}", id),
        Err(e) => {
            println!("{:?}", e);
            let _e = tx.rollback().await;
            return Err(e)
        },
    };
    
    match insert_user_role_tx(&mut tx, id, &params.role_name).await {
        Ok(_v) => println!("user role inserted"),
        Err(e) => {
            println!("{:?}", e);
            let _e = tx.rollback().await;
            return Err(e)
        },
    }

    println!("did we make it through the tx");

    match tx.commit().await {
        Ok(_v) => return Ok(id),
        Err(_e) => return Err(UsersError::FailedUserTransactionCommit),
    }
}

pub async fn save_user_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, 
    id: Uuid, 
    params: &InsertUserParams
) -> Result<(), UsersError> {
    let pw = crypto::hash_password(params.password.clone());

    match sqlx::query("INSERT INTO users (id, email, password) values ($1, $2, $3)")
        .bind(id)
        .bind(&params.email)
        .bind(&pw)
        .execute(&mut **tx)
        .await {
            Ok(_v) => {
                return Ok(())
            },
            Err(err) => {
                let e = err.as_database_error().and_then(|e| {e.constraint()});
                if e.is_some() {
                    if e.unwrap() == "users_email_key" {
                        return Err(UsersError::FailedUserInsertUniqueEmail)
                    }
                }
                return Err(UsersError::FailedUserInsert);
            },
        }
}

#[derive(Debug)]
pub struct InsertUserParams {
    pub email: String,
    pub password: String,
    pub role_name: String,
}

#[derive(sqlx::FromRow)]
struct Role {
    id: Uuid,
    name: String,
    description: String,
}

/// Insert a user role mapping to the `user_role` linking table
pub async fn insert_user_role_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
    role_name: &str
) -> Result<(), UsersError> {
    println!("{}", role_name);

    let role_id = match sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE name = ($1)")
        .bind(role_name)
        .fetch_one(&mut **tx).await {
            Ok(v) => v.id,
            Err(_e) => return Err(UsersError::FailedRoleNameLookup),
        };

    match sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(role_id)
        .execute(&mut **tx)
        .await {
            Ok(_v) => return Ok(()),
            Err(e) => {
                println!("{}", e);
                return Err(UsersError::FailedUserRoleInsert)
            }
        }
}

#[derive(sqlx::FromRow, Debug)]
struct User {
    id: Uuid,
    email: String,
    password: String,
}

pub async fn attempt_user_login(
    pool: &PgPool, 
    email: String,
    password: String,
) -> Result<(), UsersError> {
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await {
            Ok(v) => v,
            Err(e) => return Err(UsersError::FailedLogin)
        };

    println!("yeah?, {:?}", user);

    match crypto::validate_password(user.password, password) {
        true => return Ok(()),
        false => return Err(UsersError::FailedLogin),
    };
}