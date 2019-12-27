# fuzz-lazy-static

This is a demonstration that (1) using AFL in persistent mode to fuzz a program with `lazy_static`
variables causes problems, and (2) adding a `reset` function to `lazy_static` addresses those problems.

* Try `cargo afl build && make fuzz` and notice AFL's abismallly low stability reports.  Then try
`cargo afl build --features=reset && make fuzz` and notice how the stability reports are close to 100%.

* Try `cargo afl build --features=delay && make fuzz` and notice how AFL fails to report a timeout.
Then try `cargo afl build --features='delay reset' && make fuzz` and notice how a timeout is reported.

A quick note regarding the corpus: the above failure to report a timeout relies on `corpus/f7fc5f1...`
being processed after all other files within the corpus.
