/*
    Lazaro Marin CI: 27.870.547
    EBNF
    <InputElement>::= WhiteSpace | Comment | Token
    <WhiteSpace>::=  space | \t | \r | \n | \f
    <Comment>::= cualquier cadena finalizada en \r o \n
    <Token>::= Identifier | KeyWord | Literal | Separator | Operator
    <Identifier>::= Letter | Identifier Letter | Identifier Digit
    <Letter>::= a | b | … | z | A | B | … |Z
    <Digit> ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    <KeyWord>::= boolean | else | if | int | main | void | while
    <Literal> ::= Boolean Integer
    <Boolean> ::= true | false
    <Integer> ::= Digit | Integer Digit
    <Separator> ::= ( | ) | { | } | ; | .
    <Operador> ::= = | + | - | * | / | > | >= | == | != | < | <= | && ||| | !

*/
use std::collections::HashMap;
use std::fs;
use std::process;

fn main() {
    llenar_token()
}

//LLENAR EL TOKEN CON LOS RESPECTIVOS CAMPOS (un hashmap)
fn llenar_token() {
    //INDENTIFIER: LETTER | IDENTIFIER LETTER | IDENTIFIER DIGIT
    let identifier = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    //KEYWORD: BOOLEAN | ELSE | IF | INT | MAIN | VOID | WHILE
    let keyword = vec![
        "true", "false", "else", "if", "int", "main", "void", "while",
    ];

    //LITERAL: BOOLEAN | INTEGER
    let literal = vec!["boolean", "integer"];

    //SEPARATOR ( | ) | {  |  }  |  ;  |  , | .
    let separator = vec!["(", ")", "{", "}", ";", ","];

    //OPERATOR
    let operator = vec![
        "=", "+", "-", "*", "/", ">", "<", ">=", "<=", "==", "!=", "&&", "||", "!",
    ];

    //Creamos unos ciclos para guardar todo el lexico de nuestro lenguaje JAY
    let mut token = HashMap::new();

    for key in &identifier {
        token.insert(key, "identifier");
        for key in &separator {
            token.insert(key, "separator");
            for key in &literal {
                token.insert(key, "literal");
                for key in &keyword {
                    token.insert(key, "keyword");
                    for key in &operator {
                        token.insert(key, "operator");
                    }
                }
            }
        }
    }

    //vector para guardar las lineas del jay
    let mut vector: Vec<String> = Vec::new();
    //let mut aux = Vec::new();
    let text = fs::read_to_string("src/FUENTE.jay").unwrap();

    for (i, text) in text.lines().enumerate() {
        if text.trim().is_empty() {
            continue;
        }
        if text.trim().starts_with("//") {
            continue;
        }
        let comentarios: Vec<&str> = text.split("//").collect();
        let lineas: Vec<&str> = comentarios[0].trim().split_whitespace().collect();

        //aca comparamos los diferentes casos, para separar las cadenas dentro de vector en dado caso esten pegadas ej: main(){
        let mut cadenas = String::new();
        for ln in lineas {
            if let Some(var) = token.get(&ln) {
                if var == &"keyword" {
                    vector.push(ln.to_string());
                }
            } else {
                for caracter in ln.chars() {
                    if let Some(var) = token.get(&caracter.to_string().as_str()) {
                        if var == &"identifier" {
                            cadenas.push(caracter);
                        } else if var == &"separator" {
                            if !cadenas.is_empty() {
                                vector.push(cadenas.clone());
                                cadenas.clear();
                            }
                            vector.push(caracter.to_string());
                        }
                    } else {
                        println!("Error Lexico ---> {}  linea # {} ", caracter, i + 1);
                        process::exit(0);
                    }
                }
            }
        }
    }
}
