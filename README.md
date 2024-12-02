# ODE (Orthographic Dynamic Execution)

learning how to write a programming language

I dont know a ton, I'm just going to try it.

Also just starting learning rust, probably doing a lot of copying until I make my noob code use more slices.

currently its just a tokenizer but this is all of the rust code I have ever written in my life so thats a good start!

Do not expect progress in this repository, learning is the goal, not a good language.

This is effectively just me drawing in the margins of my notebook for now.

But the idea is cool, I was forced to. An ode to an idea I guess.

### Planning notes:

```hs

mutability operator: ~
shadowing is allowed in interior scopes but not in the same scope.

type constraints can contain mixed functions and types if desired

Tool _= {
  weight:int,
  length:int,
  id:int,
}
Swingable _= {
  swing = \: &self, &thing:target -> bool,
}
Breakable _= ~{
  broken:~bool,
  is_broken = ~\: &self -> bool,
}

// enums can contain type constraints, or implemented types
ToolKind #= ~{
  IndestructibleHmmr(Tool+Swingable), // + for and | for or
  Hmmr(Tool+Swingable+Breakable),
  Hmr(Hammer),
}

// Generics come first in <> followed by a type separator

<T, ~U:Tool>:GenericTypeStruct _= {
  meta:T,
  item:U,
},

// an immutable generic set can implement immutable constraints
UnbreakableHammer:Tool,Swingable,Eq ^= {
  id = random(), // <-- immutable, so this would be ran when the struct is initialized, not now.
  swing = \: &self, &thing:target -> bool: {
    << distance_from_target < self.length; // immutable scopes require return because they are not ordered.
    distance_from_target = thing.distance(self);
  },
  eq = \: &self, &thing:other -> bool: {
    << self.id == other.id;
  },
}

// an mutable impl block can implement immutable and mutable constraints
// and may create both immutable and mutable values
Hammer:Swingable,Breakable,Eq ^= ~{
  id = random(), // <-- immutable, so this would be ran when the struct is initialized, not now.
  ~broken = false, // <-- mutable impl can initialize values if desired
  is_broken = \: &self -> bool: ~{ // mutable scope, immutable function (it doesnt depend on outside mutable values, which would need a ~\:)
    broken // mutable scope can implicitly return at the end
  },
  swing = \: &self, &thing:target -> bool: thing.distance(self) < self.length,
  eq = \: &self, &thing:other -> bool: {
    <- self.id == other.id
  },
}

mace:Hammer = { weight = 10, length = 20, };

// You must create values of types by assignment, or by creating a new function that returns it
// Likely I will make a constraint that can be implemented by implementing `new` to allow typename to be callable as function with a set as argument

[] indicates optional in these snippets
fn syntax: myfn = \ named[:type[:default]], args[:type[:default]] -> [ret_type:] { body }
infix fn syntax: myfn = \: named[:type[:default]], args[:type[:default]] -> [ret_type:] { body }
multiple ret fn syntax: myfn = \ named[:type[:default]], args[:type[:default]] -> ret_type, ret_type2: { body }
mutable fn syntax: myfn = ~\ named[:type[:default]], args[:type[:default]] -> [ret_type:] { body }
vararg syntax: myfn = \ named[:type[:default]], named[:type]:... -> [ret_type:] { body }

greet = \ followup:&str, name:&str, greeting:&str:"Hello" -> String: ~{
  "$[greeting], $[name]! $[followup]!"
}

greeting = greet "How are you?";

greetAmy = greeting "Amy";

println greetAmy;

greeting2 = (\<T:Display>: greeting:&T, name:&str -> T: ~{ // if this were infix, \:<T>: instead of \<T>:
  "$[greeting], $[name]!"
} "Wazzup");

greetJosh = greeting2 "Josh";

println joshGreet;

// ~mutable functions evaluate eagerly and can only be evaluated without assigning the result in mutable scopes

~personname="James";
greeting3 = ~\ greeting:&str -> String: ~{
  "$[greeting], $[personname]!"
};
println (greeting3 "Hi");

personname="Mrowwwwwww!";
~greetOphelia = greeting3 "AAAAHHHH!!";
println greetOphelia;

```

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

`if cond then val else val end` is: `cond => {} !> {}`

No else if. Use match for that.

`#{ Pattern, [cond] => {}; }`
// where Pattern is a rust-style match case or _, although I also want to be able to | and & or types, although & will be + because you cant add things in type declarations but you can reference

`for iter \ k v {} OR for cond {}`
iter can also be something that implements iter
`for list \ k v {}`

infer types where possible

Immutable should be reference counted
Mutable should be borrow-checked, if lifetime is required it goes before the ~ (mutability operator)
which is always at the beginning of the type, or name if type is inferred.

rust result/options and multiple returns

Immutable will be lazy.
Actors are parallelized, and are given a world type defined by the Node instance that they can use in their init scope.

mutable scopes can spawn an actor with pid = node @ function varargs...
// where node is an instance of Node which defines message types and timeout value and other stuff

Hopefully I can fold stream iteration and actor message iteration and listening into these @ operators.

```hs

>>> is simple while loop and can also take an ordinary iterator.
@>> produces and loops over a stream iterator from a stream/actor message queue

err:Result<String> = pid <@ msg;

response = pid @> \ msg -> ~ {
  Ok(val) isFloat val => Ok val;
  Ok(val) => Err "Wrong type! $[inspect(val)]";
  Err(val) => Err "Execution Error: $[inspect(val)]";
  Time(val) => Err "TIMED OUT after $[val.timeout]. Total runtime of actor: $[val.running_time]";
};

// stream iterator
res = pid @>> \ Ok(msg), TTL(ttlval) -> ~ {
  Ok(val) isFloat val => Ok val;
  Ok(val) => Err "Wrong type! $[inspect(val)]";
  Err(val) => Err "Execution Error: $[inspect(val)]";
  TTL(val), ttlval > 5000 => Err "TIMED OUT after $[val.timeout]. Total runtime of actor: $[val.running_time]";
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

lazyfib = \ n:int -> int:{
  <- n <= 1 => n !> rec n;
  rec = \ num -> lazyfib (num-1)+(num-2);
}
lazyfib = \ n:int -> int:{
  <- n <= 1 => n !> lazyfib (num-1)+(num-2);
}
// has sequenced scope but doesnt depend on outside mutable variables
lazyfib = \ n:int -> int:~{
  n <= 1 => n !> lazyfib (num-1)+(num-2)
}

matchfib = \ n:int -> int:~{
  0, n<0 => n;
  1 => n;
  eagerfib (n-1)+(n-2)
}
matchfib = \ n:int -> int:~{
  , n<=1 => n;
  eagerfib (n-1)+(n-2)
}

```

`~\ args, list ->` This is an actual first class thing, it is a function that makes its value available until the next semicolon.

Scopes are declared as ```[ret_type][~|#]{}```

`\~` is also an operator on the next scope or args list or variable declaration. It is the mutability operator.
It also doubles as the thing you put lifetime before, because only mutable things use borrow checking.

`<-` is return FROM CURRENT SCOPE.
also return-returns the value if its return value was being collected,
but this is usually irrelevant because it exits that scope.
But this can come into play in mutable situations.

mutable scopes behave like rust scopes

immutable ones are executed lazily in the best order when needed and returno
Immutable scopes may only return immutable variables, and cannot use mutable variables from containing scopes.
Return is REQUIRED and can only be called once.

All files can contain 1 top level anonymous thing that the file can return. And then any number of `_=` `#=` `^=` typedefs, and immutable variables (includes immutable functions).

`val = use "name" file_descriptor` keyword will return the anonymous thing as val, and define the types, functions and constants under "name.thing";

All values from `name` will be accessible at any point in the file `use` was called within, because they must be static.

For `val` it depends on the type, and behaves as normal. Mutable scopes execute at call site, and immutable scopes are executed when they are needed, etc...



```hs
Option<String>:~{
  
  action1 = \ val:Option<String> -> Option: val ~{
    Some(v) => Some (v+"!");
    None
  }
  action2 = \ val:Option<String> -> Option: val ~{
    Some(v) => Some (v+v);
    None
  }

  purefunc = \ x:bool -> Option -> Option:{
    <- x ~{
      true => action1 |> action2;
      action2 |> action1;
    };
  };

  unres = purefunc true;

  myVal:~& = "Hello";
  // I think this is a compiler error. Mutable Some type to an immutable function.
  // Its fine if you move the value though...
  // So, maybe we require all mutable values passed to an immutable function to be moved
  res = unres Some(myVal)?;

  Some(res)

}
```
what happens here? This is kinda a problem.
is res lazy or eager? Is it mutable?

What if the inner immutable transformations contain mutable scopes which got your ref and changed it?
I think I need to find something different from borrow checking.

The above example would be fine, because the resulting string is a different one now,
but it would be possible to create an internal mutable scope and mutate it...

I only like this whole scoping operations idea if I can still make mutable scopes in immutable ones.
So its the memory model for mutable variables that has to change from borrow checking to something else.

or maybe I need to make it so that you cant change the scope of a mutable variable without defining a linear type for it,
and then making a nice way for doing that?
