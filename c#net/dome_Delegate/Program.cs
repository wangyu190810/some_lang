using System;
using System.Threading;

namespace dome_Delegate
{
    class Program
    {
        static void OtherClassMethod(){
            Console.WriteLine("Delegate an other class's method");
        }

        // static void Main(string[] args)
        // {
        //     var test = new TestDelegate();
        //     test.delegateMethod = new TestDelegate.DelegateMethod(test.NonStaticMethod);
        //     test.delegateMethod += new TestDelegate.DelegateMethod(TestDelegate.StaticMethod);
        //     test.delegateMethod += Program.OtherClassMethod;
        //     test.RunDelegateMethods();
        // }

        static void Main(string[] args)
        {
            var car = new Car(15);
            new Alerter(car);
            car.Run(120);
        }
    }

    class TestDelegate
    {
        public delegate void DelegateMethod();  //声明了一个Delegate Type

        public DelegateMethod delegateMethod;   //声明了一个Delegate对象

        public static void StaticMethod()   
        {
            Console.WriteLine("Delegate a static method");
        }

        public void NonStaticMethod()   
        {
            Console.WriteLine("Delegate a non-static method");
        }

        public void RunDelegateMethods()
        {
            if(delegateMethod != null){
                Console.WriteLine("---------");
                delegateMethod.Invoke();    
                   Console.WriteLine("---------");
            }
        }
    }

      class Car
    {
        public delegate void Notify(int value);
        public event Notify notifier;

        private int petrol = 0;
        public int Petrol
        {
            get { return petrol; }
            set
            {
                petrol = value;
                if (petrol < 10)  //当petrol的值小于10时，出发警报
                {
                    if (notifier != null)
                    {
                        notifier.Invoke(Petrol);
                    }
                }
            }
        }

        public Car(int petrol)
        {
            Petrol = petrol;
        }

        public void Run(int speed)
        {
            int distance = 0;
            while (Petrol > 0)
            {
                Thread.Sleep(500);  
                Petrol--;
                distance += speed;
                Console.WriteLine("Car is running... Distance is " + distance.ToString());
            }
        }
    }

    class Alerter
    {
        public Alerter(Car car)
        {
            car.notifier += new Car.Notify(NotEnoughPetrol);
            Console.WriteLine("new Alerter");
        }

        public void NotEnoughPetrol(int value)
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("You only have " + value.ToString() + " gallon petrol left!");
            Console.ResetColor();
        }
    }
}
