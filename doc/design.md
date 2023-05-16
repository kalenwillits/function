# Function

Function(fn) is a command line tool written in rust to assist in the 
organization and execution of bash scripts.


## Config
### env vars
FUNCTION_GLOBAL_DIR
FUNCTION_LOCAL_DIR
FUNCTION_EDITOR
 
## Behavior
Gather all *.sh file names at FUNCTION_GLOBAL_DIR.
for file name in names, cache the name, path, and gather {file name}.toml.
Gather all *.sh file names at FUNCTION_LOCAL_DIR.
for file name in names, cache the name, path, and gather {file name}.toml overwrite cached globals.


parse arguments.

if -h/--help in arguments:
	display each available command from this path and it's tagline

if -n/--new in arguments:
	Start the create-new-function sequence
		1. function name
		2. global/local
		3. open {function name}.sh file with FUNCTION_EDITOR.

if -i/--info {function name} in arguments:
	display function description

else: 
	for arg in arguments:
		execute subprocess(arg)
