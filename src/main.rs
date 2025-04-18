use rslox::{Lexer, Parser, Resolver, TypeInferrer};
use std::fs;

fn main() {
    let mut path = "source.lox".to_string();
    let source =
        fs::read_to_string(&mut path).expect(format!("Error reading file {}", path).as_str());
    let source = format!("{} ", source);

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex();

    for err in lexer.get_errors() {
        println!("Lexing error: {:?}", err);
    }

    let mut parser = Parser::new(tokens, source.as_str());
    let mut ast = parser.parse();

    for error in parser.get_errors() {
        println!("{:?}", error);
    }

    let mut resolver = Resolver::new(&ast, source.clone());
    let resolving_errors = resolver.resolve();

    for error in resolving_errors {
        println!("{:?}", error);
    }

    let mut type_inferrer = TypeInferrer::new(&ast, source);
    let inferring_errors = type_inferrer.infer();

    for error in inferring_errors {
        println!("{:?}", error);
    }
}
