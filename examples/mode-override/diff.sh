#!/bin/sh

echo "=== This example demonstrates explicit config overriding mode defaults ==="
echo ""
echo "Tali mode normally:"
echo "  - Uses symbol quotes (single quotes for identifiers)" 
echo "  - Uses minimal rule set (pyflakes + E4xx + E9xx)"
echo ""
echo "But our explicit config overrides both:"
echo "  - quote-style = \"double\" (overrides symbol quotes)"
echo "  - select = [\"F\"] (overrides minimal rule set)"
echo ""

cmd="uv run scruff check --diff"
echo ';' $cmd
$cmd

cmd="uv run scruff format --diff"
echo ';' $cmd  
$cmd