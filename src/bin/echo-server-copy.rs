use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main()-> io::Result<()>{
    let listner = TcpListener::bind("127.0.0.1:6245").await?;

    loop{
        let (mut socket, _) = listner.accept().await?;

        tokio::spawn(async move{
            let mut buff = vec![0; 1024];
            loop{
                match socket.read(&mut buff).await{
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buff[..n]).await.is_err(){
                            return
                        }
                    } ,
                    Err(_) =>{
                        return ;
                    }
                }
            }

        });
    }

}