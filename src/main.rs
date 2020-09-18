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
use std::io::BufReader;
use std::io::prelude::*;
use std::str;
use itertools::Itertools;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref FI15: Vec<String> = read_ranges_into_vector("hands/fi15.txt");
    static ref FI20: Vec<String> = read_ranges_into_vector("hands/fi20.txt");
    static ref FI25: Vec<String> = read_ranges_into_vector("hands/fi25.txt");
    static ref FI30: Vec<String> = read_ranges_into_vector("hands/fi30.txt");
    static ref FI40: Vec<String> = read_ranges_into_vector("hands/fi40.txt");
}

enum Range {
    FI15,
    FI20,
    FI25,
    FI40,
    FI30,
}

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

fn read_ranges_into_vector(rangename: &str) -> Vec<String> {
    let file = get_file(rangename).unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
  
    let vector: Vec<String> = contents.split_ascii_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

    vector
}

fn is_hand_in_range(handpermutations: &Vec<String>, range: Range) -> bool {
    match range {
        Range::FI15 => handpermutations.iter().filter(|hand| FI15.contains(hand)).count() > 0,
        Range::FI20 => handpermutations.iter().filter(|hand| FI20.contains(hand)).count() > 0,
        Range::FI25 => handpermutations.iter().filter(|hand| FI25.contains(hand)).count() > 0,
        Range::FI30 => handpermutations.iter().filter(|hand| FI30.contains(hand)).count() > 0,
        Range::FI40 => handpermutations.iter().filter(|hand| FI40.contains(hand)).count() > 0,
    }
}

fn handpermutations(hand: &str) -> Vec<String> {
    let cards = hand.as_bytes()
    .chunks(2)
    .map(str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap();

    let mut hands = Vec::new();

    for hand in cards.into_iter().permutations(4) {
        hands.push(hand.into_iter().map(|i| i.to_string()).collect::<String>());
    }

    hands
}

fn true_false_to_open_fold(binary: bool) -> String {
    match binary {
        true => String::from("<b><font color=\"green\" size=\"+1\">open</font></b>"),
        false => String::from("<b><font color=\"red\" size=\"+1\">fold</font></b>"),
    }
} 

pub fn send_html_card_response(request: Request, cardnumbers: Vec<usize>) {
    let mut htmlstring = String::new();
    
    for number in &cardnumbers {
        let img = format!("<img src=\"cards/{}.png\"></img>", number);
        htmlstring.push_str(&img);
        htmlstring.push_str(" ");
    }

    let hand = convert_number_to_cardnames(cardnumbers);
    let hands = handpermutations(&hand);

    htmlstring.push_str(&format!("<p><b>UTG</b><font size=\"2\">(15%)</font><b>:</b> {}</p>",true_false_to_open_fold(is_hand_in_range(&hands, Range::FI15))));
    htmlstring.push_str(&format!("<p><b>MP</b><font size=\"2\">(20%)</font><b>:</b> {}</p>", true_false_to_open_fold(is_hand_in_range(&hands, Range::FI20))));
    htmlstring.push_str(&format!("<p><b>CO</b><font size=\"2\">(25%)</font><b>:</b> {}</p>", true_false_to_open_fold(is_hand_in_range(&hands, Range::FI25))));
    htmlstring.push_str(&format!("<p><b>BU</b><font size=\"2\">(40%)</font><b>:</b> {}</p>", true_false_to_open_fold(is_hand_in_range(&hands, Range::FI40))));
    htmlstring.push_str(&format!("<p><b>SB</b><font size=\"2\">(30%)</font><b>:</b> {}</p>", true_false_to_open_fold(is_hand_in_range(&hands, Range::FI30))));


    htmlstring.push_str(&format!("<p>{}</p>", &hand));
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

            let allowcardparameter = false;

            if allowcardparameter {
                let howmanycards: usize = match path.parse::<usize>() {
                Ok(number) => number,
                Err(E) => 0,
            };
            
            if howmanycards == 0 || howmanycards >= numberofcards {
                request.respond(Response::from_string("Invalid number of cards.")); 
            } else {
                send_html_card_response(request, generate_unique_numbervector(Vec::new(), numberofcards, howmanycards));
            }
        } else {
             send_html_card_response(request, generate_unique_numbervector(Vec::new(), numberofcards, 4));
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