This document applies to pages that make use of the "sections" layout, currently that would be "Projects" and "Blog".

A "section" is a `<section></section>` element block that either holds content or links to other pages.

A "pagebutton" is a bunc of `<div>` blocks within an `<a>` block that acts as a link to another page.

# Sections of Pagebuttons
The implementation of pagebuttons is based off the HTML and styling used for the blog at the [Studio Minus website](https://studiominus.nl/).

Hierarchy: `class name - HTML tag name`

- pagebutton - a
  - pagebutton-title - span
  - pagebutton-tags - span
  - pagebutton-desc - span
  - pagebutton-date - span
  - pagebutton-img - div