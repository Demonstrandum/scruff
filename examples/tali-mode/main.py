import os as _


# one thing about mode="tali" is the quoting convention

fine_ident = 'my-symbol'  # single quotes around "identifiers"
not_fine_ident = "my-symbol"  # symbols/idents should be single quoted

# natural language sentences should be double quoted

fine_sentence = "Hello, World!"
not_fine_sentence = 'Hello, World!'

# We should allow defying the above rules
# when the string contains a quote (to avoid escaping)


fine = 'Hello, "World"!'
fine = '''Hello "World"!'''
