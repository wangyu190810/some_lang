# -*- coding:utf-8 -*-
class Solution:
    def jumpFloor(self, number):
        n = number / 2
        last = number % 2
        if last == 0:
            offeset =  2 ** n +1
        else:
            offeset =  (2 ** n)+1
        return offeset - 1

def main():
    data = Solution()
    offset = data.jumpFloor(4)
    print(offset)

if __name__ == '__main__':
    main()

        # write code here


