


## "Page" struct

### ptype
Instead of dealing with enums, how the Page struct is read is based on the value of this field.

#### linklist
"linklist" is a page that has a list of clickable boxes that go to other pages.

#### link
"link" does not define a page and is only to be used to define a link for linklist pages.

#### content
"content" is page that simply shows a rendered markdown document.
