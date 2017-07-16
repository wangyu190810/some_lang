using System;
using ws_proto;
using socket;

namespace net_proto
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            WSProto wsproto = new WSProto();
            wsproto.run();
            //TcpSocket tcpsocket = new TcpSocket();
            //tcpsocket.run();
            TcpSocket.run();
        }
    }
}
