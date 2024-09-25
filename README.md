# The Optim Sort    
#### Introduction   
This is a sorting algorithm leveraging merge sort, insertion sort 
and binary search design techniques combined together, to solve sorting problem 
taking advantage of all their advantages from small to large datasets.      

#### How it works     
>1.**Applying the divide and conquer approach of merge sort procedure**, we divide 
the problem(our array input) into subproblems; i.e n/k subproblems.    
>Where: *n is the number of elements in the array, and k is the desired length of the subproblems*   

>------ k is carefully chosen, that is, when insertion sort beats merge sort, *2<=n<=43* -------   

>2.**We perform an insertion sort procedure at length k, that is 2<=n<=43**  
>At this step, the problem is nolonger being divided into subproblems but sorted via insertion sort which runs faster.  
>Consider: *insertion sort running at 8n^2 and merge sort running at 64(nlgn) and log is to the base 2.*
>3.**Searching for insertion positions**. Here we replace the linear scanning for position lookups of 
sorted array A[1..j-1] to insert A[j] in insertion procedure above with binary probing technique to exploit 
the log~2~n running time when finding the positions.  
  
>**Merge back the subproblems into one final solution of the overall problem.***
