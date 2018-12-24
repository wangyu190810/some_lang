#!/usr/bin/env python
# -*-coding:utf-8-*-
#!/usr/bin/python3
# -*- coding: UTF-8 -*-


__author__ = 'xieyanke'
import sys
sys.path.append('./gen-py')
from example import format_data
from example import ttypes
from thrift.transport import TSocket
from thrift.transport import TTransport
from thrift.protocol import TBinaryProtocol
from thrift.server import TServer

__HOST = 'localhost'
__PORT = 8080

class FormatDataHandler(object):
    def do_format(self, data):
        return ttypes.Data(data.text.upper())


if __name__ == '__main__':
    handler = FormatDataHandler()

    processor = format_data.Processor(handler)
    transport = TSocket.TServerSocket(__HOST, __PORT)
    tfactory = TTransport.TBufferedTransportFactory()
    pfactory = TBinaryProtocol.TBinaryProtocolFactory()

    rpcServer = TServer.TSimpleServer(processor,transport, tfactory, pfactory)

    print('Starting the rpc server at', __HOST,':', __PORT)
    rpcServer.serve()

