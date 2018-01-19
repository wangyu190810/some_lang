using System;
using BaseCase;
namespace study
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            BaseCase.Hello hello = new BaseCase.Hello();
            hello.x  =1;
            hello.y = 2;
            object world = new BaseCase.World();
            Console.WriteLine($"{hello.x + hello.y}");
        }
    }
}
