mod lang;
use lang::Lexer;
fn main() {
    let mut lexer = Lexer::new("var xs = \n12;".to_string());
    let mut token;
    loop {
        token = lexer.lex();
        println!("{:?}", token);
        if token.is(lang::TokenKind::EOF) {
            break;
        }
    }
}
