using System;

namespace proxy
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
        }
    }


   abstract class Subject{
    public abstract void Request();   
   }

   class RealSubject : Subject{
       public override void Request(){
           Console.Write("");
       }

   }

}
