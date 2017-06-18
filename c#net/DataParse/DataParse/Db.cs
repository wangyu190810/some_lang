using MySql.Data.MySqlClient;
using System;
//using MySQLDriverCS;

namespace DataParse
{
    class Db
    {
        MySqlConnection conn;

        // C初始化函数 
        public Db() {
            conn = content();
        }

        // 连接函数
        public MySqlConnection content(){
            string connStr = "Server=localhost;Database=test;Uid=root;Pwd=2016;CharSet=utf8;";
            MySqlConnection conn = new MySqlConnection(connStr);
            
            return conn;
        }
        // sql 执行函数
        public MySqlDataReader queryOne(String query_sql) {

            
            conn.Open();
            MySqlCommand cmd = new MySqlCommand(query_sql, conn);
            MySqlDataReader data = cmd.ExecuteReader();
            return data;
        }
        public String Print_data (String query_sql)
        {
            String return_data = "";
            MySqlDataReader data = queryOne(query_sql);
            while (data.Read())
            {
                //return_data =  String.Join(return_data,)
            }
            return return_data;
        }
        // 测试函数
        public static void run()
        {
            Db db = new Db();
            
            String sql = "SELECT * FROM test.tb_stock_sector;";
            MySqlDataReader data = db.queryOne(sql);
            while (data.Read())
            {
                int id = (int)data.GetValue(0);
                String name = (string)data.GetValue(3);
                Console.Write(id);
                Console.WriteLine(name);
            }

        }

    }

}