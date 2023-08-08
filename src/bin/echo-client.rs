use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()>{
    let socket = TcpStream::connect("127.0.0.1:6245").await?;
    let(mut rd, mut wt) = io::split(socket);

    tokio::spawn(async move{
        wt.write_all(b" hello\r \n").await;
        wt.write_all(b" world \r \n").await;

        Ok::<_, io::Error>(())
    });

    let mut buff = vec![0;120];

    loop{
        let n = rd.read(&mut buff).await?;

        if n == 0{
            break;
        }

        println!("{:?}", &buff[..n]);
    }

    Ok(())

}