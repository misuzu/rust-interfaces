fn main() {
    match ifaces::ifaces() {
        Ok(interfaces) => {
            for interface in interfaces.into_iter() {
                println!("Found Interface: {:?}", interface)
            }
        },
        Err(_) => println!("Ooops ...")
    };
}
