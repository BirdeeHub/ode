use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};
mod parser;
mod runtime;
use crate::parser::{Parser,parser_types::TokenizerSettings};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {:?}", args);
    let inputvar = args.get(1).expect("No input file provided!");
    let contents = read_file(inputvar)?;
    let settings  = TokenizerSettings {
        blockcomstart: "#^",
        blockcomend: "#$",
        linecom: "#",
        ops: &[
            "=", "+", "-", "/", "%", "//", "|", "^", "++",
            ">>", "<<", "!", "||", "&&", "..",
            "!=", "==", "<=", ">=",
            "-=", "+=", "*=", "/=", "&=", "|=", "%=", "//=", "^=", "++=",
            "\\", "\\:", "...", "->", "<-", ">>=", "|>", "<|", "?",
            "'", "&", "*", "\\&",
            "=>", "!>",
            ">>>", ">>|", ">>!",
            "<@", "@", "@@", "@>", "@>>",
            ":", ".", ",", ";",
        ],
        enclosers: &[("(", ")"), ("[", "]"), ("{", "}"), ("<", ">"), ("#<", ">"), ("#@", "@#")],
        charop: "`",
        templop: "\"",
        interstart: "$[",
        interend: "]",
        escape_char: '\\',
    };
    /*
    ` for chars
    ' mutability op (lifetime if needed goes before, & goes after)
    left (\: type:default:argname, ::arg2 -> {}) right
    \ type:default:argname, ::arg -> rettype {}
    then => else !> and match # only

    struct:name:<T> [']{
      name:type:default;
    }
    trait:name:[<T>] [']{
      name:type;
    }
    enum:name:[<T>] [']{
      name:type;
    }

    Impl
    <T>:[type,names]:structname [']{
      name = value;
    }

    Scope
    [type] [']{
        [type][:]varname = value;
        <- varname;
    }
    [type] {
        [type]:varname2 = varname;
        <- varname;
        [type]:varname = value;
    }
    [type] '{
        [type][:]varname = value;
        varname
    }

    Match
    val [type] [']{
        Pattern[,][cond] -> val+2;
        !> val-2;
    }

    Sets [']{ val1, val2, val3 }
    Hashmap [']{ key1: val1, key2: val2, key3: val3 }
    Arrays ['][val1, val2, val3]

    \& makes it so that you can have multiple mutable refs?
    but dereference becomes the function defined and returns an option?
    You cant read or write to the value if you dont own it except by using this if defined?
    Is defined at use site of mutable types?
    Possibly mutable structs define a signature for it?
    Will be used instead of unsafe?

    I want to have raw pointer writing for embedded
    and IO and whatnot passed in via the node definition
    and then you can define mutable IO and monadic pure IO and pass them in.
    But im not sure how this is going to work completely

    <@ is value to stream/actor
    @ is open/run stream/actor on node
    @@ is same but on current node
    @> is value from stream/actor
    @>> untilcond, fallback TTL(int)
    >>> while ->
    >>| continue
    >>! break

    :name = 5;
    `int:name2 = 6;

    "#!" "#@" <- node config enclosers
    doubles as shebang for interpreted mode
    */

    let mut parser = Parser::new(&settings,contents.chars());
    let ast = parser.parse_program().unwrap();
    let rtvals = runtime::evaluate(&ast);
    println!("{:?}", rtvals);
    println!("Time: {:?}", start.elapsed());

    Ok(())
}
