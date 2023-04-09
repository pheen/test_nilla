static FUNCTIONS: &'static [fn(&Context, &mut String)] = &[
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
    unhandled,            // 10   Control character: Line Feed (lf)
    unhandled,            // 11   Control character: Line Tabulation
    unhandled,            // 12   Control character: Form Feed (ff)
    unhandled,            // 13   Control character: Carriage Return (cr)
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

enum Context {
    Root
}

enum Node {
    Root { children: Vec<Node> },
    Unknown,
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    // let source_code = std::fs::read_to_string(file_path).unwrap();
    let result = parse(file_path);

    println!("{:?}", result);
}

fn parse(file_path: String) -> String {
    let file = std::fs::File::open(file_path).expect("able to open file");
    let reader = std::io::BufReader::new(file);
    let mut lines = bytelines::ByteLines::new(reader);

    let mut context = Context::Root {};
    let mut ast = Node::Root { children: vec![] };

    let mut buffer = "".to_string();

    while let Some(line) = lines.next() {
        let mut start_of_line = true;

        for byte_as_index in line.unwrap() {
            // Ignore leading whitespace
            if start_of_line && *byte_as_index == 32 as u8 {
                continue;
            } else {
                start_of_line = false;
            }

            match byte_as_index {
                32..=126 => {
                    let func = FUNCTIONS[*byte_as_index as usize];
                    func(&context, &mut buffer);
                },
                _ => {
                    unhandled(&context, &mut buffer)
                },
            }

            println!("{:#?}", buffer);
        }

        buffer.push_str("\n");
    }

    "Nice!".to_string()
}

fn unhandled(_context: &Context, _buffer: &mut String) {
    println!("Unhandled");
}

fn space(context: &Context, buffer: &mut String) {
    match context {
        // Context::Root => {

        // },
        _ => {
            buffer.push_str(" ");
        },
    }
}

fn exclamation_mark(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("!");
        },
        _ => {},
    }
}

fn quotation_mark(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("\"");
        },
        _ => {},
    }
}

fn number_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("#");
        },
        _ => {},
    }
}

fn dollar_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("$");
        },
        _ => {},
    }
}

fn percent_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("%");
        },
        _ => {},
    }
}

fn ampersand(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("&");
        },
        _ => {},
    }
}

fn apostrophe(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("'");
        },
        _ => {},
    }
}

fn left_parenthesis(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("(");
        },
        _ => {},
    }
}

fn right_parenthesis(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str(")");
        },
        _ => {},
    }
}

fn asterisk(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("*");
        },
        _ => {},
    }
}

fn plus_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("+");
        },
        _ => {},
    }
}

fn comma(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str(",");
        },
        _ => {},
    }
}

fn hyphen_minus(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("-");
        },
        _ => {},
    }
}

fn full_stop(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str(".");
        },
        _ => {},
    }
}

fn solidus(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("/");
        },
        _ => {},
    }
}

fn digit_zero(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("0");
        },
        _ => {},
    }
}

fn digit_one(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("1");
        },
        _ => {},
    }
}

fn digit_two(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("2");
        },
        _ => {},
    }
}

fn digit_three(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("3");
        },
        _ => {},
    }
}

fn digit_four(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("4");
        },
        _ => {},
    }
}

fn digit_five(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("5");
        },
        _ => {},
    }
}

fn digit_six(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("6");
        },
        _ => {},
    }
}

fn digit_seven(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("7");
        },
        _ => {},
    }
}

fn digit_eight(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("8");
        },
        _ => {},
    }
}

fn digit_nine(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("9");
        },
        _ => {},
    }
}

fn colon(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str(":");
        },
        _ => {},
    }
}

fn semicolon(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str(";");
        },
        _ => {},
    }
}

fn less_than_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("<");
        },
        _ => {},
    }
}

fn equals_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("=");
        },
        _ => {},
    }
}

fn greater_than_sign(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str(">");
        },
        _ => {},
    }
}

fn question_mark(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("?");
        },
        _ => {},
    }
}

fn commercial_at(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("@");
        },
        _ => {},
    }
}

fn letter_cap_a(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("A");
        },
        _ => {},
    }
}

fn letter_cap_b(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("B");
        },
        _ => {},
    }
}

fn letter_cap_c(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("C");
        },
        _ => {},
    }
}

fn letter_cap_d(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("D");
        },
        _ => {},
    }
}

fn letter_cap_e(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("E");
        },
        _ => {},
    }
}

fn letter_cap_f(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("F");
        },
        _ => {},
    }
}

fn letter_cap_g(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("G");
        },
        _ => {},
    }
}

fn letter_cap_h(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("H");
        },
        _ => {},
    }
}

fn letter_cap_i(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("I");
        },
        _ => {},
    }
}

fn letter_cap_j(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("J");
        },
        _ => {},
    }
}

fn letter_cap_k(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("K");
        },
        _ => {},
    }
}

fn letter_cap_l(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("L");
        },
        _ => {},
    }
}

fn letter_cap_m(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("M");
        },
        _ => {},
    }
}

fn letter_cap_n(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("N");
        },
        _ => {},
    }
}

fn letter_cap_o(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("O");
        },
        _ => {},
    }
}

fn letter_cap_p(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("P");
        },
        _ => {},
    }
}

fn letter_cap_q(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("Q");
        },
        _ => {},
    }
}

fn letter_cap_r(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("R");
        },
        _ => {},
    }
}

fn letter_cap_s(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("S");
        },
        _ => {},
    }
}

fn letter_cap_t(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("T");
        },
        _ => {},
    }
}

fn letter_cap_u(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("U");
        },
        _ => {},
    }
}

fn letter_cap_v(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("V");
        },
        _ => {},
    }
}

fn letter_cap_w(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("W");
        },
        _ => {},
    }
}

fn letter_cap_x(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("X");
        },
        _ => {},
    }
}

fn letter_cap_y(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("Y");
        },
        _ => {},
    }
}

fn letter_cap_z(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("Z");
        },
        _ => {},
    }
}

fn left_square_bracket(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("[");
        },
        _ => {},
    }
}

fn reverse_solidus(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("\\");
        },
        _ => {},
    }
}

fn right_square_bracket(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("]");
        },
        _ => {},
    }
}

fn circumflex_accent(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("^");
        },
        _ => {},
    }
}

fn low_line(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("_");
        },
        _ => {},
    }
}

fn grave_accent(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("`");
        },
        _ => {},
    }
}

fn letter_a(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("a");
        },
        _ => {},
    }
}

fn letter_b(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("b");
        },
        _ => {},
    }
}

fn letter_c(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("c");
        },
        _ => {},
    }
}

fn letter_d(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("d");
        },
        _ => {},
    }
}

fn letter_e(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("e");
        },
        _ => {},
    }
}

fn letter_f(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("f");
        },
        _ => {},
    }
}

fn letter_g(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("g");
        },
        _ => {},
    }
}

fn letter_h(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("h");
        },
        _ => {},
    }
}

fn letter_i(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("i");
        },
        _ => {},
    }
}

fn letter_j(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("j");
        },
        _ => {},
    }
}

fn letter_k(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("k");
        },
        _ => {},
    }
}

fn letter_l(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("l");
        },
        _ => {},
    }
}

fn letter_m(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("m");
        },
        _ => {},
    }
}

fn letter_n(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("n");
        },
        _ => {},
    }
}

fn letter_o(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("o");
        },
        _ => {},
    }
}

fn letter_p(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("p");
        },
        _ => {},
    }
}

fn letter_q(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("q");
        },
        _ => {},
    }
}

fn letter_r(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("r");
        },
        _ => {},
    }
}

fn letter_s(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("s");
        },
        _ => {},
    }
}

fn letter_t(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("t");
        },
        _ => {},
    }
}

fn letter_u(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("u");
        },
        _ => {},
    }
}

fn letter_v(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("v");
        },
        _ => {},
    }
}

fn letter_w(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("w");
        },
        _ => {},
    }
}

fn letter_x(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("x");
        },
        _ => {},
    }
}

fn letter_y(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("y");
        },
        _ => {},
    }
}

fn letter_z(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("z");
        },
        _ => {},
    }
}

fn left_curly_bracket(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("{");
        },
        _ => {},
    }
}

fn vertical_line(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("|");
        },
        _ => {},
    }
}

fn right_curly_bracket(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("}");
        },
        _ => {},
    }
}

fn tilde(context: &Context, buffer: &mut String) {
    match context {
        Context::Root => {
            buffer.push_str("~");
        },
        _ => {},
    }
}
