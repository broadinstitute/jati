use jati;
use jati::token::token_iter::TokenIter;
use jati::code_point::CodePoint;

#[test]
fn see_whether_it_works() {
    let string = String::from("Hello, world!");
    let code_points = jati::scan_string(string);
    let mapped = code_points.map(|code_point: CodePoint|{ code_point });
}