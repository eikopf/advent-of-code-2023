# Day 8
## Question 1
I was slightly spooked by the idea that I would have to implement a graph in rust, but luckily this question basically just requires a hashmap. Traversing the graph is pretty trivial, so there's nothing clever to do here.

## Question 2
I originally tried to write a bruteforce solution, but it occurs to me that that would take ages and be basically pointless. Instead, consider that we can run each traversal up until the point that they loop back to their initial values.

> The assumption here is that each node is a component of a (relatively) short cycle, such that we only have to concern ourselves with a single loop through each of them. I'm also going to assume that each cycle only has one solution, otherwise this calculation is significantly more painful.

On the assumption that these are all cycles, we instead get the list of terminal indices $t_i$ for each node and calculate their prime factors $S_i$. Finally, we just remove duplicates and calculate the product of these factors, such that our result is the lowest number which can be factored into the terminal indices of the given nodes.
