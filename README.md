learning how to write a programming language

No, I have not read anything on the topic. I'm just going to try it.

learning rust

currently its just a tokenizer but this is all of the rust code I have ever written in my life so thats a good start!

Do not expect progress in this repository, learning is the goal, not a good language.

This is effectively just me drawing in the margins of my notebook.

### Planning notes:

```

mutability operators: ` (or `= if type is being inferred in assignments)
shadowing is allowed in interior scopes but not in the same scope.

[] indicates optional in these snippets
fn syntax: \:name [type]:named[:default], [type]:args[:default] -> [ret_type] { body }
anon fn syntax: myfn = \ [type]:named[:default], [type]:args[:default] -> [ret_type] { body }
infix fn syntax: myfn = \:: [type]:named[:default], [type]:args[:default] -> [ret_type] { body }
infix fn syntax: myfn = \:: [type]:named[:default], [type]:args[:default] -> ret_type, ret_type2 { body }

last arg may be named ... for varargs

functions are closures and your function must be declared as mutable if it references mutable values as part of its closure,
but they may have mutable arguments without being marked mutable
if they return a mutable value their return value will retain its mutability

functions may return multiple values and then may be used in place of multiple args

infix makes it so that the first arg may be on the left.
if functions are declared in impl blocks they may have first argument self.
doing infix would then make the second arg the left arg

calling function requires no parenthesis around args other than for grouping

you may curry up until the first default argument,
at which point you must provide the rest or it will call, varargs are allowed at end and cannot be curried.
if a function returns multiple values the types must be specified

scopes can be used as let in, all return a value or () if no value,
can return a value by not including semicolon on last value,
and also must be marked as mutable if they contain mutable values
all scopes can return early with << val

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

~@ Ident { Pattern [cond] => {}[,] }
Ident ~@ { Pattern [cond] => {}[,] } // where Pattern is a rust-style match case or _

in this language, you will be able to implement traits on structs not created by your file if they are mutable, sometimes allowing pseudo-structural typing
you may not have mutable instances of immutable structs or vice versa
mutable structs may have immutable values, immutable structs may NOT have mutable values

struct instances may have values added but not removed if marked. Struct instances are basically generic sets but with expected values

for iter \ k v {} OR for cond {}
iter can also be something that implements iter
for list \ k v {}

`pub` may be used for top level items but not within structs or impl blocks themselves

infer types where possible

Immutable values should be reference counted
Mutable values should be borrow-checked if possible?

no null, rust options and multiple returns

```
