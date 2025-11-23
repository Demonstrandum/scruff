import os as _
import sys as unused


# This example demonstrates that explicit configuration 
# always overrides mode defaults

# Tali mode normally uses symbol quotes (single quotes for identifiers)
# But we explicitly override it with quote-style = "double"

config_name = "database_url"     # would be 'database_url' in tali mode
api_key = "prod_key"             # would be 'prod_key' in tali mode  
service_name = "user_service"    # would be 'user_service' in tali mode

# Natural language strings stay double quoted in both cases
message = "Hello, welcome to the application!"
greeting = "Thanks for using our service!"

# Strict mode normally includes many rule categories
# But we explicitly override with select = ["F"] (only pyflakes)
# So unused imports above would normally be caught by strict mode,
# but our explicit select only enables pyflakes rules