use entity::{user_roles, roles, users};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ActiveModelTrait};
use sea_orm_migration::sea_orm::{entity::Set};
use crate::sea_orm::prelude::{ChronoTime, ChronoDate, DateTime};

use chrono::{Utc, Datelike, Timelike};

pub fn get_current_timestamp() -> DateTime {
    let now = Utc::now(); 
    let date = ChronoDate::from_ymd(now.year(), now.month(), now.day());
    let time = ChronoTime::from_hms_milli(now.hour(), now.minute(), now.second(), now.timestamp_subsec_millis());

    DateTime::new(date, time)
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

/// MIGRATION
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        setup_poem_sessions(manager).await;
        setup_clients_table(manager).await;
        setup_users_table(manager).await;
        setup_roles_table(manager).await;
        setup_profiles_table(manager).await;
        setup_user_roles_table(manager).await;
        setup_invitations_table(manager).await;
        setup_merchant_table(manager).await;
        setup_location_kind_table(manager).await;
        setup_location_table(manager).await;
        setup_map_point_table(manager).await;
        setup_deliveries_table(manager).await;
        setup_merchant_invitations_table(manager).await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_clients_table(manager).await;
        drop_user_roles_table(manager).await;
        drop_roles_table(manager).await;
        drop_profiles_table(manager).await;
        drop_users_table(manager).await;
        drop_invitations_table(manager).await;
        drop_merchant_table(manager).await;
        drop_location_table(manager).await;
        drop_location_kind_table(manager).await;
        drop_deliveries_table(manager).await;
        drop_map_point_table(manager).await;
        drop_merchant_invitations_table(manager).await;

        Ok(())
    }
}

/// PoemSessions - Table to hold the poem sessions
#[derive(Iden)]
enum PoemSessions {
    Table,
    Id,
    Expires,
    Session
}

async fn setup_poem_sessions(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(PoemSessions::Table).if_not_exists()
            .col(ColumnDef::new(PoemSessions::Id).string().not_null().primary_key())
            .col(ColumnDef::new(PoemSessions::Expires).timestamp_with_time_zone())
            .col(ColumnDef::new(PoemSessions::Session).json_binary().not_null())
            .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("poem_sessions table created")
        }
    }

    let idx = manager.create_index(
        Index::create()
            .name("poem_sessions_expires_idx")
            .table(PoemSessions::Table)
            .col(PoemSessions::Expires)
            .to_owned()
    ).await;

    match idx {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("poem_session_expires_idx index created")
        }
    }
}

/// Clients - Are applications that can request data from the system
#[derive(Iden)]
enum Clients {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    // concrete table fields
    Name,
    Secret,
}

async fn setup_clients_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Clients::Table).if_not_exists()
        .col(ColumnDef::new(Clients::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Clients::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Clients::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Clients::DeletedAt).timestamp())
        .col(ColumnDef::new(Clients::Name).string().not_null())
        .col(ColumnDef::new(Clients::Secret).string().not_null())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("clients table created")
        }
    }
}

async fn drop_clients_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Clients::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("clients table dropped")
        }
    }
}

/// Users - A person or client that can perform some actions in the system
#[derive(Iden)]
enum Users {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    Password,
}

async fn setup_users_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Users::Table).if_not_exists()
        .col(ColumnDef::new(Users::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Users::DeletedAt).timestamp())
        .col(ColumnDef::new(Users::Password).string().not_null())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("users table created")
        }
    }
}

async fn drop_users_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Users::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("users table dropped")
        }
    }
}

/// Role - A possible role an user, event, type, endpoint, or data_field is related to
///
/// Default beginning roles
/// 1. CONSUMER            - A medical/retail person that might want to schedule a delivery of a good
/// 2. MERCHANT_ADMIN      - Generally a key holder/gm of a retail brick and mortar store. (could also be the owner of the business)
/// 3. MERCHANT_ ASSOCIATE - A RETAIL ASSOCIATE THAT WILL SCHEDULE THE DELIVERY ON AN ORDER FOR THE ‘RETAIL_CONSUMER’.
/// 4. DELIVERY_DRIVER     - A VERIFIED DELIVERY PERSON THAT CAN BE SCHEDULED TO DELIVER PRODUCTS
/// 5. ACCOUNT_ADMIN       - A sales person working for the delivery application organization, that can onboard new merchant accounts. (Jono)
/// 6. SYSTEM_ADMIN        - System administrator (Ian, Andrew)
#[derive(Iden)]
enum Roles {
    Table,
    Uuid,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Name,
}

async fn setup_roles_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
    Table::create().table(Roles::Table).if_not_exists()
        .col(ColumnDef::new(Roles::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Roles::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Roles::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Roles::DeletedAt).timestamp())
        .col(ColumnDef::new(Roles::Name).string().not_null().unique_key())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("roles table created")
        }
    }

    let db = manager.get_connection();
    let roles_seed = vec!["CONSUMER", "MERCHANT_ADMIN", "MERCHANT_ASSOCIATE", "DELIVERY_DRIVER", "ACCOUNT_ADMIN", "SYSTEM_ADMIN"];

    for r in roles_seed.iter() {
        let res = roles::ActiveModel {
            uuid: Set(sea_orm::prelude::Uuid::new_v4()),
            name: Set(r.to_string()),
            created_at: Set(get_current_timestamp()),
            updated_at: Set(get_current_timestamp()),
            ..Default::default()
        }
        .insert(db)
        .await; 
        
        match res {
            Ok(v) => println!("role added: {}", v.name),
            Err(e) => panic!("{}", e)
        }
    }
}

async fn drop_roles_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Roles::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("roles table dropped")
        }
    }
}

#[derive(Iden)]
enum Profiles {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    UserUuid,
    Email,
    ThemeMode,
    PictureUrl,
    EmailVerified,
    PhoneNumber,
    Username,
}

async fn setup_profiles_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Profiles::Table).if_not_exists()
        .col(ColumnDef::new(Profiles::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Profiles::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Profiles::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Profiles::DeletedAt).timestamp())
        .col(ColumnDef::new(Profiles::UserUuid).uuid().not_null().unique_key())
        .col(ColumnDef::new(Profiles::Email).string().not_null().unique_key())
        .col(ColumnDef::new(Profiles::PhoneNumber).string().not_null().unique_key())
        .col(ColumnDef::new(Profiles::ThemeMode).string().not_null())
        .col(ColumnDef::new(Profiles::PictureUrl).string().not_null())
        .col(ColumnDef::new(Profiles::EmailVerified).boolean().default(false))
        .col(ColumnDef::new(Profiles::Username).string())
        .to_owned()
    ).await;


    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("profile table created")
        }
    };

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_user_uuid")
        .from(Profiles::Table, Profiles::UserUuid)
        .to(Users::Table, Users::Uuid)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade)
        .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("user_uuid => users::uuid")
        }
    }
}

async fn drop_profiles_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Profiles::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("Profile table dropped")
        }
    }
}

#[derive(Iden)]
enum UserRoles {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    UserUuid,
    RolesUuid,
}

async fn setup_user_roles_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
    Table::create().table(UserRoles::Table).if_not_exists().primary_key(
        Index::create().col(UserRoles::UserUuid).col(UserRoles::RolesUuid))
                .col(ColumnDef::new(UserRoles::Uuid).uuid().not_null().unique_key())
                .col(ColumnDef::new(UserRoles::CreatedAt).timestamp().not_null())
                .col(ColumnDef::new(UserRoles::UpdatedAt).timestamp().not_null())
                .col(ColumnDef::new(UserRoles::DeletedAt).timestamp())
                .col(ColumnDef::new(UserRoles::UserUuid).uuid().not_null())
                .col(ColumnDef::new(UserRoles::RolesUuid).uuid().not_null())
                .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("user_roles table created")
        }
    }

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_user_uuid")
            .from(UserRoles::Table, UserRoles::UserUuid)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk user_uuid => users::uuid")
        }
    }

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_role_uuid")
            .from(UserRoles::Table, UserRoles::RolesUuid)
            .to(Roles::Table, Roles::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk role_uuid => roles::uuid")
        }
    }
}

async fn drop_user_roles_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(UserRoles::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("user_roles table dropped")
        }
    }
}

#[derive(Iden)]
enum Invitations {
    Table,
    Uuid,
    CreatedAt,
    InviteTemplateID,
    UserUuid,
}

async fn setup_invitations_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Invitations::Table).if_not_exists()
        .col(ColumnDef::new(Invitations::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Invitations::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Invitations::InviteTemplateID).integer().not_null())
        .col(ColumnDef::new(Invitations::UserUuid).uuid().not_null().unique_key())
        .to_owned(),
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("Invitations table created")
        }
    }

    // FK EXAMPLE
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_invitations_for_user")
            .from(Invitations::Table, Invitations::UserUuid)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk FROM => TO")
        }
    }
}

async fn drop_invitations_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Invitations::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("Invitations table dropped")
        }
    }
}

// merchant
#[derive(Iden)]
/// A Merchant is a business that exists in the system. Typically this is the parent type for an organization
/// that may have different locations associated with it. For example a store/distribution center is owned by a merchant.
/// I'm specifically keeping this generic so that it can be used for many different types of applications.
enum Merchant {
    Table,
    Uuid,
    CreatedBy,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    Name
}

async fn setup_merchant_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Merchant::Table).if_not_exists()
        .col(ColumnDef::new(Merchant::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Merchant::CreatedBy).uuid().not_null())
        .col(ColumnDef::new(Merchant::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Merchant::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Merchant::DeletedAt).timestamp())
        .col(ColumnDef::new(Merchant::Name).string().not_null())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("merchant table created")
        }
    }

    // If `CreatedBy` is used, supply a fk to the users table
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_merchant_created_by")
            .from(Merchant::Table, Merchant::CreatedBy)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk create_by => users::uuid")
        }
    }
}

async fn drop_merchant_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Merchant::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("merchant table dropped")
        }
    }
}

// MerchantInvitations - Invitations with verification codes that can be used to invite a `MERCHANT_ADMIN` or `MERCHANT_ASSOCIATE`.
#[derive(Iden)]
enum MerchantInvitations {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    MerchantUuid,
    VerificationCode,
}

async fn setup_merchant_invitations_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(MerchantInvitations::Table).if_not_exists()
        .col(ColumnDef::new(MerchantInvitations::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(MerchantInvitations::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(MerchantInvitations::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(MerchantInvitations::DeletedAt).timestamp())
        .col(ColumnDef::new(MerchantInvitations::MerchantUuid).uuid().not_null())
        .col(ColumnDef::new(MerchantInvitations::VerificationCode).string().string_len(9).not_null())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("MerchantInvitations table created")
        }
    }

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_merchant_invitations_to_merchant")
            .from(MerchantInvitations::Table, MerchantInvitations::MerchantUuid)
            .to(Merchant::Table, Merchant::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk FROM => TO")
        }
    }
}

async fn drop_merchant_invitations_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(MerchantInvitations::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("MerchantInvitations table dropped")
        }
    }
}


// LocationKind - Is a way of establishing different kinds of physical locations of a merchant. (i.e Store, Distribution Center, Warehouse)
#[derive(Iden)]
enum LocationKind {
    Table,
    Uuid,
    CreatedBy,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    // concrete table fields
    Name,
    Description
}

async fn setup_location_kind_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(LocationKind::Table).if_not_exists()
        .col(ColumnDef::new(LocationKind::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(LocationKind::CreatedBy).uuid().not_null())
        .col(ColumnDef::new(LocationKind::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(LocationKind::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(LocationKind::DeletedAt).timestamp())
        .col(ColumnDef::new(LocationKind::Name).string())
        .col(ColumnDef::new(LocationKind::Description).string())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("LocationKind table created")
        }
    }

    // If `CreatedBy` is used, supply a fk to the users table
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_location_kind_created_by")
            .from(LocationKind::Table, LocationKind::CreatedBy)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk create_by => users::uuid")
        }
    }
}

async fn drop_location_kind_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(LocationKind::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("LocationKind table dropped")
        }
    }
}

/// Location represents a physical location of a merchant. The kind of location is determined in the `LocationKind` table
#[derive(Iden)]
enum Location {
    Table,
    Uuid,
    CreatedBy,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    // concrete table fields
    Name,
    Address,
    // GEOM POINT (x,y)
    MerchantUuid,
    KindUuid,
}

async fn setup_location_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Location::Table).if_not_exists()
        .col(ColumnDef::new(Location::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Location::CreatedBy).uuid().not_null())
        .col(ColumnDef::new(Location::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Location::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Location::DeletedAt).timestamp())
        // 
        .col(ColumnDef::new(Location::Name).string().not_null())
        .col(ColumnDef::new(Location::Address).string().not_null())
        .col(ColumnDef::new(Location::MerchantUuid).uuid().not_null())
        .col(ColumnDef::new(Location::KindUuid).uuid().not_null())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("location table created")
        }
    }

    // If `CreatedBy` is used, supply a fk to the users table
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_location_create_by")
            .from(Location::Table, Location::CreatedBy)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk create_by => users::uuid")
        }
    }

    // fk to merchant
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_location_merchant")
            .from(Location::Table, Location::MerchantUuid)
            .to(Merchant::Table, Merchant::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk FROM location => TO merchant")
        }
    }

    // fk to location_kind
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_location_kind")
            .from(Location::Table, Location::KindUuid)
            .to(LocationKind::Table, LocationKind::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk FROM location => TO location_kind")
        }
    }
}

async fn drop_location_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Location::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("location table dropped")
        }
    }
}

/// MapPoint - Is a coded location that can be used for a start of end of a delivery
#[derive(Iden)]
enum MapPoint {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    //
    Lng,
    Lat,
    Geom,
    AddressLine1,
    AddressLine2,
    City,
    State,
    Zip
}

async fn setup_map_point_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(MapPoint::Table).if_not_exists()
        .col(ColumnDef::new(MapPoint::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(MapPoint::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(MapPoint::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(MapPoint::DeletedAt).timestamp())
        //
        .col(ColumnDef::new(MapPoint::Lat).float())
        .col(ColumnDef::new(MapPoint::Lng).float())
        .col(ColumnDef::new(MapPoint::AddressLine1).string())
        .col(ColumnDef::new(MapPoint::AddressLine2).string())
        .col(ColumnDef::new(MapPoint::City).string())
        .col(ColumnDef::new(MapPoint::State).string())
        .col(ColumnDef::new(MapPoint::Zip).integer())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("Deliveries table created")
        }
    }
}

async fn drop_map_point_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(MapPoint::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("MapPoint table dropped")
        }
    }
}

/// Deliveries
/// 
#[derive(Iden)]
enum Deliveries {
    Table,
    Uuid,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    // concrete table fields
    Recipient,
    OriginMapPoint,
    DestinationMapPoint
}

async fn setup_deliveries_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(Deliveries::Table).if_not_exists()
        .col(ColumnDef::new(Deliveries::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(Deliveries::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(Deliveries::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(Deliveries::DeletedAt).timestamp())
        //
        .col(ColumnDef::new(Deliveries::Recipient).uuid())
        .col(ColumnDef::new(Deliveries::OriginMapPoint).uuid())
        .col(ColumnDef::new(Deliveries::DestinationMapPoint).uuid())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("Deliveries table created")
        }
    }

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_deliveries_to_recipient")
            .from(Deliveries::Table, Deliveries::Recipient)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk FROM => TO")
        }
    }

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_deliveries_to_map_point_origin")
            .from(Deliveries::Table, Deliveries::OriginMapPoint)
            .to(MapPoint::Table, MapPoint::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk deliveries_origin => map_point")
        }
    }

    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_deliveries_to_map_point_destination")
            .from(Deliveries::Table, Deliveries::DestinationMapPoint)
            .to(MapPoint::Table, MapPoint::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk deliveries_origin => map_point")
        }
    }
}

async fn drop_deliveries_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(Deliveries::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("Deliveries table dropped")
        }
    }
}