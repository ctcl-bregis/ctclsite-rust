Currently, SLAG makes use of CouchDB for database storage. 

## Structure
Each module gets its own "Document" in the database. 

### Module-specific

#### Micron
Currently, the only data that the Micron module stores are the results of an FBGA code search.

It is highly unlikely that the part number associated with an FBGA code would change so when SLAG requests the part number for a specific FBGA code, it is stored in the database indefinitely. This feature could siginficantly decrease the amount of HTTP requests to micron.com.

#### Users
The "Users" module is expected to store the most amount of data and have the most complex schema. 

**Details TBD**

#### Tags
The "Tags" module stores text and/or media associated with a user-defined tag

This feature should just consist of a single key-value dictionary. 
