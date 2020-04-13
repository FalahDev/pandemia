//! DAO untuk Sub Report
//!

use crate::{
    models::SubReport,
    result::Result,
    schema::sub_reports,
    sqlutil::lower,
    types::{EntriesResult, SubReportStatus},
    util, ID,
};
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::{dsl::any, sql_types};

#[derive(Insertable)]
#[table_name = "sub_reports"]
struct NewSubReport<'a> {
    pub creator_id: i64,
    pub creator_name: &'a str,
    pub full_name: &'a str,
    pub age: i32,
    pub residence_address: &'a str,
    pub gender: &'a str,
    pub coming_from: &'a str,
    pub arrival_date: NaiveDate,
    pub healthy: i32,
    pub notes: &'a str,
    pub status: i32,
    pub meta: &'a Vec<&'a str>,
    pub ts: NaiveDateTime,
    pub area_code: &'a str,
}

#[doc(hidden)]
pub struct UpdateSubReport<'a> {
    pub full_name: &'a str,
    pub age: i32,
    pub residence_address: &'a str,
    pub gender: &'a str,
    pub coming_from: &'a str,
    pub arrival_date: NaiveDate,
    pub healthy: i32,
    pub notes: &'a str,
    pub status: i32,
    pub meta: &'a Vec<&'a str>,
    pub area_code: &'a str,
}

/// Data Access Object for SubReport
#[derive(Dao)]
#[table_name = "sub_reports"]
pub struct SubReportDao<'a> {
    db: &'a PgConnection,
}

impl<'a> SubReportDao<'a> {
    /// Create new SubReport
    pub fn create(
        &self,
        creator_id: i64,
        creator_name: &'a str,
        full_name: &'a str,
        age: i32,
        residence_address: &'a str,
        gender: &'a str,
        coming_from: &'a str,
        arrival_date: NaiveDate,
        healthy: i32,
        notes: &'a str,
        status: i32,
        meta: &'a Vec<&'a str>,
        area_code: &'a str,
    ) -> Result<SubReport> {
        use crate::schema::sub_reports::{self, dsl};

        diesel::insert_into(sub_reports::table)
            .values(&NewSubReport {
                creator_id,
                creator_name,
                full_name,
                age,
                residence_address,
                gender,
                coming_from,
                arrival_date,
                healthy,
                notes,
                status,
                meta,
                ts: util::now(),
                area_code,
            })
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Update
    pub fn update(&self, id: ID, data: UpdateSubReport) -> Result<SubReport> {
        use crate::schema::sub_reports::{self, dsl};
        let result = diesel::update(dsl::sub_reports.filter(dsl::id.eq(id)))
            .set((
                dsl::full_name.eq(data.full_name),
                dsl::age.eq(data.age),
                dsl::residence_address.eq(data.residence_address),
                dsl::gender.eq(data.gender),
                dsl::coming_from.eq(data.coming_from),
                dsl::arrival_date.eq(data.arrival_date),
                dsl::healthy.eq(data.healthy),
                dsl::notes.eq(data.notes),
                dsl::status.eq(data.status),
                dsl::meta.eq(data.meta),
            ))
            .get_result::<SubReport>(self.db)?;
        Ok(result)
    }

    /// Search for specific sub report by creator
    pub fn search(
        &self,
        status: i32,
        area_code: &str,
        query: &str,
        creator_id: Option<i64>,
        offset: i64,
        limit: i64,
    ) -> Result<EntriesResult<SubReport>> {
        use crate::schema::sub_reports::{self, dsl};
        let mut filterer: Box<dyn BoxableExpression<sub_reports::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::area_code.eq(area_code));

        let query = query.trim();

        if query != "" {
            let like_clause = format!("%{}%", query).to_lowercase();
            filterer = Box::new(filterer.and(lower(dsl::full_name).like(like_clause)));
        }

        if let Some(creator_id) = creator_id {
            filterer = Box::new(filterer.and(dsl::creator_id.eq(creator_id)));
        }

        filterer = Box::new(filterer.and(dsl::status.eq(status)));

        Ok(EntriesResult::new(
            dsl::sub_reports
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .order(dsl::ts.desc())
                .load::<SubReport>(self.db)?,
            dsl::sub_reports
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }

    // /// Search for specific reports in area
    // pub fn area_search(
    //     &self,
    //     query: &str,
    //     province: &str,
    //     city: &str,
    //     offset: i64,
    //     limit: i64,
    // ) -> Result<EntriesResult<SubReport>> {
    //     use crate::schema::sub_reports::{self, dsl};
    //     let mut filterer: Box<dyn BoxableExpression<sub_reports::table, _, SqlType = sql_types::Bool>> =
    //         Box::new(dsl::id.ne(0));

    //     let query = query.trim();

    //     if query != "" {
    //         let like_clause = format!("%{}%", query).to_lowercase();
    //         filterer = Box::new(
    //             filterer.and(
    //                 lower(dsl::province)
    //                     .like(like_clause)
    //                     .and(dsl::creator_id.eq(province)),
    //             ),
    //         );
    //     } else {
    //         filterer = Box::new(filterer.and(dsl::creator_id.eq(creator_id)));
    //     }

    //     filterer = Box::new(filterer.and(dsl::status.eq(status)));

    //     Ok(EntriesResult::new(
    //         dsl::sub_reports
    //             .filter(&filterer)
    //             .offset(offset)
    //             .limit(limit)
    //             .order(dsl::ts.desc())
    //             .load::<SubReport>(self.db)?,
    //         dsl::sub_reports
    //             .filter(filterer)
    //             .select(diesel::dsl::count(dsl::id))
    //             .first(self.db)?,
    //     ))
    // }
}