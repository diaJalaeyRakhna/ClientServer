// Author: Nadia Sheikh, PIAIc 52062
// Batch 3, Q3, Islamabad
// Pr. Scientist, PAEC

use std::io::{self, Write};
use h2::server;
use http::{Response, StatusCode, Version};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await
                    .expect("\n\nFailed while trying to register for IP:127.0.0.1, Port:8080\n\n");

    // accept all incoming TCP connections
    println!("Server is listening for client requests on {:?} : {:?}\n", listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());

    loop {
        if let Ok((socket, peer_addr)) = listener.accept().await {
            tokio::spawn(async move { // spawn a new task to process each request
                // start HTTP/2.0 connection handshake
                let mut connection = server::handshake(socket).await
                                        .expect("\n\nFailed while handshaking with the Client.\n\n");
                println!("\n!!!!!!!!!!! HTTP/2.0 Connection is OPEN with Client {:?} !!!!!!!!!!!", peer_addr.to_string());
                
                // accept all inbound HTTP/2.0 streams sent over the connection
                while let Some(request) = connection.accept().await {
                    let (request, mut respond) = request.expect("\n\nConnection abruptly closed by remote client!\n\n");
                    println!("\nGot Request from Client: \n{:?}\n", request);
                    
                    print!("\n\nPress ENTER to send Response to the same Client... ");
                    let mut _input = String::new();
                    io::stdout().flush().ok();
                    io::stdin().read_line(&mut _input).expect("\n\n");
                    
                    // build a response
                    let response = Response::builder().version(Version::HTTP_2).status(StatusCode::OK).body(())
                                    .expect("\n\nFailed while constructing Response.\n\n");
                    
                    // send the response back to the client
                    println!("\n\nSending Response to Client: \n{:?}\n", response);
                    respond.send_response(response, true).expect("\n\nFailed while sending response to Client.\n\n");
                }
                
                println!("\n~~~~~~~~~~~ HTTP/2.0 Connection with Client {:?} is CLOSED now !!!!!! ~~~~~~~~~~~\n\n", peer_addr.to_string());
            });
        }
    }
}