rabin karp theory:
https://www.geeksforgeeks.org/rabin-karp-algorithm-for-pattern-searching

excellent video
https://www.youtube.com/watch?v=qQ8vS2btsxI

interesting article, doesn't use modular arithmatic though
https://www.jrgould.com/posts/rabin-karp-rolling-hash-string-search-algorithm


- the naive algo would use the sliding window and check every character every time.
- rabin karp would generate the hash, and if the hash matches only then will it check the individual characters

### requirements
- the hash much be efficiently computable from the current hash value and the next character in the text
- i.e.
`hash(txt[s+1 .. s+m])` 
must be computable from 
`hash(txt[s .. s+m-1])`
- rehash must be a 0(1) operation

### one formula

`hash( txt[s+1 .. s+m] ) = (d(hash( txt[s .. s+m-1]) – txt[s]*h ) + txt[s + m]) mod q`
where
hash( txt[s .. s+m-1] ) : Hash value at shift s. 
hash( txt[s+1 .. s+m] ) : Hash value at next shift (or shift s+1) 
d: Number of characters in the alphabet 
q: A prime number 


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


### General notes
- A hash collision is known as a "spurious hit"