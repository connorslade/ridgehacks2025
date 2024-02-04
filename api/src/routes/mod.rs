use afire::Server;

use crate::app::App;

mod info;
mod subscribe;
mod unsubscribe;

pub fn attach(server: &mut Server<App>) {
    info::attach(server);
    subscribe::attach(server);
    unsubscribe::attach(server);
}
