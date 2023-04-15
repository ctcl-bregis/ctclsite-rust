ctclsite-rust changelog

## 0.2.1

Changes:
- Updated dependency versions in Cargo.toml

## 0.2.0
Major release with the main feature being the addition of the "Projects" page.

Changes since 0.1.1:
- "Projects" page
- CSS improvements
- Improvements to "Blog"
- RAMList removed from navigation bar
- General code improvements

## 0.1.1
First release after the official release of the Rust version of the website.

Changes since 0.1.0:

- Removed unused variable outlined in a warning during compilation
- Fixed repeating background CSS for About/Welcome page.


## 0.1.0
First release of the Rust version of the website

General changes from ctclsite (Python) 2.1.4:

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
