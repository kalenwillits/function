# Function

*The Rust implementation of the [fn c++ repo](https://github.com/kalenwillits/fn_cpp)*

Function is a simple CLI program written in Rust for Linux to manage shell scripts and create
shorthand commands for workflow automations.
See [run-scripts](https://github.com/Kilthunox/run-scripts) for example automations.




## Build from source
### Dependencies
- Install [Rust](https://www.rust-lang.org/tools/install)
- Linux/Unix OS


To install, execute the install below commands or continue on to the "Compile" section.
```
git clone https://github.com/kalenwillits/function.git;
cd function;
./run/install.sh;
```

### Compile
Compile from source using cargo
```
cargo build --release
```


### Install
1. Move the newly compiled run executable to where you want the application to live in it's own directory.
```
mkdir -p ~/Apps/fn/;
cp target/release/fn ~/Apps/fn/fn;
```

2. Expose the `fn` directory to `PATH` in the `.bashrc` file.

`~/.bashrc`
```
...
export PATH=$PATH:~/Apps/fn/
```


3. Create a global automation script.
```
cd ~/
mkdir run
cd run
printf "#!/bin/bash \necho $1" > echo.sh
```

4. Restart the terminal.

5. Verify the installation.
```
fn --verison
```

### Usage
run searches for global scripts first, then local scripts from the working directory override global scripts.
Test if the new echo script was installed by using this script from any directory:
```
run echo HelloWorld
```
-> HelloWorld 


Adding a `run/` directory to the root of your project allows project-specific automations. 

`~/projects/project/run/echo.sh`
```
#!/bin/bash
echo $PWD
```

Test the script from `~/projects/project/.`:
```
fn echo HelloWorld 
```
-> ~/projects/project/

The working directory is printed because the local script has overwritten the global script.

Ensure that all run scripts have a `.sh` file extension. All other files will be ignored.

