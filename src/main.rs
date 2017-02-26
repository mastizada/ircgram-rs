/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate irc;
extern crate telegram_bot;

use telegram_bot::*;
use irc::client::prelude::IrcServer;
use irc::client::prelude::Server;
use irc::client::prelude::ServerExt;
use irc::client::data::Config;

fn connect_irc() -> IrcServer {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    server
}

fn connect_telegram(conf: &Config) -> Api {
    let api = Api::from_token(conf.get_option("telegram_api")).unwrap();
    println!("getMe: {:?}", api.get_me());
    api
}

fn main() {
    let server = connect_irc();
    let telegram_api = connect_telegram(server.config());
    for message in server.iter() {
        let msg: String = format!("{}", message.unwrap());
        if msg.contains("PRIVMSG") {
            telegram_api.send_message(
                server.config().get_option("chat_id").parse::<i64>().unwrap(),
                format!("{}", msg),
                None, None, None, None
            );
        }
    }
}
