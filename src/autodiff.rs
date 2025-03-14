use std::fmt;
#[derive(Clone)]
pub struct Const {
    pub value: f64,
}

impl Const {
    pub fn backward(&self) -> Const {
        Const { value: 0.0 }
    }

    pub fn compute(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Var node
#[derive(PartialEq, Clone)]
pub struct Var {
    pub name: String,
    pub value: f64,
}

impl Var {
    pub fn backward(&self, var: &Var) -> Const {
        if var == self {
            Const { value: 1.0 }
        } else {
            Const { value: 0.0 }
        }
    }

    pub fn compute(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub enum Node {
    Sum(Box<Sum>),
    Mul(Box<Mul>),
    Var(Var),
    Const(Const),
}

impl Node {
    pub fn backward(&self, var: &Node) -> Node {
        match self {
            Node::Var(v) => Node::Const(v.backward(match var {
                Node::Var(v) => v,
                _ => return Node::Const(Const { value: 0.0 }),
            })),
            Node::Const(c) => Node::Const(c.backward()),
            Node::Mul(m) => Node::Mul(Box::new(Mul {
                x: m.x.backward(var),
                y: m.y.backward(var),
            })),
            Node::Sum(s) => Node::Sum(Box::new(Sum {
                x: s.x.backward(var),
                y: s.y.backward(var),
            })),
        }
    }

    pub fn compute(&self) -> f64 {
        match self {
            Node::Var(v) => v.compute(),
            Node::Const(c) => c.compute(),
            Node::Mul(m) => m.compute(),
            Node::Sum(s) => s.compute(),
        }
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Var(v) => write!(f, "{}", v.name),
            Node::Const(c) => write!(f, "{}", c.value),
            Node::Mul(m) => write!(f, "({}*{})", m.x, m.y),
            Node::Sum(s) => write!(f, "({}+{})",s.x,s.y),
        }
    }
}

// Sum
#[derive(Clone)]
pub struct Sum {
    pub x: Node,
    pub y: Node,
}

impl Sum {
    pub fn backward(&self, var: &Node) -> Sum {
        Sum {
            x: self.x.backward(var),
            y: self.y.backward(var),
        }
    }

    pub fn compute(&self) -> f64 {
        self.x.compute() + self.y.compute()
    }
}

impl fmt::Display for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}+{})", self.x, self.y)
    }
}

// Mul
#[derive(Clone)]
pub struct Mul {
    pub x: Node,
    pub y: Node,
}

impl Mul {
    pub fn backward(&self, var: &Node) -> Sum {
        Sum {
            x: Node::Mul(Box::new(Mul {
                x: self.x.clone().backward(var),
                y: self.y.clone(),
            })),
            y: Node::Mul(Box::new(Mul {
                x: self.x.clone(),
                y: self.y.backward(var),
            })),
        }
    }

    pub fn compute(&self) -> f64 {
        self.x.compute() * self.y.compute()
    }
}

impl fmt::Display for Mul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}*{})", self.x, self.y)
    }
}