This document covers the how I style code, writing and other media. This document contains personal opinions. 

## Time and Date
When possible, [ISO standard 8601](https://www.iso.org/iso-8601-date-and-time-format.html) is used.

### Weeks
Unlike many software set to a US locale, **weeks must start on Monday**. The standard ISO 8601 has weeks start on Monday.

I believe any software dealing with time and date must have the option to set the start of the week to any day of the week. 

### Formats

#### Shorthand - Month name, day, year
Since 2022, I commonly used this format for dates in directory names: `<three letter month><day><year>`

For example:
- backup_may262024
- photos_feb222022

However, starting July 31, 2024, I would start using the numbers-only format described in the following section for this purpose.

#### Shorthand - numbers only
This date format is used in any instance that it is beneficial. This format would be used in file names, directory names, software builds and anywhere a shorthand date is needed. It replaces the format that I commonly used.

The format used is `YYYYMMDD` where:
- YYYY - Year, four-number representation
- MM - Month, two-number representation, single-digit months are to have a leading zero
- DD - Day, two-number representation, single-digit days are to have a leading zero

For example, in file and directory names:
- backup_20240526
- photos_20220222

#### Long form
In text such as documents, commit messages and general communications, this format is preferred:

Day, month and year known:
- `<month name> <day>, <year>`

Month and year known:
- `<month name>, <year>`

Examples:
- "Server backups would commence on August 11, 2024"
- "Planning started in March 2021"


## Code
This section covers how I style code. Some inspiration is taken from the style guides from [suckless](https://suckless.org/coding_style/) and [Google](https://google.github.io/styleguide/cppguide.html#Punctuation,_Spelling_and_Grammar).

This style guide covers the following languages and formats:
- Bash
- C/C++
- C#
- CSS
- Lysine
- HTML
- JavaScript
- JSON
- Lua
- OpenSCAD
- Python
- Rust

### Indentation
If possible, I use four (4) spaces for indentation.

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
Within recent times; since 2023, I have been trying to have comments more useful and only present when they are needed. A lot of times I use comments to tell myself or another contributor that the code is not 

If there must be documentation, the documentation can be done elsewhere outside of code. 

## UI and Messages
Messages from the software to the user shall get to the point while being descriptive.

### Addressing the user
The software shall not use first person terms or act as if it were to be sentient.

UI elements, log messages and code should generally avoid these terms:
- I
- I am
- I'm
- Me
- Myself
- Please
- Sorry

Gender-neutral pronouns are to be used when referring to the user or someone else. 

### Error Messages
Error messages should describe the error clearly while giving sufficient information to resolve the error.

## Terminology
Following 2020, many organizations started to change terminology in documentation and software. 

The changes done to documents have been horribly inconsistent from one vendor to another. In some recent document revisions, especially those from Texas Instruments (see SLVSDV5B Revision B, August 2021), the terminology is inconsistent throughout the document with "Slave", "Peripheral", and "Secondary" being used to refer to the same thing.  

### Slave/Master
I believe that the terms "Slave/Master" did not really make that much sense to begin with. For example, why is the chip or hard drive a "slave"? None of the components are paid anything. USB seemed to have used Host/Device well before the 2020s and that seems to be most suitable option though it only properly fits electronics engineering.

Master/Slave may continue to be used where it is to be consistent with existing documents. This includes the use of MISO/MOSI for SPI signal names.

### Whitelist/Blacklist
Projects will continue to use the terminology "Whitelist/Blacklist".