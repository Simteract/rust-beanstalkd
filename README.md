rust-beanstalkd [![Build Status](https://travis-ci.org/schickling/rust-beanstalkd.svg)](https://travis-ci.org/schickling/rust-beanstalkd)
===============

Easy-to-use beanstalkd client for Rust (IronMQ compatible)

## Install

Add this dependency to your `Cargo.toml`

```toml
beanstalkd = "*"
```

## Documentation

More documentation can be found [here](http://schickling.me/rust-beanstalkd).

## Usage

#### Producer

```rs
extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    let _ = beanstalkd.put("Hello World", 0, 0, 10000);
}
```

#### Consumer

```rs
extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    let (id, body) = beanstalkd.reserve().unwrap();
    println!("{}", body);
    let _ = beanstalkd.delete(id);
}
```

#### IronMQ example

```rs
extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let host = "mq-aws-us-east-1.iron.io";
    let token = "your token";
    let project_id = "your project id - not the name";

    let mut beanstalkd = Beanstalkd::connect(host, 11300).unwrap();
    let _ = beanstalkd.put(format!("oauth {} {}", token, project_id).as_slice(), 0, 0, 10000);
    let _ = beanstalkd.put("Hello World", 0, 0, 10000);
}
```

## License

[MIT License](http://opensource.org/licenses/MIT)
