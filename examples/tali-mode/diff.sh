#!/bin/sh

cmd="uv run scruff check --diff"
echo ';' $cmd
$cmd

cmd="uv run scruff format --diff"
echo ';' $cmd
$cmd


