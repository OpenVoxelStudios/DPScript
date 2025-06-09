# FFI

DPScript's FFI - if you can even call it that - is an interface for interacting with
Minecraft commands.
Yes, it's technically not FFI in the normal definition, but commands are a foreign function
and DPScript has an interface for them, so I call it FFI.

---

Looking at the standard library, you can see a ton of examples of this in use:

```dpscript
// std/base.dps
module std/base;

#[cmd("tellraw %% [\"\", %%]")]
pub facade fn tellraw(who: Selector, message: Component[]);

#[cmd("summon %%")]
pub facade fn summon(ent: Entity);

#[cmd("summon %% %% %% %%")]
pub facade fn summon_at(ent: Entity, x: int, y: int, z: int);

// Guys I need an identifier type
#[cmd("give %% %%")]
pub facade fn give(ent: Entity, item: NBTPath);

// Guys I need an identifier type
#[cmd("give %% %% %%")]
pub facade fn give_nbt(ent: Entity, item: NBTPath, data: NBT);
```

---

But what does this actually mean? Let's take a look at the `give_nbt` function:

```dpscript
#[cmd("give %% %% %%")]
pub facade fn give_nbt(ent: Entity, item: NBTPath, data: NBT);
```

Here, there are three components:

1. The attribute (`#[cmd("give %% %% %%")]`)

This attribute tells the compiler that the actual syntax of this command is `/give %% %% %%`.
It's pretty self-explanatory.

2. The function itself.

The `facade` part tells the compiler that this is a FFI function.

3. The arguments.

In DPScript, every value can be turned into a string, whether it be through components, getting
its store, its path, or whatever it may be. When an FFI function is compiled, the `%%` in the
attribute get replaced with the stringified form of the corresponding argument, in order. If
there are three arguments, there should be three `%%`s.

---

These aren't real functions, as their call is replaced with the command during the process of
lowering to IR. Some functions like `tellraw` have builtin helpers to make sure arguments get
their text component forms, and I'm working on a way for users to define functions like that too.

---

P.S. The "Guys I need an identifier type" comment is real. I need another type that isn't `NBTPath`
     for clarity purposes.
