use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{models::*, schema::*};

pub struct RustaceaRepository;

impl RustaceaRepository {
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(connection).await
    }
    pub async fn find_multiple(
        connection: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).get_results(connection).await
    }
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(&new_rustacean)
            .get_result(connection)
            .await
    }

    pub async fn update(
        connection: &mut AsyncPgConnection,
        id: i32,
        rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(connection)
            .await
    }

    pub async fn delete(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id))
            .execute(connection)
            .await
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(connection).await
    }
    pub async fn find_multiple(
        connection: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(connection).await
    }
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_crate: NewCrate,
    ) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(&new_crate)
            .get_result(connection)
            .await
    }

    pub async fn update(
        connection: &mut AsyncPgConnection,
        id: i32,
        crate_dto: NewCrate,
    ) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(crate_dto.rustacean_id),
                crates::name.eq(crate_dto.name),
                crates::code.eq(crate_dto.code),
                crates::version.eq(crate_dto.version),
                crates::description.eq(crate_dto.description),
            ))
            .get_result(connection)
            .await
    }

    pub async fn delete(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id))
            .execute(connection)
            .await
    }
}

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_data: NewUser,
        roles: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(&new_data)
            .get_result(connection)
            .await?;
        Ok(user)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_data: NewRole,
    ) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(&new_data)
            .get_result(connection)
            .await
    }
}
