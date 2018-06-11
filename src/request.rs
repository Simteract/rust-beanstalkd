extern crate bufstream;

use std::net::TcpStream;
use self::bufstream::BufStream;
use std::io::{BufRead, Read, Write};
use std::str::FromStr;
use std::str::from_utf8;

use error::{BeanstalkdError, BeanstalkdResult};
use response::{Response, Status};

pub struct Request<'a> {
    stream: &'a mut BufStream<TcpStream>,
}

impl<'a> Request<'a> {
    pub fn new<'b>(stream: &'b mut BufStream<TcpStream>) -> Request {
        Request { stream: stream }
    }

    pub fn send(&mut self, message: &[u8]) -> BeanstalkdResult<Response> {
        let _ = self.stream.write(message);
        let _ = self.stream.flush();

        let mut line = String::new();
        self.stream.read_line(&mut line)?;
        let line_segments: Vec<&str> = line.trim().split(' ').collect();
        let status_str = line_segments.first().ok_or_else(|| BeanstalkdError::MissingData)?;
        let status = match *status_str {
            "OK" => Status::Ok,
            "RESERVED" => Status::Reserved,
            "RELEASED" => Status::Released,
            "INSERTED" => Status::Inserted,
            "USING" => Status::Using,
            "DELETED" => Status::Deleted,
            "WATCHING" => Status::Watching,
            "NOT_IGNORED" => Status::NotIgnored,
            _ => return Err(BeanstalkdError::InvalidStatus),
        };
        let mut data = line.clone();

        if status == Status::Ok || status == Status::Reserved {
            let segment_offset = match status {
                Status::Ok => 1,
                Status::Reserved => 2,
                _ => return Err(BeanstalkdError::InvalidStatus),
            };
            let bytes_count_str = line_segments.get(segment_offset).ok_or_else(|| BeanstalkdError::MissingData)?;
            let bytes_count = usize::from_str(*bytes_count_str)?;
            let mut tmp_vec: Vec<u8> = vec![0; bytes_count + 2]; // +2 needed for trailing line break
            let payload_utf8 = &mut tmp_vec[..];
            self.stream.read(payload_utf8)?;
            let payload_str = from_utf8(&payload_utf8)?;
            data = data + &payload_str;
        }

        Ok(Response {
            status: status,
            data: data,
        })
    }
}
