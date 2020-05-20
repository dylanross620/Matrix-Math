# Matrix-Math
Interpreter to store matrices and perform operations on them

## How to use:
- To create a matrix, type `let <matrix name> = <num rows> <num cols>`. You will then be prompted to enter enough values to fill the matrix, which will fill in row-wise (similarly to reading a book)
- To print an existing matrix, type `print <matrix name>`
- To add, subtract, or multiply 2 matrices, type `<mat 1 name> <+/-/*> <mat 2 name>`. This will provide an error message if the matrices are not of the correct sizes
- To find the inverse of a matrix, type `<inv/inverse> <mat name>`
- To find the reduced row-echelon form of a matrix, type `rref <mat name>`
- To generate an n x n identity matrix, type `identity <n>`
- The results from the above can be saved into a new matrix. For example, to create the matrix `C = A + B`, you would type `let C = A + B`. If the result is not saved to a variable, it will be printed instead
