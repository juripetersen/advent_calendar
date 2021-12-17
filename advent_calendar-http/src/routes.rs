use actix_web::web;

use crate::winners;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/winners").configure(winners::configure));
}
