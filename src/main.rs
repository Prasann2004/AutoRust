use std::fmt;

fn main() {
    let constant: Var = Var {
        name: "x".to_string(),
        value: 0,
    };
    let var: Var = Var {
        name: "y".to_string(),
        value: 2,
    };
    let sum: Sum = Sum {
        x: Node::Var(constant),
        y: Node::Var(var.clone()),
    };
    let mul : Mul = Mul{
        x: Node ::Var(var.clone()),
        y: Node::Sum(Box::new(sum.clone()))
    };
    println!("Differentiation: {}", mul.backward(&Node::Var(var)));
}

// constant node
#[derive(Clone)]
struct Const {
    value: i128,
}

impl Const {
    fn backward(&self) -> Const {
        Const { value: 0 }
    }

    fn compute(&self) -> i128 {
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
struct Var {
    name: String,
    value: i128,
}

impl Var {
    fn backward(&self, var: &Var) -> Const {
        if var == self {
            Const { value: 1 }
        } else {
            Const { value: 0 }
        }
    }

    fn compute(&self) -> i128 {
        self.value
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
enum Node {
    Sum(Box<Sum>),
    Mul(Box<Mul>),
    Var(Var),
    Const(Const),
}

impl Node {
    fn backward(&self, var: &Node) -> Node {
        match self {
            Node::Var(v) => Node::Const(v.backward(match var {
                Node::Var(v) => v,
                _ => return Node::Const(Const { value: 0 }),
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

    fn compute(&self) -> i128 {
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
struct Sum {
    x: Node,
    y: Node,
}

impl Sum {
    fn backward(&self, var: &Node) -> Sum {
        Sum {
            x: self.x.backward(var),
            y: self.y.backward(var),
        }
    }

    fn compute(&self) -> i128 {
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
struct Mul {
    x: Node,
    y: Node,
}

impl Mul {
    fn backward(&self, var: &Node) -> Sum {
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

    fn compute(&self) -> i128 {
        self.x.compute() * self.y.compute()
    }
}

impl fmt::Display for Mul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}*{})", self.x, self.y)
    }
}
