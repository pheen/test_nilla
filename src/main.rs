use std::{
    io::Read,
    iter::Enumerate,
    slice::Iter,
};

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

struct ParserState<'a> {
    context: Context,
    ast: Node,
    buffer: String,
    current_node: Node,
    contents_iter: &'a mut Enumerate<Iter<'a, u8>>,
    start_of_line: bool,
}

enum Context {
    Root,
    Digit,
}

enum Node {
    Root { children: Vec<Node> },
    Unknown,
    Integer { token: String },
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
        context: Context::Root {},
        ast: Node::Root { children: vec![] },
        buffer: "".to_string(),
        current_node: Node::Unknown {},
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

        match byte {
            &NEWLINE => line_feed(parser_state, file_contents, pos),
            &CARRIAGE => carriage_return(parser_state, file_contents, pos),
            SPACE..=TILDE => {
                let func = FUNCTIONS[*byte as usize];
                func(parser_state, file_contents, pos);
            }
            _ => unhandled(parser_state, file_contents, pos),
        }

        println!("{:#?}", parser_state.buffer);
    }
}

fn unhandled(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    println!("Unhandled");
}

fn line_feed(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    parser_state.start_of_line = true;
    parser_state.buffer.push_str("\n");
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
    if !parser_state.start_of_line {
        parser_state.buffer.push_str(" ");
    }
}

fn exclamation_mark(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("!");
        }
        _ => {}
    }
}

fn quotation_mark(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("\"");
        }
        _ => {}
    }
}

fn number_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("#");
        }
        _ => {}
    }
}

fn dollar_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("$");
        }
        _ => {}
    }
}

fn percent_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("%");
        }
        _ => {}
    }
}

fn ampersand(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("&");
        }
        _ => {}
    }
}

fn apostrophe(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("'");
        }
        _ => {}
    }
}

fn left_parenthesis(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("(");
        }
        _ => {}
    }
}

fn right_parenthesis(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str(")");
        }
        _ => {}
    }
}

fn asterisk(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("*");
        }
        _ => {}
    }
}

fn plus_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("+");
        }
        _ => {}
    }
}

fn comma(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str(",");
        }
        _ => {}
    }
}

fn hyphen_minus(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("-");
        }
        _ => {}
    }
}

fn full_stop(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str(".");
        }
        _ => {}
    }
}

fn solidus(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("/");
        }
        _ => {}
    }
}

fn digit_zero(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Digit => {
            parser_state.buffer.push_str("0");
        }
        Context::Root => {
            panic!("Leading zero");
        }
        _ => {}
    }
}

fn digit_one(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    parser_state.buffer.push_str("1");

    match &mut parser_state.current_node {
        Node::Root { children } => {
            parser_state.current_node = Node::Integer {
                token: "1".to_string(),
            };
        }
        Node::Integer { token } => {
            token.push_str("1");
        }
        _ => {}
    }
}

fn digit_two(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("2");
        }
        _ => {}
    }
}

fn digit_three(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("3");
        }
        _ => {}
    }
}

fn digit_four(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("4");
        }
        _ => {}
    }
}

fn digit_five(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("5");
        }
        _ => {}
    }
}

fn digit_six(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("6");
        }
        _ => {}
    }
}

fn digit_seven(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("7");
        }
        _ => {}
    }
}

fn digit_eight(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("8");
        }
        _ => {}
    }
}

fn digit_nine(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("9");
        }
        _ => {}
    }
}

fn colon(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str(":");
        }
        _ => {}
    }
}

fn semicolon(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str(";");
        }
        _ => {}
    }
}

fn less_than_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("<");
        }
        _ => {}
    }
}

fn equals_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("=");
        }
        _ => {}
    }
}

fn greater_than_sign(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str(">");
        }
        _ => {}
    }
}

fn question_mark(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("?");
        }
        _ => {}
    }
}

fn commercial_at(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("@");
        }
        _ => {}
    }
}

fn letter_cap_a(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("A");
        }
        _ => {}
    }
}

fn letter_cap_b(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("B");
        }
        _ => {}
    }
}

fn letter_cap_c(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("C");
        }
        _ => {}
    }
}

fn letter_cap_d(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("D");
        }
        _ => {}
    }
}

fn letter_cap_e(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("E");
        }
        _ => {}
    }
}

fn letter_cap_f(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("F");
        }
        _ => {}
    }
}

fn letter_cap_g(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("G");
        }
        _ => {}
    }
}

fn letter_cap_h(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("H");
        }
        _ => {}
    }
}

fn letter_cap_i(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("I");
        }
        _ => {}
    }
}

fn letter_cap_j(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("J");
        }
        _ => {}
    }
}

fn letter_cap_k(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("K");
        }
        _ => {}
    }
}

fn letter_cap_l(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("L");
        }
        _ => {}
    }
}

fn letter_cap_m(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("M");
        }
        _ => {}
    }
}

fn letter_cap_n(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("N");
        }
        _ => {}
    }
}

fn letter_cap_o(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("O");
        }
        _ => {}
    }
}

fn letter_cap_p(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("P");
        }
        _ => {}
    }
}

fn letter_cap_q(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("Q");
        }
        _ => {}
    }
}

fn letter_cap_r(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("R");
        }
        _ => {}
    }
}

fn letter_cap_s(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("S");
        }
        _ => {}
    }
}

fn letter_cap_t(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("T");
        }
        _ => {}
    }
}

fn letter_cap_u(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("U");
        }
        _ => {}
    }
}

fn letter_cap_v(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("V");
        }
        _ => {}
    }
}

fn letter_cap_w(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("W");
        }
        _ => {}
    }
}

fn letter_cap_x(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("X");
        }
        _ => {}
    }
}

fn letter_cap_y(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("Y");
        }
        _ => {}
    }
}

fn letter_cap_z(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("Z");
        }
        _ => {}
    }
}

fn left_square_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("[");
        }
        _ => {}
    }
}

fn reverse_solidus(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("\\");
        }
        _ => {}
    }
}

fn right_square_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("]");
        }
        _ => {}
    }
}

fn circumflex_accent(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("^");
        }
        _ => {}
    }
}

fn low_line(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("_");
        }
        _ => {}
    }
}

fn grave_accent(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("`");
        }
        _ => {}
    }
}

fn letter_a(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("a");
        }
        _ => {}
    }
}

fn letter_b(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("b");
        }
        _ => {}
    }
}

fn letter_c(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("c");
        }
        _ => {}
    }
}

fn letter_d(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("d");
        }
        _ => {}
    }
}

fn letter_e(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("e");
        }
        _ => {}
    }
}

fn letter_f(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("f");
        }
        _ => {}
    }
}

fn letter_g(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("g");
        }
        _ => {}
    }
}

fn letter_h(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("h");
        }
        _ => {}
    }
}

fn letter_i(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("i");
        }
        _ => {}
    }
}

fn letter_j(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("j");
        }
        _ => {}
    }
}

fn letter_k(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("k");
        }
        _ => {}
    }
}

fn letter_l(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("l");
        }
        _ => {}
    }
}

fn letter_m(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("m");
        }
        _ => {}
    }
}

fn letter_n(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("n");
        }
        _ => {}
    }
}

fn letter_o(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("o");
        }
        _ => {}
    }
}

fn letter_p(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("p");
        }
        _ => {}
    }
}

fn letter_q(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("q");
        }
        _ => {}
    }
}

fn letter_r(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("r");
        }
        _ => {}
    }
}

fn letter_s(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("s");
        }
        _ => {}
    }
}

fn letter_t(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("t");
        }
        _ => {}
    }
}

fn letter_u(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("u");
        }
        _ => {}
    }
}

fn letter_v(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("v");
        }
        _ => {}
    }
}

fn letter_w(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("w");
        }
        _ => {}
    }
}

fn letter_x(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("x");
        }
        _ => {}
    }
}

fn letter_y(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("y");
        }
        _ => {}
    }
}

fn letter_z(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("z");
        }
        _ => {}
    }
}

fn left_curly_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("{");
        }
        _ => {}
    }
}

fn vertical_line(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("|");
        }
        _ => {}
    }
}

fn right_curly_bracket(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("}");
        }
        _ => {}
    }
}

fn tilde(parser_state: &mut ParserState, file_contents: &Vec<u8>, pos: usize) {
    match parser_state.context {
        Context::Root => {
            parser_state.buffer.push_str("~");
        }
        _ => {}
    }
}
