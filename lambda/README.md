Simple Lambda-Calculus REPL
===

This is very simple indeed. It has lesser than 500 lines of code. In contrast,
[the super tiny compiler](https://github.com/jamiebuilds/the-super-tiny-compiler/blob/master/the-super-tiny-compiler.js) has 1053 lines of bloat!
(Although half of them are comments)

It works. For instance, you type `\x.\y.x` for the Church boolean TRUE.
You can define variables, and evaluate expressions.

# Example
A real session:

```
No copyright (CC0) 2026 Youzhe Zhen
type `help` for helps, `exit` to exit
>>> ls
>>> help
This is a simple lambda calculus interpreter.
eval, e, n or nf <expression>    reduct, normal order
 cn or whnf <expression>         reduct, call-by-name
set or s <variable> <expression> define a variable
cat or c <variable>              look at its value (unevaluated)
       ls                        list all defined variables
       std                       load the standard liberary
       help                      show this message
       exit                      exit the interpreter
>>> std
>>> e not true
(\x. (\y. y))
>>> e not (not (false
(\x. (\y. y))
>>> set 2 succ 1
2 = ((\n. (\f. (\x. (f ((n f) x))))) ((\n. (\f. (\x. (f ((n f) x))))) (\f. (\x. x))))
>>> e 2
(\f. (\x. (f (f x))))
>>> e mul 2 2
(\f. (\x. (f (f (f (f x))))))
>>> e pred (mul 2 2
(\f. (\x~5. (f (f (f x~5)))))
>>> e fst (pair 3 4
3
>>> e snd (pair 3 4
4
>>> ls
fst = (\p. (p (\x. (\y. x))))
snd = (\p. (p (\x. (\y. y))))
is0 = (\n. ((n (\x. (\x. (\y. y)))) (\x. (\y. x))))
pred = (\n. ((\p. (p (\x. (\y. x)))) ((n (\p. (((\a. (\b. (\s. ((s a) b)))) ((\p. (p (\x. (\y. y)))) p)) ((\n. (\f. (\x. (f ((n f) x))))) ((\p. (p (\x. (\y. y)))) p))))) (((\a. (\b. (\s. ((s a) b)))) (\f. (\x. x))) (\f. (\x. x))))))
pair = (\a. (\b. (\s. ((s a) b))))
succ = (\n. (\f. (\x. (f ((n f) x)))))
0 = (\f. (\x. x))
1 = ((\n. (\f. (\x. (f ((n f) x))))) (\f. (\x. x)))
add = (\m. (\n. (\f. (\x. ((m f) ((n f) x))))))
not = (\b. (\x. (\y. ((b y) x))))
mul = (\m. (\n. (\f. (m (n f)))))
true = (\x. (\y. x))
Y = (\g. ((\x. (g (x x))) (\x. (g (x x)))))
false = (\x. (\y. y))
2 = ((\n. (\f. (\x. (f ((n f) x))))) ((\n. (\f. (\x. (f ((n f) x))))) (\f. (\x. x))))
>>> exit
```
