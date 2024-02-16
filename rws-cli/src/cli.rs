use clap::Parser;

/// Simple program to interact with rws-cli
#[derive(Parser, Debug)]
struct Args {
    #[clap(short='d', long, name = "DEVICE_ID")]
    device_id: String,

    #[clap(short='i', long, name = "INTERFACE_NAME")]
    interface_name: String,

    #[clap(short='p', long, name = "SOCKS5_PORT")]
    port_socks5: String,

    #[clap(short='U', long, name = "USER_PASSWD")]
    user_passwd: String,
}

fn main() {
    let args = Args::parse();

    // Access the parsed command-line arguments
    let device_id = &args.device_id;
    let interface_name = &args.interface_name;
    let socks5_port = &args.port_socks5;
    let user_passwd_parts: Vec<&str> = args.user_passwd.split(':').collect();
    let (user, passwd) = match user_passwd_parts.as_slice() {
        [user, passwd] => (user, passwd),
        _ => (&"", &""),
    };

    println!("Device ID: {}", device_id);
    println!("Interface Name: {}", interface_name);
    println!("SOCKS5 Port: {}", socks5_port);
    println!("User: {}", user);
    println!("Password: {}", passwd);
}
