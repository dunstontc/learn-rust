# [Common Collections](https://doc.rust-lang.org/book/second-edition/ch08-00-common-collections.html)

- A *vector* allows you to store a variable number of values next to each other in memory.
- A *string* is a collection of characters. 
- A *hash map* allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a *map*.
- [std::collections](https://doc.rust-lang.org/std/collections/index.html)


## Summary

> Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:

- Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
- Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
  - Keep in mind the details about UTF-8 encoding!
- Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” i
  - Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!
