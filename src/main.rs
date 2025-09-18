use std::net::UdpSocket;



fn parse_header(buf: &[u8]) {
    let id = ((buf[0] as u16) << 8) | (buf[1] as u16);
    let flags = ((buf[2] as u16) << 8) | (buf[3] as u16);
    let qdcount = ((buf[4] as u16) << 8) | (buf[5] as u16);
    let ancount = ((buf[6] as u16) << 8) | (buf[7] as u16);
    let nscount = ((buf[8] as u16) << 8) | (buf[9] as u16);
    let arcount = ((buf[10] as u16) << 8) | (buf[11] as u16);

     println!("ID={id}, Flags={flags}, QD={qdcount}, AN={ancount}, NS={nscount}, AR={arcount}");
}

fn qname(buf: &[u8]) -> usize{
    let mut qname = String::new();
    let mut i = 12;

    loop {
        let len = buf[i] as usize;
        if len == 0 {
            break;
        }
        i += 1;


        let label = &buf[i..i+len];
        qname.push_str(std::str::from_utf8(label).unwrap());
        qname.push('.');

        i += len;
    }


    if qname.ends_with('.') {
        qname.pop();
    }

    println!("Dominio richiesto: {}", qname);
    i
}

fn qtype(buf: &[u8], pos: usize) {
    let qtype = ((buf[pos + 1] as u16) << 8) | (buf[pos + 2] as u16);

    match qtype{
        1 => println!("{:?} => A",qtype),
        2 => println!("{:?} => CS",qtype),
        5 => println!("{:?} => CNAME",qtype),
        6 => println!("{:?} => SOA",qtype),
        12 => println!("{:?} => PTR",qtype),
        15 => println!("{:?} => MX",qtype),
        16 => println!("{:?} => TXT",qtype),
        28 => println!("{:?} => AAAA",qtype),
        33 => println!("{:?} => SRV",qtype),
        255 => println!("{:?} => ANY",qtype),
        _ => println!("Errore"),
    }

}

/*
1 = IN (Internet)
3 = CH (Chaos)
4 = HS (Hesiod)

*/
fn qclass(buf: &[u8], pos: usize) {
   let qclass = ((buf[pos + 3] as u16) << 8) | (buf[pos + 4] as u16);

    match qclass{
        1 => println!("{:?} => IN",qclass),
        3 => println!("{:?} => CH",qclass),
        4 => println!("{:?} => HS",qclass),
        _ => println!("Errore"),
    }
}

fn main() -> std::io::Result<()> {

    {
        let socket = UdpSocket::bind("127.0.0.1:53")?;

        let mut buf = [0u8;512];

        loop{
            let (amt, src) = socket.recv_from(&mut buf)?;

            println!("{amt} byte -> {src}");
            parse_header(&buf);

            let i = qname(&buf);
            qtype(&buf, i);
            qclass(&buf, i);
        }
    }
}
