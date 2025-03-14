pub mod autodiff;
use autodiff::{Node, Var, Sum, Mul, Const};
use std::fmt;
use rand::Rng;
pub struct SingleLinearRegression {
    pub x: Const,
    pub y: Const,
    pub w: Var,
    pub b: Var,
    pub eq : Node,
}

impl SingleLinearRegression {
    pub fn initialize(&mut self) {
        let mut rng = rand::rng();
        self.w = Var {
            name: "w".to_string(),
            value:  rng.random_range(0.0..1.0),
        };
        self.b = Var {
            name: "b".to_string(),
            value: rng.random_range(0.0..1.0),
        };
    }
    pub fn fit(&mut self, x:&[f64], y:&[f64], epochs: i32, lr: f64) {
        self.initialize();
        for i in 0..epochs{
            for j in 0..x.len(){
                self.x = Const {
                    value: x[j],
                };
                self.y = Const {
                    value: y[j],
                };
                self.eq = Node::Sum(Box::new(Sum{
                    x : Node::Mul(Box::new(Mul{
                        x: Node::Var(self.w.clone()),
                        y: Node::Const(self.x.clone()),
                    })),
                    y: Node::Var(self.b.clone()),
                }));
                let mut error = Mul{
                    x:Node::Sum(Box::new(Sum{
                        x: Node::Mul(Box::new(Mul{
                            x: Node::Const(self.y.clone()),
                            y: Node::Const(Const{value: -1.0}),
                        })),
                        y: self.eq.clone(),
                    })),
                    y:Node::Sum(Box::new(Sum{
                        x: Node::Mul(Box::new(Mul{
                            x: Node::Const(self.y.clone()),
                            y: Node::Const(Const{value: -1.0}),
                        })),
                        y: self.eq.clone(),
                    })),
                };
                let grad_w = error.backward(&Node::Var(self.w.clone()));
                let grad_b = error.backward(&Node::Var(self.b.clone()));
                self.w.value -= lr * grad_w.compute();
                self.b.value -= lr * grad_b.compute();
            }
            println!("Epoch: {}, w: {}, b: {}", i, self.w.value, self.b.value);
        }
    }
    pub fn predict(&self, x: f64) -> f64 {
        self.w.value * x + self.b.value
    }
}