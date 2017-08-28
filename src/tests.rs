use super::{parse};

#[test]
fn one_symbol(){
    assert_eq!(parse("(set!)").car().unwrap_symbol(), "set!")
}

#[test]
fn nested_list(){
    assert_eq!(parse("((set!))").car().car().unwrap_symbol(), "set!")
}


#[test]
fn numbers(){
    assert_eq!(parse("(set! x 3)").cdr().cdr().car().unwrap_number(), 3)
}

#[test]
fn true_literal(){
    assert_eq!(parse("(#t)").car().is_true(), true);
}

#[test]
fn false_literal(){
    assert_eq!(parse("(#f)").car().is_true(), false);
}

#[test]
fn bool_literal_case_insentive(){
    assert_eq!(parse("(#F)").car().is_true(), false);
    assert_eq!(parse("(#T)").car().is_true(), true);
}

#[test]
fn true_oposite_of_false(){
    assert_eq!(parse("(#t)").car().is_false(), false);
    assert_eq!(parse("(#f)").car().is_false(), true);
}

// Test for literals, to make sure that no parsing is done after a ' symbol

#[test]
fn list_literals(){
    assert_eq!(parse("('(1 2 3))").car().unwrap_literal(), "(1 2 3)")
}

#[test]
fn nested_list_literals(){
    assert_eq!(parse("('((1 2) 3))").car().unwrap_literal(), "((1 2) 3)")
}


#[test]
fn literals(){
    assert_eq!(parse("(if 'symb)").cdr().car().unwrap_literal(), "symb")
}

#[test]
fn special_symbols(){
    parse("(= * / - + ?)");
}

#[test]
#[should_panic]
fn not_closed_expression(){
    parse("(set! x 1");
}

#[test]
#[should_panic]
fn invalid_expression(){
    parse("(if #t 3 2) (exit)");
}
