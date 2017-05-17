using System;

namespace factory
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            OperateTest Operatetest = new OperateTest();
            Operatetest.run();
            
        }
    }
}
