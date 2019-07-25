# mover

Organize files with common names and group them into forlders



    Example:

    lets say you have the following Files in some folder:

    ```
    file 1.txt  file 2.txt  file 3.txt
    ```

    if you cd into this directory and call `mover file` a folder "file" will be created and all files starting with "file" will be placed there, while also removing that prefix from their respective names.

    the result might look like this

    ```
    > ls 
    1.txt file 2.txt file 3.txt
    > mover file
    > tree
    .
    └── file
        ├── 1.txt
        ├── 2.txt
        └── 3.txt

    1 directory, 3 files
    >
    ```
    


