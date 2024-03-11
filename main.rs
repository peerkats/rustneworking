
    stream.read(&mut buffer).expect("kanker");


    let request = String::from_utf8_lossy(&buffer[..]);

    println!("{}", request);
    let response = "hello".as_bytes();
    stream.write(response).expect("kanker");






}



fn main(){
    let listener = TcpListener::bind("192.168.147.128:8080").expect("failed to bind addres");

    println!("server listing on port 8080");



    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("kkr zooi, {}", e);
            }
        }
    }




}
