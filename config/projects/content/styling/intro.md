This document covers the how I style code, writing and other media. This document contains personal opinions.

Some inspiration for this document is taken from the style guides from [suckless](https://suckless.org/coding_style/) and [Google](https://google.github.io/styleguide/).

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

## Common Code Guidelines
This section covers conventions used across most programming languages. See the pages linked at the top of the page for styling for specific programming languages.

### Indentation
When possible, indentation shall be done with **four spaces**.

Many IDEs have the feature where the "Tab" key inserts four spaces. 

### Comments
In recent times, I have been trying to have comments more useful.

Like what is stated in [Google's style guide for Python]((https://google.github.io/styleguide/), comments shall not describe the code; assume however is reading the code has extensive experience with the programming language. Comments are to be used to explain why a certain thing is done in a certain way. 

## UI and Messages
Messages from the software to the user shall get to the point and be descriptive.

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
I believe that the terms "Slave/Master" did not really make that much sense to begin with. It was never a US-specific issue. USB seemed to have used Host/Device well before the 2020s and that seems to be most suitable option though it only properly fits electronics engineering. Some cases, "device" is already used to describe a different part. 

Master/Slave may continue to be used where it is to be consistent with existing documents. This includes the use of MISO/MOSI for SPI signal names.

### Whitelist/Blacklist
Projects will continue to use the terminology "Whitelist/Blacklist".

## Names

### Software Names

#### Lua
I've seen Lua spelled as "LUA" like it is an acronym. It is not an acronym; it is the Portugese word for "moon".

When referring to the language, only the "L" shall be capitalized.

#### coreboot
As stated in the [coreboot documentation](https://doc.coreboot.org), coreboot shall be referred to as "coreboot" without any capital letters or spaces.

### Company Names

#### SK hynix vs SK Hynix
Technically both "SK Hynix" and "SK hynix" are correct spelling of the name however the latter is preferred. 

#### Innolux vs InnoLux
Though the company refers to itself by either as seen in datasheets. The spelling "Innolux" would be used. 
