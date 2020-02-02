# TODOfile.rs
TODOfile.rs is a simple todo list utility I made for myself.

## How to use
Create a todo file in your home directory.

``touch ~/todo``

Write your tasks one per line

```
turn on the light
get the trash out
buy some milk
```

Prioritize your tasks by appending one or more ``+`` at the end of a task.


```
turn on the light +
get the trash out +++
buy some milk
```

Execute the program

``./todofile``

Output: 

```
get the trash out
turn on the light
buy some milk
```