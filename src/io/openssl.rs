extern crate openssl;

use std::error::Error;

use self::openssl::ssl::{SslContext, SslStream};
use io::{StreamWrapper, Stream, NegotiateSsl};

impl StreamWrapper for SslStream<Stream> {
    fn get_ref(&self) -> &Stream {
        self.get_ref()
    }

    fn get_mut(&mut self) -> &mut Stream {
        self.get_mut()
    }
}

impl NegotiateSsl for SslContext {
    fn negotiate_ssl(&self, _: &str, stream: Stream)
                     -> Result<Box<StreamWrapper>, Box<Error+Send+Sync>> {
        let stream = try!(SslStream::connect(self, stream));
        Ok(Box::new(stream))
    }
}
