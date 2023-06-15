import sys

def fib(n):
    return 1 if n in [1, 2] else fib(n-1) + fib(n-2)

n = int(sys.argv[1])
print(f"fib({n}) = {fib(n)}")