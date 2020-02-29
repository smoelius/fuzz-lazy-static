.PHONY: all fuzz

all:
	@echo "Try 'cargo afl build --features=\"FEATURES\"', where FEATURES is"
	@echo "some subset of {delay, panic, reset}.  Then try 'make fuzz'."

fuzz:
	cargo afl fuzz -i corpus -o syncdir -- target/debug/fuzz_lazy_static
