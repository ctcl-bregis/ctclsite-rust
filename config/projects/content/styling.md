This document covers the how I style code, writing and other media. This document contains personal opinions. 

## Time and Date
When possible, ISO standard 8601 is used. As part of ISO 8601, weeks start on Monday. Dates are written in the format YYYY-MM-DD.

## Code
This section covers how I style code. Some inspiration is taken from the style guides from [suckless](https://suckless.org/coding_style/) and [Google](https://google.github.io/styleguide/cppguide.html#Punctuation,_Spelling_and_Grammar).

This style guide covers the following languages and formats:
* Bash
* C/C++
* C#
* CSS/SCSS
* HTML
* JavaScript
* JSON
* Lua
* OpenSCAD
* Python
* Rust

### Indentation
If possible, I use four (4) spaces for indentation. This includes but is not limited to: Bash, C/C++, C#, CSS/SCSS, HTML, JavaScript, JSON, Lua, OpenSCAD, Python and Rust.

### Function Declaration
The open curly bracket is always on the end of the function declaration and not on a new line. Due to how functions are defined in Python and Lua, this does not apply to Python and Lua.

For example, in **Rust**:

**Do**
```rs
pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path))),
            _ => panic!("Error reading from file {}: {}", path.to_owned(), e),
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}
```

**Don't**
```rs
pub fn read_file(path: &str) -> Result<String, Error> 
{
    let mut file = match File::open(path) 
    {
        Ok(file) => file,
        Err(e) => match e.kind() 
        {
            std::io::ErrorKind::NotFound => return Err(Error::new(std::io::ErrorKind::NotFound, format!("File {} not found", path))),
            _ => panic!("Error reading from file {}: {}", path.to_owned(), e),
        }
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    Ok(buff)
}
```

### Comments
Within recent times; since 2023, I have been trying to have comments more useful and only present when they are needed.

## UI and Messages

### Addressing the user
The software shall not use first person terms or "act" as if it were to be sentient.

UI elements, log messages and code should generally avoid these terms:
* I
* I am
* I'm
* Me
* Myself
* Please
* Sorry

Gender-neutral pronouns are to be used when referring to the user or someone else. 