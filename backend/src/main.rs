#[macro_use]extern crate rocket;
#[macro_use]extern crate diesel;

use chrono::{Duration, TimeZone, Utc};
use dotenv::dotenv;
use rocket::figment::{value::{Map, Value}, util::map};
use rocket::serde::json::Json;
use rocket::serde::{Serialize,Deserialize};
use rocket::response::status::Created;
use rocket_sync_db_pools;
use rocket_sync_db_pools::database;
use std::collections::{BTreeMap, HashSet};

mod models;

pub use models::models::{Bookable, Slot, State, Template, TimeRange};
pub use models::responses::{TimeItems, DailySortedSlots};


#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub success: bool,
    pub details: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSuccess {
    pub success: bool
}


impl ApiSuccess {

    pub fn new() -> ApiSuccess {
        ApiSuccess {success: true}
    }
}


static BOOKABLES: (&str, &str, &str) = ("Padel 1", "Padel 2", "Padel 3");


#[get("/")]
fn index() -> &'static str {
    "This is my Rust reservation REST API"
}


#[get("/getBookings1")]
fn get_booking_state_1() -> Json<Vec<Vec<Slot>>> {
    let slots = get_booking_data_day_1();
    let result = vec![
        vec![
            Slot {
                id: 2,
                start: slots[0].init,
                finish: slots[0].end,
                state: State::FREE,
                bookable: Bookable {id:1, name: String::from("Padel 1")},
            },
            Slot {
                id: 3,
                start: slots[0].init,
                finish: slots[0].end,
                state: State::FREE,
                bookable: Bookable {id:2, name: String::from("Padel 2")},
            },
        ],
        vec![
            Slot {
                id: 5,
                start: slots[1].init,
                finish: slots[1].end,
                state: State::FREE,
                bookable: Bookable {id:1, name: String::from("Padel 1")},
            },
            Slot {
                id: 6,
                start: slots[1].init,
                finish: slots[1].end,
                state: State::FREE,
                bookable: Bookable {id:2, name: String::from("Padel 2")},
            },
        ],
        vec![
            Slot {
                id: 8,
                start: slots[2].init,
                finish: slots[2].end,
                state: State::FREE,
                bookable: Bookable {id:1, name: String::from("Padel 1")},
            },
            Slot {
                id: 9,
                start: slots[2].init,
                finish: slots[2].end,
                state: State::FREE,
                bookable: Bookable {id:2, name: String::from("Padel 2")},
            },
        ],
        vec![
            Slot {
                id: 11,
                start: slots[3].init,
                finish: slots[3].end,
                state: State::FREE,
                bookable: Bookable {id:1, name: String::from("Padel 1")},
            },
            Slot {
                id: 12,
                start: slots[3].init,
                finish: slots[3].end,
                state: State::FREE,
                bookable: Bookable {id:2, name: String::from("Padel 2")},
            },
        ],
        vec![
            Slot {
                id: 14,
                start: slots[4].init,
                finish: slots[4].end,
                state: State::FREE,
                bookable: Bookable {id:2, name: String::from("Padel 1")},
            },
            Slot {
                id: 15,
                start: slots[4].init,
                finish: slots[4].end,
                state: State::FREE,
                bookable: Bookable {id:2, name: String::from("Padel 2")},
            },
        ],
    ];

    Json(result)
}


#[get("/getBookings2")]
fn get_booking_state_2() -> Json<BTreeMap<String, Vec<Slot>>> {
    let slots = get_booking_data_day_1();

    let result = BTreeMap::from([
        (
            slots[0].to_string(),
            vec![
                Slot {
                    id: 1,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                    bookable: Bookable {id:1, name: String::from("Padel 1")},
                },
                Slot {
                    id: 2,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                    bookable: Bookable {id:2, name: String::from("Padel 2")},
                },
            ],
        ),
        (
            slots[1].to_string(),
            vec![
                Slot {
                    id: 3,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                    bookable: Bookable {id:1, name: String::from("Padel 1")},
                },
                Slot {
                    id: 4,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                    bookable: Bookable {id:2, name: String::from("Padel 2")},
                },
            ],
        ),
        (
            slots[2].to_string(),
            vec![
                Slot {
                    id: 5,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                    bookable: Bookable {id:1, name: String::from("Padel 1")},
                },
                Slot {
                    id: 6,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                    bookable: Bookable {id:2, name: String::from("Padel 2")},
                },
            ],
        ),
    ]);

    Json(result)
}


#[get("/getBookings4")]
fn get_booking_state_4() -> Json<Vec<TimeItems>> {

    let slots = get_booking_data_day_1();
    let result = vec![
        TimeItems {
            time: slots[0],
            items: vec![
                Slot {
                    id: 1,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                    bookable: Bookable {id: 1, name: String::from("Padel 1")},
                },
                Slot {
                    id: 2,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                    bookable: Bookable {id: 2, name: String::from("Padel 2")},
                },
            ],
        },
        TimeItems {
            time: slots[1],
            items: vec![
                Slot {
                    id: 3,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                    bookable: Bookable {id: 1, name: String::from("Padel 1")},
                },
                Slot {
                    id: 4,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                    bookable: Bookable {id: 2, name: String::from("Padel 2")},
                },
            ],
        },
        TimeItems {
            time: slots[2],
            items: vec![
                Slot {
                    id: 5,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                    bookable: Bookable {id: 1, name: String::from("Padel 1")},
                },
                Slot{
                    id: 6,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                    bookable: Bookable {id: 2, name: String::from("Padel 2")},
                },
            ],
        },
        TimeItems {
            time: slots[3],
            items: vec![
                Slot{
                    id: 7,
                    start: slots[3].init,
                    finish: slots[3].end,
                    state: State::FREE,
                    bookable: Bookable {id: 1, name: String::from("Padel 1")},
                },
                Slot {
                    id: 8,
                    start: slots[3].init,
                    finish: slots[3].end,
                    state: State::FREE,
                    bookable: Bookable {id: 2, name: String::from("Padel 1")},
                },
            ],
        },
        TimeItems {
            time: slots[4],
            items: vec![
                Slot {
                    id: 9,
                    start: slots[4].init,
                    finish: slots[4].end,
                    state: State::FREE,
                    bookable: Bookable {id: 1, name: String::from("Padel 1")},
                },
                Slot{
                    id: 10,
                    start: slots[4].init,
                    finish: slots[4].end,
                    state: State::FREE,
                    bookable: Bookable {id: 2, name: String::from("Padel 1")},
                },
            ],
        },
    ];

    Json(result)
}


#[get("/getBookings5")]
fn get_bookings_state_5() -> Json<Vec<TimeItems>> {
    let template = create_template(30);
    let bookables = vec![
        Bookable {id: 1, name: String::from("Pista 1")},
        Bookable {id: 2, name: String::from("Pista 2")}
    ];
    let slots = Template::generate_slots(&template, &bookables, HashSet::new());
    let hourly_slots = Slot::to_hour_map(slots);
    let result = TimeItems::to_hour_response(hourly_slots);

    Json(result)
}


#[get("/getBookings6")]
async fn get_bookings_state_6(db: PgDatabase) -> Json<Vec<DailySortedSlots>>  {
    let template = create_template(30);
    let bookables = vec![
        Bookable {id: 1, name: String::from("Pista 1")},
        Bookable {id: 2, name: String::from("Pista 2")}
    ];
    let db_slots = db.run(|c| models::db::fetch_all_slots(c)).await.unwrap();
    let view_slots = Template::generate_slots(
        &template, &bookables, HashSet::from_iter(db_slots)
    );
    let daily_slots = Slot::to_day_map(view_slots);
    let result = DailySortedSlots::to_day_response(daily_slots);

    Json(result)
}


fn convert_db_error(error: diesel::result::Error) -> ApiError {
    ApiError {
        success: false,
        details: error.to_string()
    }
}


#[post("/addReservations",  data = "<slots>")]
async fn add_reservations(db: PgDatabase, slots: Json<Vec<Slot>>)
-> Result<Created<Json<ApiSuccess>>, Json<ApiError>> {
    db.run(|c| models::db::insert_slots(c, slots.into_inner()))
    .await
    .map(|_| Created::new("/addReservations").body(Json(ApiSuccess::new())))
    .map_err(|e| Json(convert_db_error(e)))
}


fn get_booking_data_day_1() -> Vec<TimeRange> {
    let init_1 = Utc.ymd(2022, 1, 1).and_hms(9, 0, 0);
    let end_1 = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0);
    let slot_1 = TimeRange::new(init_1, end_1);

    let init_2 = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0);
    let end_2 = Utc.ymd(2022, 1, 1).and_hms(11, 0, 0);
    let slot_2 = TimeRange::new(init_2, end_2);

    let init_3 = Utc.ymd(2022, 1, 1).and_hms(11, 0, 0);
    let end_3 = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
    let slot_3 = TimeRange::new(init_3, end_3);

    let init_4 = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
    let end_4 = Utc.ymd(2022, 1, 1).and_hms(13, 0, 0);
    let slot_4 = TimeRange::new(init_4, end_4);

    let init_5 = Utc.ymd(2022, 1, 1).and_hms(13, 0, 0);
    let end_5 = Utc.ymd(2022, 1, 1).and_hms(14, 0, 0);
    let slot_5 = TimeRange::new(init_5, end_5);

    vec![slot_1, slot_2, slot_3, slot_4, slot_5]
}


fn create_template(add: i64) -> Template {
    let template = Template {
        init_day: Utc::today(),
        end_day: Utc::today() + Duration::days(add),
        init_time: chrono::NaiveTime::from_hms(9, 0, 0),
        end_time: chrono::NaiveTime::from_hms(14, 0, 0),
        duration: Duration::minutes(30),
    };

    template
}


#[get("/test_database")]
async fn db_test(db: PgDatabase) -> &'static str {

    //let slot: db::Slot = db.run(|c| { schema::slots::table.first::<db::Slot>(c) }).await?;
    //let slots: Vec<db::Slot> = db.run(|c| { schema::slots::table.load::<db::Slot>(c) }).await?;
    //let time_ranges: Vec<db::TimeRange> = db.run(|c| { schema::time_range::table.load::<db::TimeRange>(c) }).await?;
    //let slots: Vec<db::Slot> = db.run(|c| { schema::slots::table.inner_join(db::TimeRange::table).load::<db::Slot>(c) }).await?;

    db.run(|c| {models::db::test_database(c)}).await;
    "This is a database test with diesel."
}


#[get("/init_database")]
async fn init_database(
    db: PgDatabase
) -> Result<Created<Json<ApiSuccess>>, Json<ApiError>> {
    db.run(|c| models::db::init_database(c, BOOKABLES))
    .await
    .map(|_| Created::new("/init_database").body(Json(ApiSuccess::new())))
    .map_err(|e| Json(convert_db_error(e)))
}


#[database("reservations_db")]
struct PgDatabase(rocket_sync_db_pools::diesel::PgConnection);


#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url: String = std::env::var("DATABASE_URL").unwrap();
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into()
    };

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("databases", map!["reservations_db" => db]));

    println!("{:?}", figment);

    rocket::custom(figment)
    .attach(PgDatabase::fairing())
    .mount(
        "/",
        routes![
            index,
            get_booking_state_1,
            get_booking_state_2,
            get_booking_state_4,
            get_bookings_state_5,
            get_bookings_state_6,
            init_database,
            add_reservations,
        ],
    )
}