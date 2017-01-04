#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)] mod tests;

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;

struct HitCount(AtomicUsize);

#[get("/")]
fn index(hit_count: State<HitCount>) -> &'static str {
    hit_count.0.fetch_add(1, Ordering::Relaxed);
    "Your visit has been recorded!"
}

#[get("/count")]
fn count(hit_count: State<HitCount>) -> String {
    let count = hit_count.0.load(Ordering::Relaxed);
    format!("Hit count: {}", count)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, count])
        .manage(HitCount(AtomicUsize::new(0)))
        .launch();
}
