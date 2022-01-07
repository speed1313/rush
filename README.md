# rush

rush is a toy shell in rust.

# How to implement rush

1. print prompt.
2. read the input line.
3. separate the input line by whitespace.
4. bear a child process and the process executes command with some arguments.
(if command is "cd", bearing a child proocess is useless, so rush should execute it by itselves.).
5. loop to 1.

