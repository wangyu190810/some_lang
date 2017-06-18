using System;
using System.Collections.Generic;
using System.IO;
using MySql.Data.MySqlClient;

namespace DataParse
{
    class Program
    {
        static void Main(string[] args)
        {

            Dictionary<string, string> dic = parse_file();
            String MySql;
            Db db = new Db();
            
            foreach (KeyValuePair<string, string> kvp in dic)
            { String name = kvp.Value.ToString();
                MySql = String.Format("select * from test.tb_stock_sector where name = {0};", name);
                MySqlDataReader data = db.queryOne(MySql);
                
                while (data.Read())
                {
                    if (data == null)
                    {
                        continue;
                    }
                    int id = (int)data.GetValue(0);
                    String name_info = (string)data.GetValue(3);
                    Console.Write(id);
                    Console.WriteLine(name);
                }
            }

            
         //   Db.run();
        }
        public static string ToSimplifiedChinese(string strTraditional)
        {
            string strSimple = Microsoft.VisualBasic.Strings.StrConv(strTraditional, Microsoft.VisualBasic.VbStrConv.SimplifiedChinese, 0);
            return strSimple;
        }

        

        public static Dictionary<String,String> parse_file()
        {
           
            Dictionary<string, string> dic = new Dictionary<string, string>();
            string path = @"F:\opensource\some_lang\c#net\Design_pattern\dataParse\psd.dat";
            FileStream fs = new FileStream("test.txt", FileMode.OpenOrCreate, FileAccess.ReadWrite); //可以指定盘符，也可以指定任意文件名，还可以为word等文件
            StreamWriter sw = new StreamWriter(fs); // 创建写入流
            try
            {
                // using (StreamWriter sw = new StreamWriter(path)) 
                // {
                //     sw.WriteLine("This");
                //     sw.WriteLine("is some text");
                //     sw.WriteLine("to test");
                //     sw.WriteLine("Reading");
                // }

                // StreamReader sr = File.OpenText(path);
                StreamReader sr = new StreamReader((System.IO.Stream)File.OpenRead(path), System.Text.Encoding.GetEncoding("Big5"));
                string nextLine;
                while ((nextLine = sr.ReadLine()) != null)
                {
                    
                    string[] split_text = nextLine.Split('\t');
                    string company_type = ToSimplifiedChinese(split_text[8]);
                    Console.Write(split_text[4]);
                    Console.WriteLine(company_type);
                    dic.Add(split_text[4], company_type);
                    // 创建文件
                    if (company_type.Length == 0) {
                        continue;
                    }
                    sw.Write(split_text[4]); // 写入Hello World
                    sw.Write(',');
                    sw.WriteLine(company_type);
                }
                
            }
            catch (Exception e)
            {
                Console.WriteLine("The process failed: {0}", e.ToString());
               
            }
            sw.Close(); //关闭文件
            return dic;
        }
        
    }
}
