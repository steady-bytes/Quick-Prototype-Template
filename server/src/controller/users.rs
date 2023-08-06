use uuid::Uuid;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub enum UsersError{
    FailedCount,
    FailedRoleNameLookup,
    FailedUserInsert,
    FailedUserInsertUniqueEmail,
    FailedUserRoleInsert,
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
pub async fn insert_user(pool: &PgPool, params: &InsertUserParams) -> Result<Uuid, UsersError> {
    println!("insert_user");

    let id = Uuid::new_v4();

    // TODO: Hash password
    
    match sqlx::query("INSERT INTO users (id, email, password) values ($1, $2, $3)")
        .bind(id)
        .bind(&params.email)
        .bind(&params.password)
        .execute(pool)
        .await {
            Ok(_v) => {
                println!("{}", id);
            },
            Err(err) => {
                println!("{}", err);
                let e = err.as_database_error().and_then(|e| {e.constraint()});
                if e.is_some() {
                    if e.unwrap() == "users_email_key" {
                        return Err(UsersError::FailedUserInsertUniqueEmail)
                    }
                }

                return Err(UsersError::FailedUserInsert)
            },
        }

    // match insert_user_role(pool, id.to_string().as_str(), &params.role_name).await {
    //     Ok(_v) => println!("user role inserted"),
    //     Err(e) => {
    //         println!("{:?}", e)

    //         // TODO: rollback inserted_user
    //     },
    // }

    Ok(id)
}

pub struct InsertUserParams {
    pub email: String,
    pub password: String,
    pub role_name: String,
}

#[derive(sqlx::FromRow)]
struct Role {
    id: Uuid,
    _name: String,
    _description: String,
}

/// Insert a user role mapping to the `user_role` linking table
pub async fn insert_user_role(pool: &PgPool, user_id: &str, role_name: &str) -> Result<(), UsersError> {
    // query roles by name to get the role id
    let role_id = match sqlx::query_as::<_, Role>("SELECT id FROM roles WHERE name = ($1)")
        .bind(role_name)
        .fetch_one(pool).await {
            Ok(v) => v.id,
            Err(_e) => return Err(UsersError::FailedRoleNameLookup),
        };

    match sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(role_id)
        .execute(pool)
        .await {
            Ok(_v) => return Ok(()),
            Err(_e) => return Err(UsersError::FailedUserRoleInsert),
        }
}