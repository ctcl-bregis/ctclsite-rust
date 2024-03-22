# ctclsite-rust v1 - 2024
On February 18, 2024, a rewrite continuing to use Actix was started and the first release was on March 3, 2024.

## 1.5.1 - March 21, 2024

Changes:
- Scrollbar fixes, again
- Made the play button look less like Analog Devices and match the pixel theme
- Fixes to clientinfo.js
- Canvas fingerprints are now hashed server-side

## 1.5.0 - March 20, 2024

Additions:
- Add support for icons on project page entries
- Play/pause button for videos on the about page

Changes:
- Fix website embed on services such as Discord
- Template consolidation
- build.rs now minimizes all .js files found under src/js/
- Remove unused imports

## 1.4.3 - March 17, 2024

Additions: 
- Add redirect: "/projects/nonmonolithic/" to "/projects/nonmono/"
- JavaScript Console message moved to header.html and header_nonav.html from navbar.html

Changes:
- Fixed scrollbar colors
- SCSS fixes

## 1.4.2 - March 15, 2024

Additions:
- Added current URL path to client-side logging

Changes:
- Logger module cleanup

## 1.4.1 - March 14, 2024
This update covers a small feature that I missed during the last release

Additions:at
- Binding IP and port is now specified in config/config.json instead of being hardcoded. IP is stored as a string, port is stored as u16.

Changes:
- Removed public declaration from GlobalCfg fields in main.rs as they are unneeded
- Code cleanup

## 1.4.0 - March 13, 2024
This update mostly covers the loading of website configuration data into memory before use.

Additons:
- BCC TC link list page implemented
- Website configuration data is now stored in memory instead of being read from disk on every page load. This decreases file accesses on the server side siginficantly.
- Custom scrollbars that follow the current page theme

Changes:
- Restructuring of structs used for configuration data

## 1.3.1 - March 10, 2024

Additons:
- --content-only flag for mkrelease

## 1.3.0 - March 6, 2024
This update adds favicons to the website. 

By default; if unspecified, the icons are a square with the color of the page theme.

Additions:
- Page favicons

## 1.2.0 - March 5, 2024
With this update, the website is uploaded manually and the web server no longer clones/pulls changes directly from GitHub.

Additions:
- mkrelease shell command: compiles ctclsite then gathers all of the needed files into a .tar.gz file
- JavaScript files are now minimized then copied to static during mkrelease

Changes and Fixes:
- src/routes/logger/mod.rs and clientinfo.js updated to have "snake case" names for JSON keys to resolve Rust serde compiler warnings
- Code optimizations including removing unused imports
- JavaScript files are no longer linked and are now embedded in the page head
- SCSS and JavaScript files are now under directories within src/
- SCSS fixes
- Fixed "theme-color" and "msapplication-TileColor" meta tags having the value "[object]"
- HTML figure tags in Markdown files not being rendered

Removed:
- "Lite" version of the website
- Local IP grabbing by clientinfo.js

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
