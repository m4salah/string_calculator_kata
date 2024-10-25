# String Calculator Kata tests

1. Create a simple String calculator with a method
    - int Add(string numbers) // Function signature
    - The method can take 0, 1, or 2 comma-separated numbers, and will return their sum
        - for an empty string it will return 0
        - for example "" or "1" or "1,2.
    Start with the simplest test case of an empty string and move to 1 and two numbers
    Remember to solve things as simply as possible so that you force yourself to write tests you did not think about
    Remember to refactor after each passing test // Done
2. Allow the Add method to handle an unknown amount of numbers // Done
3. Allow the Add method to handle new lines between numbers (instead of commas).
    the following input is ok:  “1\n2,3”  (will equal 6)
    the following input is NOT ok:  “1,\n” (not need to prove it - just clarifying)

4. Support different delimiters
    to change a delimiter, the beginning of the string will contain a separate line that looks like this:   “//[delimiter]\n[numbers…]” for example “//;\n1;2” should return three where the default delimiter is ‘;’ .
    the first line is optional. all existing scenarios should still be supported
5. Calling Add with a negative number will throw an exception “negatives not allowed” - and the negative that was passed.if there are multiple negatives, show all of them in the exception message stop here if you are a beginner. Continue if you can finish the steps so far in less than 30 minutes.
6. Numbers bigger than 1000 should be ignored, so adding 2 + 1001  = 2
7. Delimiters can be of any length with the following format:  “//[delimiter]\n” for example: “//[***]\n1***2***3” should return 6
8. Allow multiple delimiters like this:  “//[delim1][delim2]\n” for example “//[*][%]\n1*2%3” should return 6.
9. make sure you can also handle multiple delimiters with length longer than one char

> We will divide the problem into smaller peaces.
> We will work on the first point only
> After writing the test in the this md file
> Red: failing test,
> Green: make the minimum to make the test passes,
> Refactor the function.

## 1

- Empty string produces 0, "" -> 0
- One number string return it,
  - "1" -> 1, "0" -> 0, "2" -> 2,.., and so on
- Two or more number string return the sum of them,
  - "0, 1, 2" -> 3, ...and so on

## 2

- The method takes arbitrary string length.

## 3

- Allow the method to take new line as a delimiter
  - "1\n2,3" -> 6
- NO consecutive delimiter
  - "1\n,2" -> Err(ConsecutiveSeparators)

## 4

- Supporting different delimiters by this operation "//;\n1;2":
  - "//;\n1;2" -> 3
  - it's optional to provide custom serperator so all the previous tests must pass

## 5

- If any negative numbers return it as error.
  - "//;\n1;-2" -> Err([-2])
  - "1, 2, -3, -4" -> Err([-3, -4])

## 6

- If any negative numbers return it as error.
  - "//;\n1;-2" -> Err([-2])
  - "1, 2, -3, -4" -> Err([-3, -4])

## 7

- number with bigger than 1000 should be ignored
  - "//;\n1;1002" -> Ok(1)
  - "1, 2000" -> Ok(1)
