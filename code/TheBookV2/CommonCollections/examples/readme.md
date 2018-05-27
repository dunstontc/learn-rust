# [CommonCollections](https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html#summary)

> Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. 
> Here are some exercises you should now be equipped to solve:

1. Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
2. Convert strings to pig latin. 
    - The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
    - Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
    - Keep in mind the details about UTF-8 encoding!
3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
    - For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
    - Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!
