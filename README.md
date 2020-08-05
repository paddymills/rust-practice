# rust-practice
code for learning rust

## Necklace Counting

As said on [r/dailyprogrammer](https://www.reddit.com/r/dailyprogrammer/comments/g1xrun/20200415_challenge_384_intermediate_necklace/):

*a k-ary necklace of length n is a sequence of n letters chosen from k options, e.g. `ABBEACEEA` is a 5-ary necklace of length 9. Note that not every letter needs to appear in the necklace. Two necklaces are equal if you can move some letters from the beginning to the end to make the other one, otherwise maintaining the order. For instance, `ABCDE` is equal to `DEABC`. For more detail*

Essentially, k is the index of the highest valid letter.

1. necklace parsing
2. necklace generation (`necklaces(k, len)`)