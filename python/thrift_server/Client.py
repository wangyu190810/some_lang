#! /usr/bin/env python
# -*- coding: utf-8 -*-
import sys
sys.path.append('./gen-py')
from thrift.transport import TSocket
from thrift.transport import TTransport
from thrift.protocol import TBinaryProtocol
from example.format_data import Client
from example.format_data import Data

__HOST = '0.0.0.0'
__PORT = 8080

tsocket = TSocket.TSocket(__HOST, __PORT)
transport = TTransport.TBufferedTransport(tsocket)
protocol = TBinaryProtocol.TBinaryProtocol(transport)
client = Client(protocol)

data = Data('hello,world!')
transport.open()

print(client.do_format(data).text)

