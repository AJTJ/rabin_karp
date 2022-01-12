rabin karp theory:
https://www.geeksforgeeks.org/rabin-karp-algorithm-for-pattern-searching

excellent video
https://www.youtube.com/watch?v=qQ8vS2btsxI

interesting article, doesn't use modular arithmatic though
https://www.jrgould.com/posts/rabin-karp-rolling-hash-string-search-algorithm

article includes horner's method
https://medium.com/swlh/rabin-karp-algorithm-using-polynomial-hashing-and-modular-arithmetic-437627b37db6

More on Horner's method
https://en.wikipedia.org/wiki/Horner%27s_method

## TODO
- Perhaps upgrade the hash function used
  - Look into the original Rabin-Karp fingerprint

### requirements
- the hash much be efficiently computable from the current hash value and the next character in the text
- i.e.
`hash(txt[s+1 .. s+m])` 
must be computable from 
`hash(txt[s .. s+m-1])`
- rehash must be a 0(1) operation

### formula
```
hash = ((p[0] * 10^l) + (p[1] * 10^l- 1).. (p[n] * 10^0)) % v

// also

hash[s + 1.. s + m] = 
hash[s.. s + m - 1] - (s * 10^p) + m] 
```
where
p = the pattern
n = possible length of the pattern
l = the length of the pattern - 1
v = some value to modulo against to avoid overflow

Updated using Horner's method: minimizes multiplications and thus potential overflow

```
hash = p[0] + 10(p[1] + 10(p[2] + ... + 10(p[n - 1] + 10 * p[n]))
```

### General notes
- Hash collision resulting in mis-match: "spurious hit"