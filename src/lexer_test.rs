use super::*;

#[test]
fn test_next_token_with_some_literals() {
    let tests = [
        (token::TokenType::ASSIGN, "="),
        (token::TokenType::PLUS, "+"),
        (token::TokenType::LPAREN, "("),
        (token::TokenType::RPAREN, ")"),
        (token::TokenType::LBRACE, "{"),
        (token::TokenType::RBRACE, "}"),
        (token::TokenType::COMMA, ","),
        (token::TokenType::SEMICOLON, ";"),
        (token::TokenType::EOF, "\0"),
    ];

    let input: &str = "=+(){},;";

    let mut lexer = lexer::Lexer::new(input);

    for (token_type, character) in tests {
        let token = lexer.next_token();

        assert_eq!(token.token_type, token_type);
        assert_eq!(token.literal, character);
    }
}

#[test]
fn test_next_token_with_more_literals() {
    let input: &str = r#"
        let five = 5;

        let ten = 10;

        let add = fn(x,y) {
            x+y;
        };

        let result = add(five, ten);
    "#;

    let tests = [
        (token::TokenType::LET, "let"),
        (token::TokenType::IDENT, "five"),
        (token::TokenType::ASSIGN, "="),
        (token::TokenType::INT, "5"),
        (token::TokenType::SEMICOLON, ";"),
        (token::TokenType::LET, "let"),
        (token::TokenType::IDENT, "ten"),
        (token::TokenType::ASSIGN, "="),
        (token::TokenType::INT, "10"),
        (token::TokenType::SEMICOLON, ";"),
        (token::TokenType::LET, "let"),
        (token::TokenType::IDENT, "add"),
        (token::TokenType::ASSIGN, "="),
        (token::TokenType::FUNCTION, "fn"),
        (token::TokenType::LPAREN, "("),
        (token::TokenType::IDENT, "x"),
        (token::TokenType::COMMA, ","),
        (token::TokenType::IDENT, "y"),
        (token::TokenType::RPAREN, ")"),
        (token::TokenType::LBRACE, "{"),
        (token::TokenType::IDENT, "x"),
        (token::TokenType::PLUS, "+"),
        (token::TokenType::IDENT, "y"),
        (token::TokenType::SEMICOLON, ";"),
        (token::TokenType::RBRACE, "}"),
        (token::TokenType::SEMICOLON, ";"),
        (token::TokenType::LET, "let"),
        (token::TokenType::IDENT, "result"),
        (token::TokenType::ASSIGN, "="),
        (token::TokenType::IDENT, "add"),
        (token::TokenType::LPAREN, "("),
        (token::TokenType::IDENT, "five"),
        (token::TokenType::COMMA, ","),
        (token::TokenType::IDENT, "ten"),
        (token::TokenType::RPAREN, ")"),
        (token::TokenType::SEMICOLON, ";"),
        (token::TokenType::EOF, "\0"),
    ];

    let mut lexer = lexer::Lexer::new(input);

    for (token_type, character) in tests {
        let token = lexer.next_token();
        assert_eq!(token.token_type, token_type);
        assert_eq!(token.literal, character);
    }
}
