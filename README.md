# bee

A busybox-like binary on steroids.

# Features

* Single static binary
* Includes common functionality used in shell scripts
* Supports YAML/JSON input and output for better composition with automation
* Uses the busybox pattern, if a command is symlinked it will run the base command
* Backwards compatible with most Linux environments

# Command line

--json - uses json or yaml with stdin to handle the command, see the *JSON* section

--yaml - equivalent to json

symlinks - create a ./bin folder with all commands symlinked back, to create a functional Linux like environment
(command name) - run the specified command, taking and passing all arguments directly to the command

# JSON

All commands can take JSON or YAML if prefixed with the --json flag. This changes the format that commands accept and return information, so the intent of the command is more clear. For example, any ambiguity over spaces or attempting to parse commands is more explicit.

Input: `{"stdin": base64, "args": []{string: string}}`

Output: `{"stdout": base64, "stderr": base64, "rc" int8}`

# Modules to Implement

## GNU

### file attributes (all consolidated to one command)

* chcon change security context
* chgrp change group ownership
* chown change file ownership
* chmod change permissions
* touch changes file timestamps, can create a new file

### file handling (all consolidated to one command)

* cp copy a file to path (consolidated with rsync like functionality)
* dd copy and convert a file
* dircolors (not implemented, a part of the binary config for --color)
* install
* ln create a symlink to a file
* mv move a file
* rm remove files, directories, device nodes, and symbolic links
* rmdir removes an empty directory
* shred overwrites a file to hidei ts contents, optionally delete
* truncate srhink or extend a file to a specific size

### file information

* dir equivalent to ls -C -b (consolidated with ls)
* df show disk information
* ls list files in a directory
* realpath returns resolved absolute or relative path
* vdir equivalent to ls -1 -b

### Other file

* mkdir make a directory
* mkfifo make named pipes
* mknod makes block or character special files
* mktemp create temporary file or directory
* sync flushes file system buffers

### Text utilities

* b2sum computes and checks blake2b message digest
* base32 encodes or decodes base32 and prints to stdout
* base64 encodes or decodes base32 and prints to stdout
* cat concatenates and prints files on stdout
* cksum checksums and count the byte in a file
* comm compares two sorted files line by line
* csplit split a file into sections determined by context lines
* cut removes sections from each line of files
* expand converts tabs to spaces
* fmt simple optimal text formatter
* fold wraps each input line to fit in specified width
* head outputs the first part of files
* join joins lines of two files on a common field
* md5sum computes and checks md5 checksum
* nl numbers lines of files
* numfmt reformat numbers
* od dumps files in octal and other formats
* paste merges lines of files
* ptx produces a permuted index of file contents
* pr converts text files for printing
* sha1sum,sha244sum,sha256sum,sha384sum,sha512sum computes sha checksums
* shuf generate random permutations
* sort sorts lines of text
* split splits a file into pieces
* sum checksums and counts the blocks in a file
* tac concatenates and reverses order line by line
* tail outputs the last part of files
* tr translates or deletes characters
* tsort performs topological sort
* unexpand converts spaces to tabs
* uniq removes duplicate lines
* wc prints number of bytes, words, lines in files

### Shell utils
* arch prints machine hardware name (uname -m)
* basename removes the path prefix for a pathname
* chroot changes root directory
*  date prints or sets the system date and time
* dirname strips non directory suffex from file name
* du shows disk usage
* echo displays a line of text
* env displays and modifies environment variables
* expr evaluates an expression
* factor factors numbers
* false a false value, rc 1
* groups prints the groups the user is in
* hostid prints numeric identifier of the current host
* id prints the real or effective uid or gid
* link creates a link to a file
* logname prints user login name
* nice modifies scheduling priority
* nohup allows a command to continue running after logging out
* nproc queries number of active processors
* pathchk check whether file names are valid or portable
* pinky lightweight version of finger
* printenv prints environment variables (consolidated with env)
* printf formats and prints data
* pwd prints the current workign directory
* readlink displays value of symlink
* runcon run command with specified security context
* seq prints a sequence of numbers
* sleep wait for specified amount of time
* stat returns data about an inode
* stdbuf controls buffering for commands that use stdio
* stty changes and prints terminal line settings
* tee send output to multiple files
* test evaluates an expression
* timeout run a command with a time limit
* true a true value, rc 0
* tty prints terminal name
* uname prints system information
* unlink removes specified file using the unlink function
* uptime how long the system has been running
* users prints the user names of users currently logged in
* who prints a list of all users logged in
* w alias of who
* whoami prints the effective userid
* yes prints a string repeatedly
* [] test an expression

## Syntax

if, when, and data manipulation are implemented both as they are with a POSIX environment or with improved syntax

## Supplementary

* curl/wget get http files
* jq json parser
* grep implements similar to ripgrep
* supervisor an alternative to nohup that will spawn similar to systemd, with a given path for supervise state, logs, and tmp
* metrics provides a comprehensive list of metrics for the system, cpu, disk, io, etc, can be subqueried
* sysinfo get comprehensive system informatin, similar to ohai or facter, can be subqueried
