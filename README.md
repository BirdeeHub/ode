learning how to write a programming language

No, I have not read anything on the topic. I'm just going to try it.

learning rust

currently its just a tokenizer but this is all of the rust code I have ever written in my life so thats a good start!

Do not expect progress in this repository, learning is the goal, not a good language.

This is effectively just me drawing in the margins of my notebook.

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
ToolKind =| `{
  IndestructibleHmmr(Tool:Swingable),
  Hmmr(Tool:Swingable:Breakable),
  Hmr(Hammer),
}

// Generics come first in <>

<T, `U:Tool>GenericTypeStruct _= {
  T:meta,
  U:item,
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
  broken `= false,
  `\:is_broken &self -> bool: {
    broken // mutable scope can implicitly return at the end
  },
  \:swing &self, &thing:target -> bool: thing.distance(self) < self.length,
  \:eq &self, &thing:other -> bool: {
    << self.id == other.id
  },
}

mace = Hammer { weight:10, length:20, };

[] indicates optional in these snippets
fn syntax: \:myfn named[:type[:default]], args[:type[:default]] -> [ret_type]: { body }
anon fn syntax: myfn = \ named[:type[:default]], args[:type[:default]] -> [ret_type]: { body }
infix fn syntax: myfn = \:: named[:type[:default]], args[:type[:default]] -> [ret_type]: { body }
infix fn syntax: \::myfn named[:type[:default]], args[:type[:default]] -> [ret_type]: { body }
multiple ret fn syntax: myfn = \:: named[:type[:default]], args[:type[:default]] -> ret_type, ret_type2: { body }
mutable fn syntax: `\:myfn named[:type[:default]], args[:type[:default]] -> [ret_type]: { body }
mutable anon fn syntax: myfn = `\ named[:type[:default]], args[:type[:default]] -> [ret_type]: { body }
vararg syntax: \:myfn named[:type[:default]], named[:type]:... -> [ret_type]: { body }

\:greet name:&str, followup:&str, greeting:&str:"Hello" -> String: {
  "$[greeting], $[name]! $[followup]!"
}

amyGreet = greet "Amy";

greeting = amyGreet "How are you?";

println greeting;

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

~| Ident { Pattern [cond] => {}[,] }
Ident ~| { Pattern [cond] => {}[,] } // where Pattern is a rust-style match case or _

for iter \ k v {} OR for cond {}
iter can also be something that implements iter
for list \ k v {}

infer types where possible

Immutable should be reference counted
Mutable should be borrow-checked if possible?

rust result/options and multiple returns

possible:?????

mutable scopes can create actor processes but immutable ones cant?
Immutable will be lazy, and it will also attempt to automatically parallelize if possible.

```
