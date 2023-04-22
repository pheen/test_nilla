mod nodes;
use nodes::*;

use std::{
    io::Read,
    iter::Enumerate,
    slice::Iter,
};

use chumsky::text::{Character, newline};

const NEWLINE: u8 = 10;
const CARRIAGE: u8 = 13;
const SPACE: u8 = 32;
const TILDE: u8 = 126;

const FUNCTIONS: &'static [fn(&mut ParserState, &Vec<u8>, usize)] = &[
    unhandled,            // 0    �	Control character: Null
    unhandled,            // 1    Control character: Start Of Heading
    unhandled,            // 2    Control character: Start Of Text
    unhandled,            // 3    Control character: End Of Text
    unhandled,            // 4    Control character: End Of Transmission
    unhandled,            // 5    Control character: Enquiry
    unhandled,            // 6    Control character: Acknowledge
    unhandled,            // 7    Control character: Bell
    unhandled,            // 8    Control character: Backspace
    unhandled,            // 9    Control character: Character Tabulation
    line_feed,            // 10   Control character: Line Feed (lf)
    unhandled,            // 11   Control character: Line Tabulation
    unhandled,            // 12   Control character: Form Feed (ff)
    carriage_return,      // 13   Control character: Carriage Return (cr)
    unhandled,            // 14   Control character: Shift Out
    unhandled,            // 15   Control character: Shift In
    unhandled,            // 16   Control character: Data Link Escape
    unhandled,            // 17   Control character: Device Control One
    unhandled,            // 18   Control character: Device Control Two
    unhandled,            // 19   Control character: Device Control Three
    unhandled,            // 20   Control character: Device Control Four
    unhandled,            // 21   Control character: Negative Acknowledge
    unhandled,            // 22   Control character: Synchronous Idle
    unhandled,            // 23   Control character: End Of Transmission Block
    unhandled,            // 24   Control character: Cancel
    unhandled,            // 25   Control character: End Of Medium
    unhandled,            // 26   Control character: Substitute
    unhandled,            // 27   Control character: Escape
    unhandled,            // 28   Control character: Information Separator Four
    unhandled,            // 29   Control character: Information Separator Three
    unhandled,            // 30   Control character: Information Separator Two
    unhandled,            // 31   Control character: Information Separator One
    space,                // 32   Space
    exclamation_mark,     // 33   !	Exclamation Mark
    quotation_mark,       // 34   "	Quotation Mark
    number_sign,          // 35   #	Number Sign
    dollar_sign,          // 36   $	Dollar Sign
    percent_sign,         // 37   %	Percent Sign
    ampersand,            // 38   &	Ampersand
    apostrophe,           // 39   '	Apostrophe
    left_parenthesis,     // 40   (	Left Parenthesis
    right_parenthesis,    // 41   )	Right Parenthesis
    asterisk,             // 42   *	Asterisk
    plus_sign,            // 43   +	Plus Sign
    comma,                // 44   ,	Comma
    hyphen_minus,         // 45   -	Hyphen-minus
    full_stop,            // 46   .	Full Stop
    solidus,              // 47   /	Solidus
    digit_zero,           // 48   0	Digit Zero
    digit_one,            // 49   1	Digit One
    digit_two,            // 50   2	Digit Two
    digit_three,          // 51   3	Digit Three
    digit_four,           // 52   4	Digit Four
    digit_five,           // 53   5	Digit Five
    digit_six,            // 54   6	Digit Six
    digit_seven,          // 55   7	Digit Seven
    digit_eight,          // 56   8	Digit Eight
    digit_nine,           // 57   9	Digit Nine
    colon,                // 58   :	Colon
    semicolon,            // 59   ;	Semicolon
    less_than_sign,       // 60   <	Less-than Sign
    equals_sign,          // 61   =	Equals Sign
    greater_than_sign,    // 62   >	Greater-than Sign
    question_mark,        // 63   ?	Question Mark
    commercial_at,        // 64   @	Commercial At
    letter_cap_a,         // 65   A	Latin Capital Letter A
    letter_cap_b,         // 66   B	Latin Capital Letter B
    letter_cap_c,         // 67   C	Latin Capital Letter C
    letter_cap_d,         // 68   D	Latin Capital Letter D
    letter_cap_e,         // 69   E	Latin Capital Letter E
    letter_cap_f,         // 70   F	Latin Capital Letter F
    letter_cap_g,         // 71   G	Latin Capital Letter G
    letter_cap_h,         // 72   H	Latin Capital Letter H
    letter_cap_i,         // 73   I	Latin Capital Letter I
    letter_cap_j,         // 74   J	Latin Capital Letter J
    letter_cap_k,         // 75   K	Latin Capital Letter K
    letter_cap_l,         // 76   L	Latin Capital Letter L
    letter_cap_m,         // 77   M	Latin Capital Letter M
    letter_cap_n,         // 78   N	Latin Capital Letter N
    letter_cap_o,         // 79   O	Latin Capital Letter O
    letter_cap_p,         // 80   P	Latin Capital Letter P
    letter_cap_q,         // 81   Q	Latin Capital Letter Q
    letter_cap_r,         // 82   R	Latin Capital Letter R
    letter_cap_s,         // 83   S	Latin Capital Letter S
    letter_cap_t,         // 84   T	Latin Capital Letter T
    letter_cap_u,         // 85   U	Latin Capital Letter U
    letter_cap_v,         // 86   V	Latin Capital Letter V
    letter_cap_w,         // 87   W	Latin Capital Letter W
    letter_cap_x,         // 88   X	Latin Capital Letter X
    letter_cap_y,         // 89   Y	Latin Capital Letter Y
    letter_cap_z,         // 90   Z	Latin Capital Letter Z
    left_square_bracket,  // 91   [	Left Square Bracket
    reverse_solidus,      // 92   \	Reverse Solidus
    right_square_bracket, // 93   ]	Right Square Bracket
    circumflex_accent,    // 94   ^	Circumflex Accent
    low_line,             // 95   _	Low Line
    grave_accent,         // 96   `	Grave Accent
    letter_a,             // 97   a	Latin Small Letter A
    letter_b,             // 98   b	Latin Small Letter B
    letter_c,             // 99   c	Latin Small Letter C
    letter_d,             // 100  d	Latin Small Letter D
    letter_e,             // 101  e	Latin Small Letter E
    letter_f,             // 102  f	Latin Small Letter F
    letter_g,             // 103  g	Latin Small Letter G
    letter_h,             // 104  h	Latin Small Letter H
    letter_i,             // 105  i	Latin Small Letter I
    letter_j,             // 106  j	Latin Small Letter J
    letter_k,             // 107  k	Latin Small Letter K
    letter_l,             // 108  l	Latin Small Letter L
    letter_m,             // 109  m	Latin Small Letter M
    letter_n,             // 110  n	Latin Small Letter N
    letter_o,             // 111  o	Latin Small Letter O
    letter_p,             // 112  p	Latin Small Letter P
    letter_q,             // 113  q	Latin Small Letter Q
    letter_r,             // 114  r	Latin Small Letter R
    letter_s,             // 115  s	Latin Small Letter S
    letter_t,             // 116  t	Latin Small Letter T
    letter_u,             // 117  u	Latin Small Letter U
    letter_v,             // 118  v	Latin Small Letter V
    letter_w,             // 119  w	Latin Small Letter W
    letter_x,             // 120  x	Latin Small Letter X
    letter_y,             // 121  y	Latin Small Letter Y
    letter_z,             // 122  z	Latin Small Letter Z
    left_curly_bracket,   // 123  {	Left Curly Bracket
    vertical_line,        // 124  |	Vertical Line
    right_curly_bracket,  // 125  }	Right Curly Bracket
    tilde,                // 126  ~	Tilde
];

#[derive(Debug)]
struct ParserState<'a> {
    context: Context,
    ast: Root<'a>,
    buffer: String,
    prev_pos: u64,
    current_node: Node<'a>,
    contents_iter: &'a mut Enumerate<Iter<'a, u8>>,
    start_of_line: bool,
}

#[derive(Debug)]
enum Context {
    Root,
    Digit,
    Whitespace,
    BinaryOp,
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let result = parse_file(file_path);

    println!("{:?}", result);
}

fn parse_file(file_path: String) -> Option<u8> {
    let mut file = std::fs::File::open(file_path).expect("able to open file");
    let mut file_contents = Vec::new();

    if let Err(error) = file.read_to_end(&mut file_contents) {
        panic!("Unable to read file {:#?}", error);
    }

    let mut contents_iter = file_contents.iter().enumerate();
    let mut parser_state = ParserState {
        ast: Root { children: vec![] },
        current_node: Node::Unknown(Unknown { source_code: "".to_string() }),

        context: Context::Root {},
        buffer: "".to_string(),
        prev_pos: 0,
        contents_iter: &mut contents_iter,
        start_of_line: true,
    };

    parse_contents(&mut parser_state, &file_contents);

    Some(0)
}

fn parse_contents(parser_state: &mut ParserState, file_contents: &Vec<u8>) {
    while let Some((pos, byte)) = parser_state.contents_iter.next() {
        if *byte != SPACE {
            parser_state.start_of_line = false;
        }

        println!("char: {:#?}", byte.to_char());

        match byte {
            &NEWLINE => line_feed(parser_state, file_contents, pos),
            &CARRIAGE => carriage_return(parser_state, file_contents, pos),
            SPACE..=TILDE => {
                let func = FUNCTIONS[*byte as usize];
                func(parser_state, file_contents, pos);
            }
            _ => unhandled(parser_state, file_contents, pos),
        }

        println!("{:#?}", parser_state.current_node);
    }
}

fn unhandled(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    println!("Unhandled");
}

fn line_feed(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    parser_state.start_of_line = true;

    // categorize current_node

    // parser_state.buffer.push_str("\n");
}

fn carriage_return(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    parser_state.start_of_line = true;
    parser_state.buffer.push_str("\n");

    // Treat \r\n as a single newline character
    if let Some(next_byte) = file_contents.get(pos + 1) {
        if *next_byte == CARRIAGE {
            parser_state.contents_iter.next();
        }
    }
}

fn space(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    // if !parser_state.start_of_line {
    //     parser_state.buffer.push_str(" ");
    // }

    match &mut parser_state.current_node {
        Node::Unknown(Unknown{ source_code: _ }) => { }
        Node::Integer(Integer{ token: _ }) => {
            parser_state.context = Context::Whitespace;
        }
        _ => {}
    }
}

fn exclamation_mark(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "!".to_string(),
            });
        }
        _ => {}
    }
}

fn quotation_mark(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "\"".to_string(),
            });
        }
        _ => {}
    }
}

fn number_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "#".to_string(),
            });
        }
        _ => {}
    }
}

fn dollar_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "$".to_string(),
            });
        }
        _ => {}
    }
}

fn percent_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "%".to_string(),
            });
        }
        _ => {}
    }
}

fn ampersand(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "&".to_string(),
            });
        }
        _ => {}
    }
}

fn apostrophe(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "'".to_string(),
            });
        }
        _ => {}
    }
}

fn left_parenthesis(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "(".to_string(),
            });
        }
        _ => {}
    }
}

fn right_parenthesis(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: ")".to_string(),
            });
        }
        _ => {}
    }
}

fn asterisk(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "*".to_string(),
            });
        }
        _ => {}
    }
}

fn plus_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Unknown(Unknown { source_code: _ }) => { panic!("Leading plus sign isn't good") }
        Node::Integer(Integer { token: _ }) => {
            match &mut parser_state.context {
                Context::Whitespace => { panic!("Whitespace in an integer") }
                Context::Digit => {
                    parser_state.context = Context::BinaryOp;

                    // todo

                    // parser_state.current_node = Node::Add(Add {
                    //     left: &parser_state.current_node,
                    //     right: &Node::Unknown(Unknown { source_code: "".to_string() }),
                    // })

                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn comma(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: ",".to_string(),
            });
        }
        _ => {}
    }
}

fn hyphen_minus(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "-".to_string(),
            });
        }
        _ => {}
    }
}

fn full_stop(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: ".".to_string(),
            });
        }
        _ => {}
    }
}

fn solidus(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "/".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_zero(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Unknown(Unknown { source_code: _ }) => {
            panic!("Leading zero");
        }
        Node::Add(Add { left, right }) => {
            panic!("Leading zero");
        }
        Node::Integer(Integer { token }) => {
            match &mut parser_state.context {
                Context::Whitespace => { panic!("Whitespace in an integer") }
                Context::Digit => { token.push_str("0") }
                _ => {}
            }
        }
        _ => {}
    }
}

fn digit_one(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Unknown(Unknown { source_code: _ }) => {
            parser_state.context = Context::Digit;
            parser_state.current_node = Node::Integer(Integer {
                token: "1".to_string(),
            });
        }
        Node::Add(Add { left, right }) => {
            // parser_state.current_node

            // if let nodes::Add(a_node) = parser_state.current_node {
            //     // right = &mut Box::new(nodes::Integer {
            //     //     token: "1".to_string(),
            //     // });
            // }

            // match parser_state.current_node {
            //     nodes::Unknown { source_code: _ } => {
            //         if let nodes::Unknown { source_code: _ } = &mut parser_state.current_node {
            //         }

            //         // parser_state.context = Context::Digit;
            //         //  = nodes::Integer {
            //         //     token: "1".to_string(),
            //         // };
            //     }
            //     // nodes::Add { left, right } => {

            //     // }
            //     // nodes::Integer { token } => {
            //     //     match &mut parser_state.context {
            //     //         Context::Whitespace => { panic!("Whitespace in an integer") }
            //     //         Context::Digit => { token.push_str("1") }
            //     //         _ => {}
            //     //     }
            //     // }
            //     _ => {}
            // }
        }
        Node::Integer(Integer { token }) => {
            match &mut parser_state.context {
                Context::Whitespace => { panic!("Whitespace in an integer") }
                Context::Digit => { token.push_str("1") }
                _ => {}
            }
        }
        _ => {}
    }
}

fn digit_two(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "2".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_three(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "3".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_four(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "4".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_five(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "5".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_six(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "6".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_seven(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "7".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_eight(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "8".to_string(),
            });
        }
        _ => {}
    }
}

fn digit_nine(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "9".to_string(),
            });
        }
        _ => {}
    }
}

fn colon(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: ":".to_string(),
            });
        }
        _ => {}
    }
}

fn semicolon(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: ";".to_string(),
            });
        }
        _ => {}
    }
}

fn less_than_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "<".to_string(),
            });
        }
        _ => {}
    }
}

fn equals_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "=".to_string(),
            });
        }
        _ => {}
    }
}

fn greater_than_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: ">".to_string(),
            });
        }
        _ => {}
    }
}

fn question_mark(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "?".to_string(),
            });
        }
        _ => {}
    }
}

fn commercial_at(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "@".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_a(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "A".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_b(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "B".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_c(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "C".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_d(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "D".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_e(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "E".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_f(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "F".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_g(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "G".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_h(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "H".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_i(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "I".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_j(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "J".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_k(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "K".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_l(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "L".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_m(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "M".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_n(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "N".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_o(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "O".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_p(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "P".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_q(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "Q".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_r(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "R".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_s(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "S".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_t(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "T".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_u(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "U".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_v(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "V".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_w(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "W".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_x(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "X".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_y(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "Y".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_cap_z(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "Z".to_string(),
            });
        }
        _ => {}
    }
}

fn left_square_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "[".to_string(),
            });
        }
        _ => {}
    }
}

fn reverse_solidus(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "\\".to_string(),
            });
        }
        _ => {}
    }
}

fn right_square_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "]".to_string(),
            });
        }
        _ => {}
    }
}

fn circumflex_accent(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "^".to_string(),
            });
        }
        _ => {}
    }
}

fn low_line(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "_".to_string(),
            });
        }
        _ => {}
    }
}

fn grave_accent(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "`".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_a(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "a".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_b(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "b".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_c(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "c".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_d(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "d".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_e(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "e".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_f(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "f".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_g(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "g".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_h(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "h".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_i(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "i".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_j(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "j".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_k(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "k".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_l(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "l".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_m(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "m".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_n(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "n".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_o(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "o".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_p(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "p".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_q(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "q".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_r(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "r".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_s(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "s".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_t(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "t".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_u(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "u".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_v(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "v".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_w(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "w".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_x(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "x".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_y(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "y".to_string(),
            });
        }
        _ => {}
    }
}

fn letter_z(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "z".to_string(),
            });
        }
        _ => {}
    }
}

fn left_curly_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "{".to_string(),
            });
        }
        _ => {}
    }
}

fn vertical_line(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "|".to_string(),
            });
        }
        _ => {}
    }
}

fn right_curly_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "}".to_string(),
            });
        }
        _ => {}
    }
}

fn tilde(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match &mut parser_state.current_node {
        Node::Root(Root{ children }) => {
            parser_state.current_node = Node::Integer(Integer{
                token: "~".to_string(),
            });
        }
        _ => {}
    }
}
