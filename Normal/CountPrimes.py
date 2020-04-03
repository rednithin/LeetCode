class Solution:
    def countPrimes(self, n):
        """
        :type n: int
        :rtype: int
        """
        primes = [True for i in range(n)]
        prime = 2
        while prime ** 2 <= n:
            if(primes[prime] == True):
                for i in range(prime*2, n, prime):
                    primes[i] = False
            prime += 1
        return sum(primes[2:])
