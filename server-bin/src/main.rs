use tokio::net::UdpSocket;
use std::error::Error;

const UDP_BIND_ADDR: &str = "0.0.0.0:50120";
const NOTIFY_BROADCAST_ADDR: &str = "10.15.200.255:50121";

#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error>>
{

}

async fn broadcast_notification() -> Result<(), Box<dyn Error>> 
{
    let socket = UdpSocket::bind(UDP_BIND_ADDR).await?;

    socket.set_broadcast(true)?;

    let mensagem = b"Hello via Tokio!";
    let destino = NOTIFY_BROADCAST_ADDR;

    socket.send_to(mensagem, destino).await?;
    println!("Datagrama assíncrono enviado para {}", destino);
    Ok(())
}

