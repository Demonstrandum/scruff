# quoting convention: symbols

fine_ident = 'my-symbol'  # single quotes around "identifiers"
not_fine_ident = "my-symbol"  # symbols/idents should be single quoted

# custom regex for what constitutes a symbol

not_a_symbol = 'xyz;abc'
is_a_symbol = 'xyz|abc'  # only because we allowed `|` in our config

empty = ''  # for example, our regex does not match empty strings

# natural language sentences should be double quoted

fine_sentence = "Hello, World!"
not_fine_sentence = 'Hello, World!'

# f-strings are respected

symbolic = "Even if this isn't"
fine_fstring = f'still_{symbolic}'
not_fine_fstirng = f'Not {symbolic}'

# Does not apply to triple quote strings

not_fine_docstr = '''hello_there'''
