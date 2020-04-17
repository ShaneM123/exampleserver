use serde::{Deserialize, Serialize};
use crate::error_handler::CustomError;
use crate::schema::users;
use diesel::prelude::*;


#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}
#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}


impl Users {
    pub fn find_all(conn: &SqliteConnection) -> Result<Vec<Self>, CustomError> {
        users::table
            .select(users::all_columns)
            .first(conn)
            .map_err(Into::into)
    }
    pub fn find(conn: &SqliteConnection, id: i32) -> Result<Self, CustomError> {
        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }
    pub fn create_user(conn: &SqliteConnection, user: User) -> Result<Self, CustomError> {
        //The connection type supports a method transaction which takes a closure. The closure must return a Result.
        conn.transaction(|| {
            diesel::insert_into(users::table)
                .values((users::username.eq(user.username),))
                // error Result with our AppError error type because of our From implementation in our errors module.
                .execute(conn)?;

            //Sqlite does not support getting the id of a just inserted row as part of the insert statement.
            // to actually get the data back out to build a User struct we do another query.
            users::table
                .order(users::id.desc())
                .select((users::id, users::username))
                .first(conn)
                //uses the function signature to determine what to transform the error into.
                .map_err(Into::into)
        })
    }

   /* pub fn create(conn: &SqliteConnection, user: User) -> Result<Self, CustomError>{
      //  let conn = db::connection()?;
        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }*/

    pub fn update(conn: &SqliteConnection, id: i32, user: User) -> Result<Self, CustomError> {
        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;
        Ok(user)
    }


    pub fn delete(conn: &SqliteConnection, id: i32) -> Result<usize, CustomError> {
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
impl User {
    fn from(user: User) -> User {
        User {
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            age: user.age,

        }
    }
}