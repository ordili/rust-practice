use expr_eval::expr::Expr;

fn main() {
    let src = "92 + 5 + 5 * 27 - (92 - 12) / 4 + 26";
    let mut expr = Expr::new(src);
    let result = expr.eval().unwrap();
    assert_eq!(result,238);
    println!("res = {}", result);
}