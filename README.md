# Cortado

_Compact & focused — fewer surprises, more clarity._

---

# What's that?

So — a while back I ran into Perl and Raku and thought, wow, these languages are beautiful. But Raku sometimes gets so noisy that I end up tearing my hair out over the syntax. I’d love a language that has the expressive, delightful bits of Raku but with a much simpler layout and fewer weird edge cases — basically what I imagine Matz might do if he wanted Raku’s power without the clutter. I’m starting by writing an interpreter and, if things go well, I’ll keep the design friendly to a future LLVM backend.

---

# What I want Cortado to be

- **Pretty, not painful.** Keep the lovely expressive features that make coding fun, but ditch the punctuation gymnastics that make me cry.
- **Compact syntax.** Readable by default — fewer ceremony tokens, clear defaults.
- **Pipelines first.** Nice, composable ways to transform streams of data — think practical scripts that are pleasant to read.

# Okay, what does Cortado looks like?

```raku
method calculate-factorial(n) {
    given n {
        when it < 2 => 1
        default => n * factorial(n - 1)
    }
}

10.calculate-factorial.print
```

## What’s going on

- `method calculate-factorial(n) { ... }` — defines a callable named calculate-factorial. In Cortado this method form is callable in the method-style you used (so 10.calculate-factorial passes 10 as the argument). Note that `-` is allowed to be a part of the function name (same goes for `?` and `!`).

- `given n { … }` — a match block: n is the subject for the following when/default clauses.

- `when it < 2 => 1` — pattern/predicate branch; `it` is the implicit subject inside when. The => returns the expression on the right as the result of the given block when the predicate matches.

- `default => n * factorial(n - 1)` — fallback branch; also returns its expression when no when matches.

- `10.calculate-factorial.print` — method-style call on the literal 10: call calculate-factorial with 10, then call .print on the result. No parentheses needed — chaining reads top-to-bottom like natural prose.
