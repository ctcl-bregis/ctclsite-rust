ctclsite-rust changelog

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


