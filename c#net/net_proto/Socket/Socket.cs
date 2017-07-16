using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
// 网络
using System.Net;
using System.Net.Sockets;
using System.Threading;

namespace socket
{

    class TcpSocket
    {
        private static byte[] result = new byte[1024];
        static Socket serverSocket;

        static int port = 6000;
        static string host = "0.0.0.0";

        public static void run()

        {
            IPAddress ip = IPAddress.Parse(host);
            IPEndPoint ipe = new IPEndPoint(ip, port);

            serverSocket = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);

            serverSocket.Bind(ipe);
            serverSocket.Listen(0);
            Console.WriteLine("服务器启动");
            Thread thread = new Thread(Listenclientconn);
            thread.Start();
            Console.ReadLine();
        }



        // }
        private static void Listenclientconn()
        {
            while (true)
            {
                Socket clientSocket = serverSocket.Accept();
                clientSocket.Send(Encoding.ASCII.GetBytes("Server Say Hello"));
                Thread receiveThread = new Thread(ReceiveMessage);
                receiveThread.Start(clientSocket);
            }
        }

        private static void ReceiveMessage(object clientSocket)
        {
            Socket myClientSocket = (Socket)clientSocket;
            String msg = "";
            while (true)
            {
                try
                {
                    // 接收客户端数据

                    Console.WriteLine(msg);
                    int receiveNumber = myClientSocket.Receive(result);
                    
                    String revc = Encoding.UTF8.GetString(result, 0, receiveNumber);

                    msg = String.Join(msg, revc);
                    Console.WriteLine(msg);
                    Console.WriteLine("receive from {0} data {1}",
                    myClientSocket.RemoteEndPoint.ToString(),
                    msg);
                    if (revc.EndsWith("\n")){
                        msg = "";
                    }
                }
                catch (Exception ex)
                {
                    Console.WriteLine(ex.Message);
                    myClientSocket.Shutdown(SocketShutdown.Both);
                    break;
                }
            }
        }
        public void Thread_run()
        {
            Thread thread = new Thread(this.Thread_test);
            thread.Start();
        }

        public void Thread_test()
        {
            while (true)
            {
                Thread.Sleep(10 * 10);
                Console.WriteLine("thread test");
            }
        }

    }


}