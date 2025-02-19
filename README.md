
## carrot checker

Carrot checker analyzes Calpads files to see if they have the proper number of fields.



function steps which work will work on files or directories of files:

* Need to check the file for bad UTF-8 characters. program will fail if it runs into a single bad
* character.
* steps = open the file 
* read the file into a vec of u8 (bytes) 
* convert the bytes to a string
* with a lossy function that will replace bad utf8 with ?.
* Split the string into lines
* get the filetype from the first 4 chars of the 1st line.
* if the chars don't correspond to a rule return an error as the file is out of spec
* Split the lines by Delimiters (carrot ^)
* compare the splits with the rule in the hashmap for each line.
* If there is ever a point where columns are more or less then they should be 
* return an error message.


Column rules will need to be adjusted manually for now. As new file specs get released change lines 67 to 85 that correspond with the rule type.

### known issues:

