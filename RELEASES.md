# ctclsite-rust v1 - 2024
On February 18, 2024, a rewrite continuing to use Actix was started

## 1.2.0 - [UNRELEASED]
To-Do:

- JavaScript files are no longer linked and are now embedded in the page head
- JavaScript is now minimized
- Accessed configuration files and content are now cached in memory

## 1.1.0 - March 3, 2024
Initial release

### Changes from ctclsite-python v5
Other than the switch from Python Django and Rust Actix bringing significant performance improvements.

- "Lite" version of the website temporarily unimplemented
- Server request logging temporarily removed
- SCSS modified per theme split into a separate file called theme.scss, this should decrease SCSS compilation time
- Some modifications to SCSS files required for the rewrite
- Some modifications to JSON files required for the rewrite
- Blog icons are now larger on screen

# ctclsite-rust v0 - 2022-2023

## 0.3.0 - April 27, 2023
Logging Update

Additions:
- Rust application would now log requests and store the logs in the "log" directory

## 0.2.1 - April 15, 2023

Changes:
- Updated dependency versions in Cargo.toml

## 0.2.0 - February 26, 2023
Major release with the main feature being the addition of the "Projects" page.

Changes since 0.1.1:
- "Projects" page
- CSS improvements
- Improvements to "Blog"
- RAMList removed from navigation bar
- General code improvements

## 0.1.1 - January 13, 2023
First release after the official release of the Rust version of the website.

Changes since 0.1.0:

- Removed unused variable outlined in a warning during compilation
- Fixed repeating background CSS for About/Welcome page.

## 0.1.0 - January 13, 2023
First release of the Rust version of the website

General changes from ctclsite-python 2.1.4:

- Rewritten entirely in Rust, mainly utilizing actix-web and Tera
- Smaller directory count
- Updates from GitHub, allowing for easier user contributions (pull requests)
- Blog is easier to maintain
- ramlist.css and main.css merged
- Contact, About and Main pages merged
- CSS and HTML minification to reduce the amount of data sent to the browser
- More customization
- Logging is removed and is now handled by the upstream webserver, may it be apache2 or nginx.

RAMList changes:

- Significant performance improvement due to using Rust over Python for the backend
- List pages are now themed the same color as the menu button
- Removed "Last Updated" due to the date changing every time the website is updated. "Last Updated" may be added in a future update once a more reliable way to keep track of modification times.
