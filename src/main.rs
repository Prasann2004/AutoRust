mod linear_regression;
use linear_regression::SingleLinearRegression;
use linear_regression::autodiff::{Node, Var, Sum, Mul, Const};

fn main() {
    // let constant: Var = Var {
    //     name: "x".to_string(),
    //     value: 0,
    // };
    // let var: Var = Var {
    //     name: "y".to_string(),
    //     value: 2,
    // };
    // let sum: Sum = Sum {
    //     x: Node::Var(constant),
    //     y: Node::Var(var.clone()),
    // };
    // let mul : Mul = Mul{
    //     x: Node ::Var(var.clone()),
    //     y: Node::Sum(Box::new(sum.clone()))
    // };
    // println!("Differentiation: {}", mul.backward(&Node::Var(var)));
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [2.0, 4.0, 6.0, 8.0, 10.0];
    let mut sing_linear_reg = SingleLinearRegression {
        x: Const { value: 0.0 },
        y: Const { value: 0.0 },
        w: Var { name: "w".to_string(), value: 0.0 },
        b: Var { name: "b".to_string(), value: 0.0 },
        eq: Node::Sum(Box::new(Sum {
            x: Node::Mul(Box::new(Mul {
                x: Node::Var(Var { name: "w".to_string(), value: 0.0 }),
                y: Node::Const(Const { value: 0.0 }),
            })),
            y: Node::Var(Var { name: "b".to_string(), value: 0.0 }),
        })),
    };
    sing_linear_reg.fit(&x, &y, 100, 0.5);
    println!("w: {}, b: {}", sing_linear_reg.w.value, sing_linear_reg.b.value);
    print!("Sample prediction: {} \n", sing_linear_reg.predict(5.0));

}