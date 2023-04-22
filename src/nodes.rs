// #[derive(Clone, Debug)]
#[derive(Debug)]
pub enum Node<'a> {
    Root(Root<'a>),
    Unknown(Unknown),

    Integer(Integer),
    Var(Var),

    Add(Add<'a>),
    Subtract(Subtract<'a>),
    Mult(Mult<'a>),
    Div(Div<'a>),
    Negate(Negate<'a>),
}

// pub trait Node: std::fmt::Debug {
// pub trait NodeFns {
//   fn eval(&self) -> i64;
//   fn print(&self);
// }

impl Node<'_> {
    pub fn eval(&self) -> i64 {
        match self {
            Node::Root(Root{ children }) => { children.iter().map(|child| child.eval()).sum() },
            Node::Unknown(Unknown{ source_code })  => { source_code; 0 },

            Node::Integer(Integer{ token }) => { token.parse::<i64>().unwrap() },
            Node::Var(Var{ token }) => { token; 0 },

            Node::Add(Add{ left, right }) => { left.eval() + right.eval() },
            Node::Subtract(Subtract{ left, right }) => { left.eval() - right.eval() },
            Node::Mult(Mult{ left, right }) => { left.eval() * right.eval() },
            Node::Div(Div{ left, right }) => { left.eval() / right.eval() },
            Node::Negate(Negate{ arg }) => { -(arg.eval()) },
        }
    }

    pub fn print(&self) {
        match self {
            Node::Root(Root{ children }) => { children.iter().for_each(|child| child.print()); },
            Node::Unknown(Unknown{ source_code })  => { println!("{:#?}", source_code); },

            Node::Integer(Integer{ token }) => { println!("{:#?}", token); },
            Node::Var(Var{ token }) => { println!("{:#?}", token); },

            Node::Add(Add{ left, right }) => { println!("({:#?} * {:#?})", left.print(), right.print()); },
            Node::Subtract(Subtract{ left, right }) => { println!("({:#?} * {:#?})", left.print(), right.print()); },
            Node::Mult(Mult{ left, right }) => { println!("({:#?} * {:#?})", left.print(), right.print()); },
            Node::Div(Div{ left, right }) => { println!("({:#?} * {:#?})", left.print(), right.print()); },
            Node::Negate(Negate{ arg }) => { println!("{:#?}", arg.print()); },
        }
    }
}

// impl Debug for dyn Node {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "Series{{{}}}", self.len())
//     }
// }

#[derive(Debug)]
pub struct Root<'a> {
  pub children: Vec<Node<'a>>
}
impl Root<'_> {
  fn eval(&self) -> i64 { self.children.iter().map(|child| child.eval()).sum() }
  fn print(&self) { self.children.iter().for_each(|child| child.print()); }
}

#[derive(Debug)]
pub struct Unknown {
  pub source_code: String
}
impl Unknown {
  fn eval(&self) -> i64 { self.source_code.clone(); 0 }
  fn print(&self) { println!("{:#?}", self.source_code); }
}

#[derive(Debug)]
pub struct Integer {
  pub token: String
}
impl Integer {
  fn eval(&self) -> i64 { self.token.parse::<i64>().unwrap() }
  fn print(&self) { println!("{:#?}", self.token); }
}

#[derive(Debug)]
pub struct Var {
  pub token: String
}
impl Var {
  fn eval(&self) -> i64 { self.token.clone(); 0 }
  fn print(&self) { println!("{:#?}", self.token); }
}

#[derive(Debug)]
pub struct Add<'a> {
  pub left: &'a Node<'a>,
  pub right: &'a Node<'a>,
}
impl Add<'_> {
  fn eval(&self) -> i64 { self.left.eval() + self.right.eval() }
  fn print(&self) { println!("({:#?} * {:#?})", self.left.print(), self.right.print()); }
}

#[derive(Debug)]
pub struct Subtract<'a> {
  pub left: &'a Node<'a>,
  pub right: &'a Node<'a>,
}
impl Subtract<'_> {
  fn eval(&self) -> i64 { self.left.eval() - self.right.eval() }
  fn print(&self) { println!("({:#?} * {:#?})", self.left.print(), self.right.print()); }
}

#[derive(Debug)]
pub struct Mult<'a> {
  pub left: &'a Node<'a>,
  pub right: &'a Node<'a>,
}
impl Mult<'_> {
  fn eval(&self) -> i64 { self.left.eval() * self.right.eval() }
  fn print(&self) { println!("({:#?} * {:#?})", self.left.print(), self.right.print()); }
}

#[derive(Debug)]
pub struct Div<'a> {
  pub left: &'a Node<'a>,
  pub right: &'a Node<'a>,
}
impl Div<'_> {
  fn eval(&self) -> i64 { self.left.eval() / self.right.eval() }
  fn print(&self) { println!("({:#?} * {:#?})", self.left.print(), self.right.print()); }
}

#[derive(Debug)]
pub struct Negate<'a> {
  pub arg: &'a Node<'a>,
}
impl Negate<'_> {
  fn eval(&self) -> i64 { -(self.arg.eval()) }
  fn print(&self) { println!("{:#?}", self.arg.print()); }
}
