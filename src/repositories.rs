use diesel::{
    dsl::{now, IntervalDsl},
    prelude::*,
};
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
    pub async fn find_since(
        connection: &mut AsyncPgConnection,
        hours_since: i32,
    ) -> QueryResult<Vec<Crate>> {
        crates::table
            .filter(crates::created_at.gt(now - hours_since.hours()))
            .get_results(connection)
            .await
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
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(connection).await
    }
    pub async fn find_by_username(
        connection: &mut AsyncPgConnection,
        username: &String,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result(connection)
            .await
    }
    pub async fn find_with_roles(
        connection: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users_data = users::table.load::<User>(connection).await?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(connection)
            .await?
            .grouped_by(&users_data);

        Ok(users_data.into_iter().zip(result).collect())
    }
    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_data: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(&new_data)
            .get_result::<User>(connection)
            .await?;
        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(connection, &role_code).await {
                    NewUserRole {
                        role_id: role.id,
                        user_id: user.id,
                    }
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        code: role_code,
                        name,
                    };
                    let role = RoleRepository::create(connection, new_role).await?;
                    NewUserRole {
                        role_id: role.id,
                        user_id: user.id,
                    }
                }
            };
            diesel::insert_into(users_roles::table)
                .values(&new_user_role)
                .get_result::<UserRole>(connection)
                .await
                .expect("Error creating user role at create user repository");
        }
        Ok(user)
    }
    pub async fn delete(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id)))
            .execute(connection)
            .await?;
        diesel::delete(users::table.find(id))
            .execute(connection)
            .await
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_ids(
        connection: &mut AsyncPgConnection,
        ids: Vec<i32>,
    ) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results(connection)
            .await
    }
    pub async fn find_by_code(
        connection: &mut AsyncPgConnection,
        code: &RoleCode,
    ) -> QueryResult<Role> {
        roles::table
            .filter(roles::code.eq(code))
            .first(connection)
            .await
    }
    pub async fn find_by_user(
        connection: &mut AsyncPgConnection,
        user: &User,
    ) -> QueryResult<Vec<Role>> {
        /* roles::table
        .inner_join(users_roles::table)
        .filter(users_roles::user_id.eq(user_id))
        .select(roles::all_columns)
        .get_results(connection)
        .await */
        let user_roles = UserRole::belonging_to(&user)
            .get_results::<UserRole>(connection)
            .await?;
        let role_ids: Vec<i32> = user_roles
            .iter()
            .map(|user_role: &UserRole| user_role.role_id)
            .collect();
        Self::find_by_ids(connection, role_ids).await
    }

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
