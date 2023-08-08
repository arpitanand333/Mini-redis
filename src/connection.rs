use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, BufWriter};
use bytes::Buf;
use mini_redis::Result;
use mini_redis::Frame;
use mini_redis::frame::Error::Incomplete;
use std::io::Cursor;

struct Connection{
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection{
    pub fn new(stream: TcpStream) -> self{
        Connection{
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capasity(4096),
        }

    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>>{
        loop{
            if let Some(frame) = self.parse_frame()?{
                Ok(Some(frame))
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await?{
                if self.buffer.is_empty(){
                    return Ok(None);
                }else{
                    return Err("Connection ended by peer".into());
                }
                
            }
        }
    }

    pub fn parse_frame(&mut self)-> Result<Option<Frame>>{
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf){
            Ok(_) =>{
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))

            }
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    async fn write_frame(&mut self, frame: &Frame) -> io::Result<()>{
        match frame{
            Frame::Simple(val) =>{}
            Frame::Integral(val)=> {}
            Frame::Null =>{}
            Frame::Bulk => {}
            Frame::Array => {}
        }

        self.stream.flush().await;
        Ok(())

    }

}



