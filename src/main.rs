// Copyright © 2016 the Bolt ϟ developers
//
// Bolt ϟ is free software; You can redistribute it and/or modify it under the terms of either:
// - the MIT License as published by the Massachusetts Institute of Technology.
// - the Apache License version 2 as published by the Apache Software Foundation.
//
// You don't have to do anything special to choose one license or the other and you don’t have to notify anyone which license you are using.
//
// Bolt ϟ is distributed in the hope that it will be useful, but without any warranty;
// without even the implied warranty of merchantability or fitness for a particular purpose.
// See your chosen license for more details.
//
// You should have received a copy of all licenses along with Bolt ϟ.
// If not, see <https://github.com/minora-oss/bolt/blob/master/license/>.

extern crate cookie;
extern crate log;
extern crate mime;
extern crate openssl;
extern crate regex;
extern crate time;
extern crate url;

pub mod version;
pub mod status;

pub mod http;
pub mod quic;
pub mod spdy;

fn main() {
    println!("Hello, world! This is Bolt ϟ!");
}
