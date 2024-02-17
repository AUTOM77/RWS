pub fn print_info( device_id: &str, interface_name: &str, port_socks5: &str, user_passwd: &str) {
    let (user, passwd) = match user_passwd.split(':') {
        mut iter => (iter.next().unwrap_or(""), iter.next().unwrap_or("")),
    };

    println!("Device ID: {}", device_id);
    println!("Interface Name: {}", interface_name);
    println!("SOCKS5 Port: {}", port_socks5);
    println!("User: {}", user);
    println!("Password: {}", passwd);
}