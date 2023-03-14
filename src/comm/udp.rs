// #[tokio::main]
#[inline(always)]
pub async fn robocup_udp() -> std::io::Result<()> {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:8080").await?;
    let gcip = std::env::var("GAMECONTROLLER_IP").expect("`GAMECONTROLLER_IP` undefined");
    socket.connect(gcip).await?;
    let mut buf = [0; 1024];
    loop {
        let len = socket.recv(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, remote_addr);

        let len = socket.send(&buf[..len]).await?;
        println!("{:?} bytes sent", len);
    }
}
