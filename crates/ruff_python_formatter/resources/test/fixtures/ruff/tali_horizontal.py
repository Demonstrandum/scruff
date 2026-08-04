# Assignment alignment is preserved and normalized only when the source signals intent.
short   = 1
long_name = 2

plain = 3
other = 4

# Blank lines and non-assignment statements end alignment groups.
left    = 1

right = 2
print(right)
after   = 3
another = 4


# Annotation content and annotated values align independently.
x:    int       = 1
long: str = 2

first:   bytes
second: str


# A wide source gap opts a consecutive statement family into comment alignment.
a=1        # first
long_name=2  # second

unaligned = 3  # this run has no alignment signal
also_unaligned = 4  # and remains conventionally spaced

# Blank lines and statement-family changes prevent unrelated comment alignment.
separate = 1      # separate

print(separate)      # unrelated


def nested():
    small    = 1    # nested first
    longer_name = 2  # nested second


# The comment does not make the code wrap; an overflowing comment moves below as a pointer.
result = some_function(argument)  # This deliberately long explanation makes the complete source line exceed the configured formatter width.

# Pragmas must remain attached to the statement even when the source line is long.
pragma_result = some_function(argument)  # noqa: E501, F401, F841, E722, E731, PLR0911, PLR0912

# Chained and augmented assignments are intentionally outside horizontal alignment.
first = second    = value
counter    += 1
