# ODE (Orthographic Dynamic Execution)

learning how to write a programming language

I dont know a ton, I'm just going to try it.

Also just starting learning rust, probably doing a lot of copying until I make my noob code use more slices.

currently its just a tokenizer but this is all of the rust code I have ever written in my life so thats a good start!

Do not expect progress in this repository, learning is the goal, not a good language.

This is effectively just me drawing in the margins of my notebook for now.

But the idea is cool, I was forced to. An ode to an idea I guess.

### Planning notes:

```

mutability operator: `
shadowing is allowed in interior scopes but not in the same scope.

type constraints can contain mixed functions and types if desired

Tool _= {
  weight:int,
  length:int,
  id:int,
}
Swingable _= {
  \:swing &self, &thing:target -> bool,
}
Breakable _= `{
  `\:is_broken &self -> bool,
}

// enums can contain type constraints, or implemented types
ToolKind ~= `{
  IndestructibleHmmr(Tool+Swingable), // + for and | for or
  Hmmr(Tool+Swingable+Breakable),
  Hmr(Hammer),
}

// Generics come first in <> followed by a type separator

<T, `U:Tool>:GenericTypeStruct _= {
  meta:T,
  item:U,
},

// an immutable generic set can implement immutable constraints
UnbreakableHammer:Tool,Swingable,Eq = {
  id = random(), // <-- immutable, so this would be ran when the struct is initialized, not now.
  \:swing &self, &thing:target -> bool: {
    << distance_from_target < self.length; // immutable scopes require return because they are not ordered.
    distance_from_target = thing.distance(self);
  },
  \:eq &self, &thing:other -> bool: {
    << self.id == other.id;
  },
}

// an mutable impl block can implement immutable and mutable constraints
// and may create both immutable and mutable values
Hammer:Swingable,Breakable,Eq ^= `{
  id = random(), // <-- immutable, so this would be ran when the struct is initialized, not now.
  `broken = false, // <-- mutable impl can initialize values if desired
  is_broken = \: &self -> bool: `{ // mutable scope, immutable function (it doesnt depend on outside mutable values, which would need a `\:)
    broken // mutable scope can implicitly return at the end
  },
  \:swing &self, &thing:target -> bool: thing.distance(self) < self.length,
  eq = \: &self, &thing:other -> bool: {
    << self.id == other.id
  },
}

mace:Hammer = { weight = 10, length = 20, };

// You must create values of types by assignment, or by creating a new function that returns it
// Likely I will make a constraint that can be implemented by implementing `new` to allow typename to be callable as function with a set as argument

[] indicates optional in these snippets
fn syntax: myfn = \ named[:type[:default]], args[:type[:default]] -> [ret_type:] { body }
infix fn syntax: myfn = \: named[:type[:default]], args[:type[:default]] -> [ret_type:] { body }
multiple ret fn syntax: myfn = \ named[:type[:default]], args[:type[:default]] -> ret_type, ret_type2: { body }
mutable fn syntax: myfn = `\ named[:type[:default]], args[:type[:default]] -> [ret_type:] { body }
vararg syntax: myfn = \ named[:type[:default]], named[:type]:... -> [ret_type:] { body }

greet = \ followup:&str, name:&str, greeting:&str:"Hello" -> String: `{
  "$[greeting], $[name]! $[followup]!"
}

greeting = greet "How are you?";

greetAmy = greeting "Amy";

println greetAmy;

greeting2 = (\<T:Display>: greeting:&T, name:&str -> T: `{ // if this were infix, \:<T>: instead of \<T>:
  "$[greeting], $[name]!"
} "Wazzup");

greetJosh = greeting2 "Josh";

println joshGreet;

// `mutable functions evaluate eagerly and can only be evaluated without assigning the result in mutable scopes

`personname="James";
greeting3 = `\ greeting:&str -> String: `{
  "$[greeting], $[personname]!"
};
println (greeting3 "Hi");

personname="Mrowwwwwww!";
`greetOphelia = greeting3 "AAAAHHHH!!";
println greetOphelia;

functions are closures and your function must be declared as mutable if it references external mutable values as part of its closure,
if they return a mutable value their return value will retain its mutability

functions may return multiple values and then may be used in place of multiple args

infix makes it so that the first arg may be on the left.
if functions are declared in impl blocks they may have first argument self.
doing infix would then make the second arg the left arg

calling function requires no parenthesis around args other than for grouping

you may curry up until the first default argument,
at which point you must provide the rest or it will call, varargs are allowed at end and cannot be curried.
if a function returns multiple values the types must be specified

Scopes all return a value or () if no value,
scopes can be used as let in if immutable (order doesnt matter)
and also must be marked as mutable if they contain mutable values

mutable scopes can return early with << val
and can return a value by not including semicolon on last value,

immutable scopes MUST return with << val; and may only do so once
Conventional to place it at beginning or end of scope

All this requiring of marking things mutable is very important.
The idea is to be explicit enough about it that it is possible to
lazily evaluate all non-mutable things.

tuples are [ [type]:val, [type]:val2 ] and can be destructured the same way on argument and return, (with [:default] as well)
if mutable this is a list if generic and an array if not
if lazy it can always be made contiguous in memory like a struct can (hopefully)

generic sets can be made with { sdadsa = sdasdadas[,] }
differentiated from block by using , instead of ; (if no trailing , the last line has = whereas in a scope it either needs a semicolon, or wouldnt have an =)
If not mutable, they can recursively self-access

`if cond then val else val end` is: cond => {} >> {}
`if cond then val else if cond then val else val end` is: cond => {} >>> cond => {}

~ Ident { Pattern, [cond] => {}; }
Ident ~ { Pattern, [cond] => {}; } // where Pattern is a rust-style match case or _, although I also want to be able to | and & or types, although & will be + because you cant add things in type declarations but you can reference

for iter \ k v {} OR for cond {}
iter can also be something that implements iter
for list \ k v {}

infer types where possible

Immutable should be reference counted
Mutable should be borrow-checked, if lifetime is required it goes before the ` (mutability operator)
which is always at the beginning of the type, or name if type is inferred.

rust result/options and multiple returns

Immutable will be lazy.
Actors are parallelized, and are given a world type defined by the Node instance that they can use in their init scope.

mutable scopes can spawn an actor with pid = node @ function varargs...
// where node is an instance of Node which defines message types and timeout value and other stuff

Hopefully I can fold stream iteration and actor message iteration and listening into these @ operators.

>>> is simple while loop and can also take an ordinary iterator.
@>> produces and loops over a stream iterator from a stream/actor message queue

err:Result<String> = pid <@ msg;

response = pid @> \ msg -> ~ {
  Ok(val) isFloat val => Ok val,
  Ok(val) => Err "Wrong type! $[inspect(val)]",
  Err(val) => Err "Execution Error: $[inspect(val)]",
  Time(val) => Err "TIMED OUT after $[val.timeout]. Total runtime of actor: $[val.running_time]",
};

// stream iterator
res = pid @>> \ Ok(msg), TTL(ttlval) -> ~ {
  Ok(val) isFloat val => Ok val,
  Ok(val) => Err "Wrong type! $[inspect(val)]",
  Err(val) => Err "Execution Error: $[inspect(val)]",
  TTL(val), ttlval > 5000 => Err "TIMED OUT after $[val.timeout]. Total runtime of actor: $[val.running_time]",
};

// argument specififications such as in match and fn decleration may reference earier arguments

// A typematch is to be an actual type.

file structure.

Top level must be immutable, or typedef/impl

all files may contain a single scope, mutable or immutable, at top level

files with a mutable scope at top level and a node type to implement may be called as actors.

files with an immutable scope at top level may be called as lazily evaluated functions.

Scopes may only be declared anonymously. (Top level file scopes may be upvalued with use keyword if your scope has a compatible mutablility type?)

If you were wishing you could do that, make some types... Its basically that

```

`\ args, list ->` This is an actual first class thing, it is a function that takes a scope.

The scope declared is either the ```[~`[:ret_type]]{}```  or until the next semicolon

`~` match is an operator on the next scope, it takes a thing to match on, can take an args list and match on one of the args at a time in arms.
It doesn't have `<-` and the last semicolon is optional, but including it or not doesnt change behavior.

`\`` is also an operator on the next scope or args list or variable declaration. It is the mutability operator. It also doubles as the thing you put lifetime before, because only mutable things use borrow checking.

mutable scopes behave like rust scopes `<-` is return

immutable ones are executed lazily in the best order when needed and return is REQUIRED and can only be called once.

All files can contain 1 top level anonymous non-typedef thing that the file can return. And then any number of `_=` `~=` `^` typedefs, named immutable functions, and immutable variables.

`val = use "name" file_descriptor` keyword will return the anonymous thing as val, and define the types, functions and constants under "name.thing";

If the top level anonymous thing is a mutable scope, it is ran when the file `use`ing it is ran.

It is likely not a good idea to do it a ton,
because thats the only case when circular dependency matters.

In all other cases it should be possible to declare the contents lazily without circular dependency causing much issue.

### Currently Completely BS EBNF:

My next effort will be to formally specify a context free grammar so that
I have an actual yardstick to aim at for the parser.

Again, currently, this EBNF is still a work in progress.

```EBNF
(* Types and Declarations *)
Constraint        = Identifier, "_=", "{", { Field, "," }, "}".
Impl              = Identifier, "^=", "{", { Assignment, ";" }, "}".
Enum              = Identifier, "~=", "{", { EnumPattern, "," }, "}".
EnumPattern       = Identifier, "(", TypeConstraints, ")".
TypeConstraints   = [[Identifier,]"`",["&"|"*",] ] Identifier, { ( "+", Identifier ) | ( "|", Identifier ) }.
GenericDecl       = "<", Generics, ">", ":".
Generics          = { Identifier, ":", TypeConstraints [, "," ] }.
Type              = [[Identifier,]"`",["&"|"*",] ] [ Identifier, ] ":".

(* Scopes *)
Scope             = [ GenericDecl,] [Type,] [ScopeType, ] "{" ScopeBody|MatchArms "}", ";".
ScopeBody         = { Statement[, ";" ] }.
MatchArms         = { Pattern, [",", Expression], ["=>", Expression], [";"] }.
ScopeType         = "~"|[Identifier,] "`".

(* Functions *)
FnArgs            = RegFnArgs | InfixFnArgs.
RegFnArgs         = "\", [GenericDecl ,] Parameters, "->".
InfixFnArgs       = "\:", [GenericDecl ,] Parameters, "->".
Parameters        = Parameter, { ",", Parameter }.
Parameter         = Identifier, [":", Type, [":", DefaultValue]].
DefaultValue      = Literal | Expression.

(* Statements *)
Statement         = Expression | ReturnStatement.
Expression        = Assignment | FunctionCall | Operation | Scope | FnArgs | Pattern | Loop | StreamIteration.
ReturnStatement   = "<-", { Expression, ",", }.
Assignment        = [GenericDecl,] [Type,] Identifier, "=", Expression.
FunctionCall      = Identifier, { { " " | "\n" | "\t" | "\r" | "\f" | "\b" }, Argument, }.
Operation         = Expression, Operator, Expression.
Pattern           = Identifier, "(", PatternConstraints, ")".
PatternConstraints= Literal | ([[Identifier,]"`",["&"|"*",] ] Identifier, { ( "+", Identifier ) | ( "|", Identifier ) }).
Argument          = Literal | Expression | Identifier.

(* Control Structures *)
ThenElse          = Condition, "=>", Scope, ["!>", Scope], ";".
Loop              = ">>>", FnArgs, ScopeBody, ";".
StreamIteration   = "@>>", FnArgs, ScopeBody, ";".

(* Literals and Identifiers *)
Literal           = Integer | String | Float | Boolean.
Identifier        = Letter, { Letter | Digit | "_" }.
Operator          = "=", "+", "-", "/", "%",
                    "!", "!=", "==", "<=", ">=",
                    "=", "<", ">", "||", "&&",
                    "|", ">>", "<<",
                    (*above are standard stuff, you should recognize & * ? from below also*)
                    "\\", "\\:", "|>", "<-", "->", "...",
                    "~", "?", "&", "*", "`",
                    "|=", "^=", "~=",
                    ">>>", ">>|", ">>!", (* while continue break, continue and break can be given values to return matching scope return type *)
                    "=>", "!>",
                    "<@", "@", "@>", "@>>",
                    ":", ",", ";",
                    "#",

(* Miscellaneous *)
Comment           = LineComment | BlockComment.
LineComment       = "//", { AnyChar }.
BlockComment      = "/*", { AnyChar }, "*/".

(* File Structure *)
File              = { Declaration | UseStatement }.
Declaration       = TypeDef | FunctionDecl | VariableDecl.
VariableDecl      = Identifier, "=", Expression.
UseStatement      = "use", String, Identifier.

(* Enclosures *)
Enclosure         = "(", Expression, ")" | "[", ListItems, "]" | Scope.
        (*enclosers: &[("<", ">"), ("#<", ">"), ("$[","]")],*)
ListItems         = { ",", Expression }.
SetItems         = { { Expression, "," } "}".
```
