# File-lock Utility 
Easy file encryption with a password. Uses the ChaCha20-Poly1305 method.

## Usage:  
```
lock [OPTIONS] <PATH>
```
#### Options:
- -**p &lt;PASSWORD&gt;** Enter a password to accompany your request. WARNING: if locking a file, will not check input matches your intended password!
- **-l** Specify that you intend to lock the file. If neither  -l or -u are present, this is the default
- **-u** Specify that you intend to unlock the file
- **-P, --preserve** Do not replace the original file with the locked, and keep both
- **-h, --help** Print this documentation
- **-V, --version** Print cargo version information

## Examples
Open the [examples folder](https://github.com/mrodz/file-secret/tree/master/examples) to view example command usage via the `first_command.bat` and `second_command.bat` Batchfile scripts, or run a tutorial program via `tutorial.bat`.