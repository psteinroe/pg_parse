use pg_parse;

fn main() {
    let sql = "SELECT id, name FROM users WHERE active = true";

    match pg_parse::parse(sql) {
        Ok(result) => {
            println!("Successfully parsed SQL!");
            println!("Stderr: {}", result.stderr);
            println!("Parse tree has {} statements", result.protobuf.stmts.len());
            println!("Parse tree has {:#?}", result.protobuf.stmts);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}

