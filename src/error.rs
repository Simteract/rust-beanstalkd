use std::fmt::{Formatter,Display,Error};

pub enum BeanstalkdError { ConnectionError, RequestError }

impl Display for BeanstalkdError {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
