extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "scheme.pest"]
struct SchemeParser;
use pest::{Parser, RuleType};
use pest::iterators::{Pairs, Pair};
use pest::inputs::{Input,StringInput};

#[cfg(test)]
mod tests;

/// Value represents all the things that are elements of the list expression
#[derive(Debug)]
pub enum Value{
    /// A linked list
    List(Cons),

    /// Symbol, eg. (if this than that), the word "if" is a symbol as well as "this", "than" and
    /// "that"
    Symbol(String),

    ///  A number
    Number(usize),

    /// A boolean (true or false) 
    Bool(bool),

    /// A literal which is not futher parsed
    Literal(String),

    /// The end of a linked list 
    Null
}

impl Value{
    /// Gives the head of the list if value is a list
    pub fn car(self) -> Box<Value>{
        match self{
           Value::List(c) => c.car,
            _ => panic!("Not a pair")
        }
    }

    /// Gives the tail of the list if value is a list
    pub fn cdr(self) -> Box<Value>{
        match self{
            Value::List(c) => c.cdr,
            _ => panic!("Not a pair")
        }
    }

    /// Gives true if the current value is a symbol false if not
    pub fn is_symbol(&self) -> bool{
        match self{
            &Value::Symbol(_) => true,
            _ => false
        }
    }

    /// Gives true if the current value is a number false if not
    pub fn is_number(&self) -> bool{
        match self{
            &Value::Number(_) => true,
            _ => false
        }
    }

    /// If the current value is a boolean, gives the value of that boolean
    /// otherwise false is returned
    pub fn is_true(&self) -> bool{
        match self{
            &Value::Bool(val) => val,
            _ => false
        }
    }

    /// Oposite of is_true()
    pub fn is_false(&self) -> bool{
        !self.is_true()
    }

    /// Gives the symbol stored in the Value container
    pub fn unwrap_symbol(self) -> String{
        match self{
            Value::Symbol(symb) => symb,
            _ => panic!("value is not a symbol")
        }
    }

    /// Gives the number stored in the Value container
    pub fn unwrap_number(self) -> usize{
        match self{
            Value::Number(n) => n,
            _ => panic!("value is not a number")
        }
    }


    /// Gives the literal contained in Value
    pub fn unwrap_literal(self) -> String{
        match self{
            Value::Literal(lit) => lit,
            _ => panic!("value is not a literal")
        }
    }
}

/// A linked list
#[derive(Debug)]
pub struct Cons{
    car: Box<Value>, // head
    cdr: Box<Value> // tail
}   

/// Some extra methods that can be used on a pest::iterators::Pair 
trait PairMixin{
    /// Give the string value of the current expression
    fn string_value(self) -> String;

    /// Give the integer value of the current expression
    /// will panic if the current expression is not something that can be converted to a integer
    fn usize_value(self) -> usize;
}

impl<R: RuleType,I: Input> PairMixin for Pair<R,I>{
    fn string_value(self) -> String{
        self.into_span().as_str().to_string()
    }

    fn usize_value(self) -> usize{
        self.into_span().as_str().to_string().parse().unwrap()
    }
}

trait PairsMixin<R: RuleType, I: Input>{
    fn first_pair(self) -> Option<Pair<R,I>>;
}

impl<R: RuleType, I: Input> PairsMixin<R,I> for Pairs<R,I>{
    fn first_pair(mut self) -> Option<Pair<R,I>>{
        self.next()
    }
}

/// This function will return a abstract syntax tree represented as a deep nested linked list
/// for given s-expression syntax. 
///
/// If some syntax error was in the input the program calling this function will panic.
///
/// Example:
///
/// ```
/// use sparser::parse;
///
/// let tree = parse("(define (fac n) (if (= n 0) 1 (* (fac (- n 1)))))");
/// assert_eq!(tree.car().unwrap_symbol(), "define")
/// ```
pub fn parse(input: &'static str) -> Value{
    let pairs = SchemeParser::parse_str(Rule::scheme, input).expect("Parsed output");
    make_ast(pairs.first_pair().unwrap().into_inner().first_pair().unwrap())
}

fn make_ast(pair: Pair<Rule, StringInput>) -> Value{
    match pair.as_rule(){
            Rule::list => make_list(pair.into_inner()),
            Rule::symbol => Value::Symbol(pair.string_value()),
            Rule::number => Value::Number(pair.usize_value()),
            Rule::true_lit => Value::Bool(true),
            Rule::false_lit => Value::Bool(false), 
            Rule::literal => Value::Literal(pair.string_value()),
            _ => panic!("Not covered expression {:?}. Please implement.", pair.as_rule())
    }
}

fn make_list(mut pairs: Pairs<Rule, StringInput>) -> Value{
    let element = pairs.next();
    match element{
        Some(el) => Value::List(Cons{car: Box::new(make_ast(el)),
                                cdr: Box::new(make_list(pairs))}),
        None => Value::Null
    }
}

