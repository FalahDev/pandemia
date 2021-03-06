//! Definisi struct untuk model-model yang ada di dalam database.

use crate::{result::Result, schema::user_settings, types::RecordDiff};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::Serialize;

use std::fmt;

use crate::ID;

/// Interface untuk model yang pasti memiliki id
/// sehingga bisa dijadikan model generik untuk mendapatkan id
pub trait HasID {
    /// Get this record ID.
    fn get_id(&self) -> ID;
}

/// Bentuk model akun di dalam database.
#[derive(Queryable, Clone, Serialize, PartialEq)]
pub struct User {
    /// ID dari akun.
    pub id: i64,

    /// Nama lengkap akun.
    pub full_name: String,

    /// Alamat email dari akun.
    pub email: String,

    /// Nomor telepon.
    pub phone_num: String,

    /// Penanda apakah akun aktif atau tidak,
    /// apabila tidak aktif maka akun tidak diperkenankan untuk beroperasi.
    pub active: bool,

    /// Waktu kapan akun ini didaftarkan.
    pub register_time: NaiveDateTime,

    /// Location latitude
    pub latitude: f64,

    /// Location latitude
    pub longitude: f64,

    /// meta
    pub meta: Vec<String>,
}

impl User {
    /// Get area code for this user
    pub fn get_area_code(&self) -> &str {
        meta_value_str!(self, "area_code", "=")
    }

    /// Get village id where this user work
    pub fn get_village_id(&self) -> Option<ID> {
        meta_value_i64!(self, "village_id")
    }

    /// Get user location if registered
    /// if not return empty string ""
    pub fn get_village_name(&self) -> &str {
        meta_value_str!(self, "village", "=")
    }

    /// Get district id where this user work
    pub fn get_district_id(&self) -> Option<ID> {
        meta_value_i64!(self, "district_id")
    }

    /// Get user district name
    /// if not return empty string ""
    pub fn get_district_name(&self) -> &str {
        meta_value_str!(self, "district", "=")
    }

    /// Get city ID by area code
    pub fn get_city_id(&self) -> Option<ID> {
        meta_value_i64!(self, "city_id")
    }

    /// Check whether this user is blocked
    pub fn is_blocked(&self) -> bool {
        list_has_flag!(self.meta, "blocked")
    }

    /// Check whether this user is marked as deleted
    pub fn is_deleted(&self) -> bool {
        list_has_flag!(self.meta, "deleted")
    }

    /// Check whether this user is medic
    pub fn is_medic(&self) -> bool {
        list_has_flag!(self.meta, "medic")
    }
}

/// Bentuk model dari alamat untuk akun.
#[derive(Queryable)]
pub struct Address {
    /// ID dari record ini.
    pub id: i64,

    /// ID dari akun yang memiliki alamat ini.
    pub user_id: i64,

    /// Jenis alamat, 0: Domisili, 1: Kelahiran
    pub kind: i64,

    /// Alamat
    pub address: String,

    /// Kabupaten
    pub regency: String,

    /// Provinsi
    pub province: String,

    /// Negara
    pub country: String,

    /// Nomor telepon yang bisa dihubungi.
    pub phone_num: String,

    /// Penanda apakah alamat ini masih aktif atau tidak.
    pub active: bool,

    /// Catatan tentang alamat ini.
    pub notes: String,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct RegisterUser {
    // pub id: i64,
    pub token: String,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub register_time: NaiveDateTime,
    pub code: String,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, PartialEq, Debug)]
pub struct AccessToken {
    pub token: String,
    pub user_id: i64,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct UserPashash {
    pub user_id: i64,
    pub passhash: String,
    pub deperecated: bool,
    pub ver: i32,
    pub created: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct UserKey {
    pub id: ID,
    pub user_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User({}, {})", self.id, self.full_name)
    }
}

impl fmt::Display for UserKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({})", &self.pub_key[..8])
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Admin {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone_num: String,
    pub meta: Vec<String>,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

impl Admin {
    /// Get city ID by area code
    pub fn get_city_id(&self) -> Option<ID> {
        meta_value_i64!(self, "city_id")
    }

    /// Check whether user has access to some resource
    pub fn has_access(&self, access_name: &str) -> bool {
        if self.id == 1 {
            // selalu true untuk super admin
            return true;
        }
        self.meta
            .iter()
            .find(|a| *a == &format!("access.{}", access_name))
            .is_some()
    }

    /// Check whether this is super admin
    pub fn is_super_admin(&self) -> bool {
        self.id == 1
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct AdminAccessToken {
    pub token: String,
    pub admin_id: ID,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct ResetPasswordAdmin {
    pub admin_id: ID,
    pub token: String,
    pub created: NaiveDateTime,
    pub expiration: Option<NaiveDateTime>,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Record {
    pub id: ID,
    pub loc: String,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub latest: bool,
    pub meta: Vec<String>,
    pub last_updated: NaiveDateTime,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odp: i32,
    pub odpsp: i32,
    pub pdp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,

    pub loc_path: String,
}

impl Record {
    /// Get diff for this with other
    pub fn diff(&self, other: &Self) -> RecordDiff {
        let new_cases = self.total_cases - other.total_cases;
        let new_deaths = self.total_deaths - other.total_deaths;
        let new_recovered = self.total_recovered - other.total_recovered;
        let new_critical = self.critical_cases - other.critical_cases;

        RecordDiff {
            new_cases,
            new_deaths,
            new_recovered,
            new_critical,
        }
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Notif {
    pub id: ID,
    pub kind: i16,
    pub text: String,
    pub initiator_id: ID,
    pub receiver_id: ID,
    pub read: bool,
    pub keywords: Vec<String>,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Feed {
    pub id: ID,
    pub creator_id: ID,
    pub creator_name: String,
    pub loc: String,
    pub kind: i16,
    pub text: String,
    pub hashtags: Vec<String>,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct UserSetting {
    pub id: ID,
    pub user_id: ID,
    pub s_key: String,
    pub s_value: String,
}

#[derive(Insertable)]
#[table_name = "user_settings"]
struct NewUserSetting<'a> {
    user_id: ID,
    s_key: &'a str,
    s_value: &'a str,
}

#[doc(hidden)]
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct LatLong {
    /// The latitude
    pub lat: f64,
    /// The longitude
    pub long: f64,
}

impl User {
    /// Set user setting
    pub fn set_setting(&self, key: &str, value: &str, conn: &PgConnection) -> Result<()> {
        {
            use crate::schema::user_settings::{self, dsl};

            let already_exists: bool = user_settings::table
                .filter(dsl::user_id.eq(self.id).and(dsl::s_key.eq(key)))
                .count()
                .first::<i64>(conn)?
                > 0;
            if already_exists {
                diesel::update(dsl::user_settings.filter(dsl::user_id.eq(self.id).and(dsl::s_key.eq(key))))
                    .set(dsl::s_value.eq(&value))
                    .execute(conn)?;
            } else {
                diesel::insert_into(user_settings::table)
                    .values(&NewUserSetting {
                        user_id: self.id,
                        s_key: key,
                        s_value: value,
                    })
                    .execute(conn)?;
            }
        }

        // for optimization only
        if key == "enable_push_notif" {
            use crate::schema::user_connect::{self, dsl};
            diesel::update(dsl::user_connect.filter(dsl::user_id.eq(self.id)))
                .set(dsl::enable_push_notif.eq(value == "true"))
                .execute(conn)?;
        }

        Ok(())
    }

    /// Get user setting value.
    pub fn get_setting(&self, key: &str, conn: &PgConnection) -> Result<Option<String>> {
        use crate::schema::user_settings::{self, dsl};
        match user_settings::table
            .filter(dsl::user_id.eq(self.id).and(dsl::s_key.eq(key)))
            .select(dsl::s_value)
            .first::<String>(conn)
        {
            Ok(a) => Ok(Some(a)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all user settings
    pub fn get_settings(&self, conn: &PgConnection) -> Result<Vec<UserSetting>> {
        use crate::schema::user_settings::{self, dsl};
        user_settings::table
            .filter(dsl::user_id.eq(self.id))
            .load(conn)
            .map_err(From::from)
    }

    /// Define is current user is satgas
    pub fn is_satgas(&self) -> bool {
        self.meta.contains(&":satgas:".to_string())
    }

    /// Get latitude longitude
    pub fn get_lat_long(&self) -> LatLong {
        LatLong {
            lat: self.latitude,
            long: self.longitude,
        }
    }

    /// Check whether user has access to some resource
    pub fn has_access(&self, access_name: &str) -> bool {
        self.meta
            .iter()
            .find(|a| *a == &format!("access.{}", access_name))
            .is_some()
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct GeolocCache {
    pub id: ID,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct MapMarker {
    pub id: ID,
    pub name: String,
    pub info: String,
    pub latitude: f64,
    pub longitude: f64,
    pub kind: i16,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

impl MapMarker {
    /// Get i32 type meta value.
    pub fn get_meta_value_i32(&self, key: &str) -> i32 {
        self.meta
            .iter()
            .find(|a| a.starts_with(&format!("{}:", key)))
            .and_then(|a| a.splitn(2, ':').last())
            .and_then(|a| a.parse::<i32>().ok())
            .unwrap_or(0)
    }

    /// Get string type meta value.
    pub fn get_meta_value_str(&self, key: &str) -> &str {
        self.meta
            .iter()
            .find(|a| a.starts_with(&format!("{}:", key)))
            .and_then(|a| a.splitn(2, ':').last())
            .unwrap_or("")
    }
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Log {
    pub id: ID,
    pub activity: String,
    pub initiator_id: ID,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct SubReport {
    pub id: ID,
    pub creator_id: ID,
    pub creator_name: String,
    pub full_name: String,
    pub age: i32,
    pub residence_address: String,
    pub gender: String,
    pub coming_from: String,
    pub arrival_date: NaiveDate,
    pub healty: i32,
    pub notes: String,
    pub status: i32,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
    pub city_id: ID,
    pub district_id: ID,
    pub village_id: ID,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Village {
    pub id: ID,
    pub name: String,
    pub district_name: String,
    pub city: String,
    pub province: String,
    pub latitude: f64,
    pub longitude: f64,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
    pub city_id: ID,
    pub district_id: ID,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct City {
    pub id: ID,
    pub name: String,
    pub province: String,
    pub country_code: String,
    pub area_code: String,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct VillageData {
    pub id: ID,
    pub village_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub ts: NaiveDateTime,
    pub city_id: ID,
    pub meta: Vec<String>,
    pub district_id: ID,
    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct ReportNote {
    pub id: ID,
    pub title: String,
    pub notes: String,
    pub creator_id: ID,
    pub creator_name: String,
    pub city_id: ID,
    pub published: bool,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct KvStore {
    pub id: ID,
    pub a_key: String,
    pub a_val: String,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct District {
    pub id: ID,
    pub name: String,
    pub city_id: ID,
    pub meta: Vec<String>,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct DistrictData {
    pub id: ID,
    pub district_id: ID,
    pub odp: i32,
    pub pdp: i32,
    pub cases: i32,
    pub recovered: i32,
    pub deaths: i32,
    pub last_updated: NaiveDateTime,
    pub last_updated_by_id: ID,
    pub city_id: ID,
    pub meta: Vec<String>,
    pub ts: NaiveDateTime,

    pub ppdwt: i32,
    pub pptb: i32,
    pub odpsp: i32,
    pub pdps: i32,
    pub pdpm: i32,
    pub otg: i32,
}
