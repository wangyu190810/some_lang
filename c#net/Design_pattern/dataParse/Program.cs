using System;
using System.IO;

namespace dataParse
{
    class Program
    {
    public static void Main() 
    {
        string path = @"F:\opensource\some_lang\c#net\Design_pattern\dataParse\psd.dat";
        try 
        {
                    // using (StreamWriter sw = new StreamWriter(path)) 
            // {
            //     sw.WriteLine("This");
            //     sw.WriteLine("is some text");
            //     sw.WriteLine("to test");
            //     sw.WriteLine("Reading");
            // }

            StreamReader sr = File.OpenText(path); 
            string nextLine; 
            while ((nextLine = sr.ReadLine()) != null) 
            {   string[] split_text = String.split(nextLine,"\t");
                Console.WriteLine(split_text[8]); 
             } 
        } 
        catch (Exception e) 
        {
            Console.WriteLine("The process failed: {0}", e.ToString());
        }
    }
    }
}
