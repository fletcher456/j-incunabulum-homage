use actix_files as fs;
use actix_web::{App, HttpServer};
use std::io;

fn main() -> io::Result<()> {
    println!("Starting server at http://0.0.0.0:5000");
    println!("Visit http://0.0.0.0:5000/hello_world.html in your browser");

    // Initialize the system
    let sys = actix_web::rt::System::new();
    
    // Start the server
    sys.block_on(async {
        HttpServer::new(|| {
            App::new()
                .service(fs::Files::new("/", "./static").index_file("hello_world.html"))
        })
        .bind("0.0.0.0:5000")?
        .run()
        .await
    })
}
