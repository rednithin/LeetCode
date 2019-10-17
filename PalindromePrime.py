class Solution:
    def isPrime(self, primes, num):
        sqrt_num = num ** .5
        for prime in primes:
            if prime > sqrt_num + 1:
                return True
            if num % prime == 0:
                return False
        return True

    def isPalin(self, num):
        return str(num) == str(num)[::-1]

    def split_num(self, num):
        str_num = str(num)
        str_len = len(str_num)
        str_len_by_2 = str_len // 2

        if len(str(num)) % 2 == 0:
            return str_num[:str_len_by_2], '', str_num[str_len_by_2:]
        else:
            return str_num[:str_len_by_2], str_num[str_len_by_2], str_num[str_len_by_2 + 1:]

    def primePalindrome(self, N: int) -> int:
        primes = [2]

        LIMIT = 2 * 10 ** 8
        SQRT_LIMIT = int(LIMIT ** (1 / 2))

        for num in range(3, SQRT_LIMIT + 2, 2):
            if self.isPrime(primes, num):
                primes.append(num)

        if N <= 2:
            return 2
        if N <= 3:
            return 3
        elif N <= 5:
            return 5
        elif N <= 7:
            return 7

        num = max(num, 11)
        num = N + 1 if N % 2 == 0 else N

        while True:
            if self.isPrime(primes, num) and self.isPalin(num):
                return num

            beg, mid, end = self.split_num(num)
            # print(num)
            # print(beg, mid, end)
            # print(10 ** len(str(num)) - 1 == num)
            # print()
            if 10 ** len(str(num)) - 1 == num:
                num = 10 ** len(str(num)) + 1
            elif mid == '':  # Even number
                if beg[::-1] > end:
                    end = beg[::-1]
                else:
                    beg = str(int(beg) + 1)
                    end = beg[::-1]
                num = int(beg + mid + end)
            else:
                if beg[::-1] > end:
                    end = beg[::-1]
                elif mid != '9':
                    mid = str(int(mid) + 1)
                    end = beg[::-1]
                else:
                    mid = '0'
                    beg = str(int(beg) + 1)
                    end = beg[::-1]
                num = int(beg + mid + end)


# obj = Solution()
# print(obj.primePalindrome(1))
# print(obj.primePalindrome(2))
# print(obj.primePalindrome(6))
# print(obj.primePalindrome(7))
# print(obj.split_num(13))
# print(obj.split_num(131))
# print(obj.split_num(13))

# print(obj.primePalindrome(13))
# print(obj.primePalindrome(9989900))
