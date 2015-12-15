extern crate notify_rust;
fn main() {

    // here we print information about the server

    println!("server information:\n  {:?}",
             notify_rust::get_server_information());

    // and ask it for it's capabilities
    println!("\ncapabilities:\n  {:?}",
             notify_rust::get_capabilities());
}

