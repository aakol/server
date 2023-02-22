use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream }
     
};



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    
   
    //let response = http::get(&"127.0.0.1:7878");
    
   let  _ukryte_dane = "do sklepu trzeba czekać".to_string();
    
    match http_request[0].as_str()
    {
        "POST / HTTP/1.1" => 
        { 
           
            //let tresc = format!("<!DOCTYPE html><html>idzemy po paery,{ukryte_dane}</html>");
            let tresc2 = fs::read_to_string("ukryte_dane.html").expect("no nic z tego");

            let konwersja = konw(tresc2.clone(), "{{ukryte_dane}}".to_string(), "inna odpowiedz".to_string());
            let konwersja1= konw(konwersja.clone(), "{{data}}".to_string(), " nie wypisze tutaj daty".to_string());
            
            stream.write_all((format!("HTTP/1.1 200 OK \r\n Content-Type: application/xml \r\n Content-Length: {} \r\n\r\n{}",tresc2.len(), konwersja1)).as_bytes()).unwrap();
            
        }
        "GET / HTTP/1.1" =>
        {
            //let tresc = format!("<!DOCTYPE html><html>idzemy po paery,{ukryte_dane}</html>");
            let tresc2 = fs::read_to_string("ukryte_dane.html").expect("no nic z tego");

            let konwersja = konw(tresc2.clone(), "{{ukryte_dane}}".to_string(), "inna odpowiedz".to_string());
            let konwersja1= konw(konwersja.clone(), "{{data}}".to_string(), " nie wypisze tutaj daty".to_string());
            
            stream.write_all((format!("HTTP/1.1 200 OK \r\n Content-Type: application/xml \r\n Content-Length: {} \r\n\r\n{}",tresc2.len(), konwersja1)).as_bytes()).unwrap();
        }

        "GET /strona HTTP/1.1" =>
        {
            
            //let tresc = "<!DOCTYPE html><html>strona taka sobie,</html>".to_string();
            let tresc2 = fs::read_to_string("start.php").unwrap();
            
            stream.write_all((format!("HTTP/1.1 200 OK \r\n Content-Type: application/xml \r\n Content-Length: {} \r\n\r\n{}",tresc2.len(), tresc2)).as_bytes()).unwrap();
        }
        "GET /hello HTTP/1.1" => 
        { 
            
            //let tresc = "<!DOCTYPE html><html>idzemy po paery,</html>".to_string();
            let tresc2 = fs::read_to_string("hello.html").unwrap();
           
            stream.write_all((format!("HTTP/1.1 200 OK \r\n Content-Type: application/xml \r\n Content-Length: {} \r\n\r\n{}",tresc2.len(), tresc2)).as_bytes()).unwrap();
        }
        _ => 
        {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let tresc_html = fs::read_to_string("404.html").unwrap();
            let dlugosc_html = tresc_html.len();
            
            let response = format!("{status_line}\r\n Content-Length: {dlugosc_html}\r\n\r\n{tresc_html}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    
    

        
}
fn konw(a: String,b: String,c: String)->String
{
    let wynik = str::replace(&a, &b, &c);
    return  wynik;
}       