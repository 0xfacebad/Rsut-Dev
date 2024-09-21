#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::net::{TcpListener, TcpStream};
use std::io::{stdin, BufRead, BufReader, Error, Write};
use std::{env, str, thread};
use std::fmt;

#[derive(Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

impl Point3D {
    fn from_csv(data: &str) -> Result<Self, String> {
        let parts: Vec<&str> = data.split(',').collect();
        if parts.len() != 3 {
            return Err("Invalid number of values".into());
        }

        let x = parts[0].parse::<u32>().map_err(|_| "Invalid x value")?;
        let y = parts[1].parse::<u32>().map_err(|_| "Invalid y value")?;
        let z = parts[2].parse::<u32>().map_err(|_| "Invalid z value")?;

        Ok(Point3D { x, y, z })
    }
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);

    loop {
        data.clear();
        let read_bytes = stream.read_until(b'\n', &mut data)?;
        if read_bytes == 0 {
            return Ok(());
        }

        // Convert data to string and trim whitespace/newline characters
        let data_str = str::from_utf8(&data).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?.trim();

        // Debugging output to inspect received data
        println!("Received data: {:?}", data_str);

        // Parse the CSV data into Point3D
        match Point3D::from_csv(data_str) {
            Ok(input) => {
                let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
            
                writeln!(stream.get_mut(), "Response from server: {}", f64::from(value).sqrt())?;
            }
            Err(e) => {
                eprintln!("Failed to parse data: {}", e);
                // Optionally, send an error response back to the client
            }
        }
    }
}


fn main() {
   let args:Vec<_> = env::args().collect();
   if args.len() != 2 {
   	     eprintln!("Please provide an args --client / --server");
   	     std::process::exit(1);
   }  
   if args[1] == "--server" {
       let listener = TcpListener::bind("0.0.0.0:8000")
                .expect("could not bind the socket");
            for stream in listener.incoming(){
                  match stream {
                  	     Err(e)=> eprintln!("Failed stream {}", e),
                  	     Ok(stream)=> {
                  	     	   thread::spawn(move|| {
                  	     	   	  handle_client(stream).unwrap_or_else(|error|
                  	     	   	  	 eprintln!("I was called {:?}" ,error));
                  	     	   });
                  	     }
                  }
            }
   }else if args[1] == "--client" {
             let mut stream = TcpStream::connect("127.0.0.0:8000")
                     .expect("Failed to connect");
             println!("Please enter the three points seperated with commas");
             loop {
                       let mut input = String::new();
                       let mut buffer:Vec<u8> = Vec::new();
                       stdin().read_line(&mut input)
                       .expect("Failed to get input in stdin");
                       let part: Vec<&str> = input.trim_matches('\n')
                           .split(",").collect();
                       let point = Point3D{
                                x: part[0].parse().unwrap(),
                                y: part[1].parse().unwrap(),
                                z: part[2].parse().unwrap(),
                         };
                         stream.write_all(serde_json::to_string(&point)
                            .unwrap().as_bytes())
                            .expect("Failed to write the bytes to server");
                        let mut reader = BufReader::new(&stream);
                         reader.read_until(b'\n' , &mut buffer);
                         let input = str::from_utf8(&buffer).
                         expect("Could not write buffer string");
                         if input == " "{
                            eprintln!("Empty no string found");
                         }
                         println!("Response from server {}",input);
                  }  
      }

}
