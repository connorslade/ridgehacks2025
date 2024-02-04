use afire::Server;

use crate::app::App;

mod info;
mod stats;
mod subscribe;
mod unsubscribe;

pub fn attach(server: &mut Server<App>) {
    info::attach(server);
    stats::attach(server);
    subscribe::attach(server);
    unsubscribe::attach(server);
}
