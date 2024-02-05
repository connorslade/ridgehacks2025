use afire::Server;

use crate::app::App;

mod admin;
mod info;
mod stats;
mod subscribe;
mod unsubscribe;

pub fn attach(server: &mut Server<App>) {
    admin::attach(server);
    info::attach(server);
    stats::attach(server);
    subscribe::attach(server);
    unsubscribe::attach(server);
}
