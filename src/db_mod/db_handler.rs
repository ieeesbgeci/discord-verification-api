use super::{
    error_handler::{ApiError, CheckResponse},
    models::{CheckUser, DiscordUsers, NewUser},
    schema::discord_users,
    PoolConn,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use discord_users::dsl::*;

pub async fn add_db(pg_conn: PoolConn, data: web::Json<NewUser>) -> Result<(), ApiError> {
    let user = NewUser::new(&data.uname, &data.name,&data.e_mail);
    let res = diesel::insert_into(discord_users::table)
        .values(&user)
        .on_conflict(uname)
        .do_nothing()
        .execute(&pg_conn);
    match res {
        Err(err) => Err(ApiError::DbError(err)),
        Ok(_val) => Ok(()),
    }
    // .expect("Error adding data to Db");
    // .get_result(&pg_conn)
    // .unwrap()
}

// pub async fn disp_db(pg_conn:PoolConn)->Result<(),ApiError>{
// 	let result=discord_users::dsl::discord_users
// 				.load::<DiscordUsers>(&pg_conn)
// 				.expect("Error loading data from Db");
// 	for res in result{
// 		println!("{:?}",res);
// 	}
// 	Ok(())
// }

pub async fn check_db(
    pg_conn: PoolConn,
    data: web::Json<CheckUser>,
) -> Result<CheckResponse, ApiError> {
    let u_name: String = format!("{}", data.uname);
    let result = discord_users::dsl::discord_users
        .filter(discord_users::dsl::uname.eq(u_name))
        .load::<DiscordUsers>(&pg_conn);
    match result {
        Ok(val) if val.len() == 0 => Ok(CheckResponse::CheckFlag(false)),
        Ok(_) => Ok(CheckResponse::CheckFlag(true)),
        Err(err) => Err(ApiError::DbError(err)),
    }
}
