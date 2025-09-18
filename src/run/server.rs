use std::net::UdpSocket;




pub fn server_start() -> std::io::Result<()>{
    {
        let socket = UdpSocket::bind("127.0.0.1:53");

        let mut buf = [0,10];

        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("{src:?}");
    }
    Ok(())
}
