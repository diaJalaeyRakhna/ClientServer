use std::io::{self, Write};
use std::error::Error;

use h2::client;
use http::{Request, Method, Version};
use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    //~~~~~~~~~~~~~~~~~~ establish TCP connection to the server
    let tcp = TcpStream::connect("127.0.0.1:8080").await
                    .expect("\n\nFailed while trying to connect with Server.\n\n");
    let (mut http2client, connection) = client::handshake(tcp).await
                    .expect("\n\nFailed while handshaking with the Server.\n\n");

    print!("\n\nPress ENTER to send Request to Server... ");
    let mut _input = String::new();
    io::stdout().flush().ok();
    io::stdin().read_line(&mut _input).expect("\n\n");

    // create a task to run the connection
    tokio::spawn(async move {
        connection.await.expect("\n\nConnection abruptly closed by the Server!\n\n");
    });
    
    //~~~~~~~~~~~~~~~~~~ prepare HTTP request
    let request = Request::builder().version(Version::HTTP_2).method(Method::GET).uri("https://www.nadia.com//").body(())
                    .expect("\n\nFailed while constructing Request.\n\n");
    
    //~~~~~~~~~~~~~~~~~~ send request to the server
    println!("\n\nSending Request to Server: \n{:?}\n", request);
    let (response, _) = http2client.send_request(request, true)
                    .expect("\n\nFailed while sending request to Server.\n\n");

    //~~~~~~~~~~~~~~~~~~ receive server's response
    let (header, mut body) = response.await?.into_parts();
    println!("\nGot Response from Server: \n{:?}\n", header);

    let mut flow_control = body.flow_control().clone();
     while let Some(chunk) = body.data().await {
         let chunk = chunk?;
         println!("RX: {:?}", chunk);

         // release capacity back to the server
         let _ = flow_control.release_capacity(chunk.len());
    }
     
    print!("\nShutting down the client now!\nPress ENTER to exit... ");
    io::stdout().flush().ok();
    io::stdin().read_line(&mut _input).expect("\n\n");

    Ok(())
}