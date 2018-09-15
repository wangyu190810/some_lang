#!-*-coding:utf-8-*-
"""
创建Block节点
"""


import hashlib
import datetime as date
import sys


class Block(object):
    """
    节点
    """
    def __init__(self, index, timestamp, data, previous_hash):
        self.index = index
        self.timestamp = timestamp
        self.data = data
        self.previous_hash = previous_hash
        self.hash = self.hash_data()

    def get_data(self):
        """
        获取hash需要的字符串
        """
        data = str(self.index) +\
            str(self.timestamp) +\
            str(self.data) +\
            str(self.previous_hash)
        if sys.version_info[0] == 3:
            return data.encode("utf=8")
        return data

    def hash_data(self):
        """数据hash"""
        sha = hashlib.sha256()
        sha.update(self.get_data())
        return sha.hexdigest()

    def __str__(self):
        return "Index %s timestamp %s data %s \n" % (self.index, self.timestamp, self.data)

    __repr__ = __str__


def create_genesis_block():
    """创建第一节点"""
    return Block(1, date.datetime.now(), "first block", "0")


def next_block(last_block, data=None):
    """
    创建新节点
    """
    this_index = last_block.index + 1
    this_timestamp = date.datetime.now()
    if data is None:
        this_data = "default data" + str(this_index)
    else:
        this_data = data
    this_previous_bash = last_block.hash
    return Block(this_index, this_timestamp, this_data, this_previous_bash)


def test_first_block():
    """测试第一个节点"""
    print(create_genesis_block())


def create_one_block(flag=1):
    "创建新节点"
    block_list = []
    first_block = create_genesis_block()
    block_list.append(first_block)
    while True:
        _block = block_list[-1]
        new_block = next_block(_block)
        block_list.append(new_block)
        if flag == len(block_list):
            break
    return block_list


def test_create_one_block():
    """测试创建新节点"""
    print(create_one_block(10))


def main():
    """mian"""
    test_first_block()
    test_create_one_block()

if __name__ == '__main__':
    main()
