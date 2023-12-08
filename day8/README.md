# Day 8
## Question 1
I was slightly spooked by the idea that I would have to implement a graph in rust, but luckily this question basically just requires a hashmap. Traversing the graph is pretty trivial, so there's nothing clever to do here.

## Question 2
I originally tried to write a bruteforce solution, but it occurs to me that that would take ages and be basically pointless. Instead, consider that we can run each traversal up until the point that they loop back to their initial values.

> The assumption here is that each node is a component of a (relatively) short cycle, such that we only have to concern ourselves with a single loop through each of them. I'm also going to assume that each cycle only has one solution, otherwise this calculation is significantly more painful.

This gives us a list of cycle lengths, and we also want to know the indices of the terminal nodes in each cycle. Call the cycle lengths $L_i$ and the terminal indices $t_i$ for the cycle $i$; then in particular we know that the set of solutions for cycle $i$ is $$\text{Sol}_i=\{n\in\mathbb{N}_0:t_i\equiv n\pmod L_i\}.$$
