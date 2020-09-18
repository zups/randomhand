use tiny_http::{Server, Request, Response, StatusCode, Header};
use rand::distributions::{Distribution, Uniform};
use std::io::{Cursor};
use std::fs::File;
use std::path::Path;
use strum;
use strum_macros::{EnumIter, EnumString, EnumProperty};
use strum::IntoEnumIterator;
use strum::EnumProperty;
use std::str::FromStr;


#[derive(EnumIter, Debug, EnumString, EnumProperty)]
enum Cards {
    #[strum(props(number="1"))]
    cA,
    #[strum(props(number="2"))]
    sA,
    #[strum(props(number="3"))]
    hA,
    #[strum(props(number="4"))]
    dA,
    #[strum(props(number="5"))]
    cK,
    #[strum(props(number="6"))]
    sK,
    #[strum(props(number="7"))]
    hK,
    #[strum(props(number="8"))]
    dK,
    #[strum(props(number="9"))]
    cQ,
    #[strum(props(number="10"))]
    sQ,
    #[strum(props(number="11"))]
    hQ,
    #[strum(props(number="12"))]
    dQ,
    #[strum(props(number="13"))]
    cJ,
    #[strum(props(number="14"))]
    sJ,
    #[strum(props(number="15"))]
    hJ,
    #[strum(props(number="16"))]
    dJ,
    #[strum(props(number="17"))]
    cT,
    #[strum(props(number="18"))]
    sT,
    #[strum(props(number="19"))]
    hT,
    #[strum(props(number="20"))]
    dT,
    #[strum(props(number="21"))]
    c9,
    #[strum(props(number="22"))]
    s9,
    #[strum(props(number="23"))]
    h9,
    #[strum(props(number="24"))]
    d9,
    #[strum(props(number="25"))]
    c8,
    #[strum(props(number="26"))]
    s8,
    #[strum(props(number="27"))]
    h8,
    #[strum(props(number="28"))]
    d8,
    #[strum(props(number="29"))]
    c7,
    #[strum(props(number="30"))]
    s7,
    #[strum(props(number="31"))]
    h7,
    #[strum(props(number="32"))]
    d7,
    #[strum(props(number="33"))]
    c6,
    #[strum(props(number="34"))]
    s6,
    #[strum(props(number="35"))]
    h6,
    #[strum(props(number="36"))]
    d6,
    #[strum(props(number="37"))]
    c5,
    #[strum(props(number="38"))]
    s5,
    #[strum(props(number="39"))]
    h5,
    #[strum(props(number="40"))]
    d5,
    #[strum(props(number="41"))]
    c4,
    #[strum(props(number="42"))]
    s4,
    #[strum(props(number="43"))]
    h4,
    #[strum(props(number="44"))]
    d4,
    #[strum(props(number="45"))]
    c3,
    #[strum(props(number="46"))]
    s3,
    #[strum(props(number="47"))]
    h3,
    #[strum(props(number="48"))]
    d3,
    #[strum(props(number="49"))]
    c2,
    #[strum(props(number="50"))]
    s2,
    #[strum(props(number="51"))]
    h2,
    #[strum(props(number="52"))]
    d2,
    #[strum(props(number="53"))]
    Jb,
    #[strum(props(number="54"))]
    Jr,
}

fn get_file(path: &str) -> Result<File, String> {
    let file = File::open(&Path::new(&path));
    let file = match file {
        Ok(file) => file,
        Err(_error) => {
            return Err(String::from("Not found"))
        }
    };
    Ok(file)
}

fn card_response(request: Request) {
    let path = String::from(&request.url()[1..]);
    match get_file(&path) {
        Ok(file) => request.respond(Response::from_file(file)),
        Err(err) => request.respond(Response::from_string(err)),
    };
}

pub fn generate_unique_numbervector(generated_numbers: Vec<usize>, treshold_exluded: usize, size: usize) -> Vec<usize> {
    if generated_numbers.len() == size {
        return generated_numbers
    }

    let mut vec: Vec<usize> = Vec::new();
    vec.extend(&generated_numbers);
    let mut rng = rand::thread_rng();

    let cards = Uniform::new(1, treshold_exluded);
    loop {
        let cardnumber = cards.sample(&mut rng);
        if !vec.contains(&cardnumber) {
            vec.push(cardnumber);
            break
        }
    }

    generate_unique_numbervector(vec, treshold_exluded, size)
}

pub fn send_html_card_response(request: Request, cardnumbers: Vec<usize>) {
    let mut htmlstring = String::new();
    
    for number in &cardnumbers {
        let img = format!("<img src=\"cards/{}.png\"></img>", number);
        htmlstring.push_str(&img);
        htmlstring.push_str(" ");
    }


    htmlstring.push_str(&format!("<p>{}</p>", convert_number_to_cardnames(cardnumbers)));
    htmlstring.push_str("<form action=\"\" method=\"get\">");
    htmlstring.push_str("<input type=\"submit\" value=\"New hand\">");
    htmlstring.push_str("</form>");

    let data_len = htmlstring.len();

    let response = Response::new(
        StatusCode(200),
        vec![
            Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=UTF-8"[..]).unwrap()
        ],
        Cursor::new(htmlstring),
        Some(data_len),
        None,
    );
    
    request.respond(response);
}

pub fn convert_number_to_cardnames(cardnumbers: Vec<usize>) -> String {
    let mut numberstring = String::new();

    for cardnumber in cardnumbers {
        for name in Cards::iter() {
            let enumname: String = format!("{:?}", name);
            let name = Cards::from_str(&String::from(&enumname)).unwrap();
            let enumnumber: String = name.get_str("number").unwrap().to_string();
            if enumnumber.parse::<usize>().unwrap() == cardnumber {
                numberstring.push_str(&enumname.chars().rev().collect::<String>())
            }
        }
    }

    for name in Cards::iter() {
        let enumname = format!("{:?}", name);
        Cards::from_str(&String::from(enumname)).unwrap().get_str("number").unwrap();
    }

    numberstring
}

fn main() {
    let server = Server::http("0.0.0.0:8081").unwrap();    

    for request in server.incoming_requests() {
        convert_number_to_cardnames(Vec::new());
        let numberofcards = 53;
        let mut path = String::from(request.url().replace("/", ""));
        path = path.replace("?", "");

        if path.contains("cards") {
            card_response(request);
        } else { 
            println!("(Path: {}\n From: {})", request.url(), request.remote_addr());

            let howmanycards: usize = match path.parse::<usize>() {
                Ok(number) => number,
                Err(E) => 0,
            };

            if howmanycards == 0 || howmanycards >= numberofcards {
                request.respond(Response::from_string("Invalid number of cards.")); 
            } else {
                send_html_card_response(request, generate_unique_numbervector(Vec::new(), numberofcards, howmanycards));
            }
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_all_numbers() {
        let vec = generate_unique_numbervector(Vec::new(), 5, 4);
        assert!(vec.contains(&1));
        assert!(vec.contains(&2));
        assert!(vec.contains(&3));
        assert!(vec.contains(&4));
    }

    #[test]
    fn turn_cardnumber_into_string() {
        convert_number_to_cardnames(Vec::new());
    }

    #[test]
    fn doesnt_contain_number_out_of_range() {
        let vec = generate_unique_numbervector(Vec::new(), 5, 4);
        assert!(!vec.contains(&5));
        assert!(!vec.contains(&6));
    }

    #[test]
    fn doesnt_contain_same_number_twice() {
        let vec = generate_unique_numbervector(Vec::new(), 5, 4);
        let mut times = 0;
        for element in vec {
            if element == 1 {
                times += 1;
            }
        }
        assert!(times == 1);
    }
}