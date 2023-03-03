# mybox

Type Brainteaster! Can you write the following code in your language?

# Problem

A box contains some data as a string, it can be in two states: _unlocked_ or _locked_.
You can lock the box only in the unlocked state, and you can unlock the box only in the locked state, given the correct pin.
You can access the data in the box, but you can only do it in the unlocked state.
There is no way a client of your library should be able to construct a locked box without the box being unlocked first.
Lastly, you need a way to duplicate (clone) the box in both states, this should keep the same state as the original box.
All of this has to be checked at compile time, no `instanceof` spaghetti.

Flexibility in the challenge: you can either mutate your box or create a new one every time you lock/unlock it.

### Rust example

The Rust code is provided as an example under `src/lib.rs`, you can run it with `cargo run`.
