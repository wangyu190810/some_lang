using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.IO;
using System.Xml.Linq;
using static System.Console;

namespace OCRparse.model
{
    class JsonProcess
    {

        public static void CreateJson() {
            var book1 = new JObject();
            book1["title"] = "解读中国经济";
            book1["publisher"] = "北京大学出版社";
            var book2 = new JObject();
            book2["title"] = "java 并发编程";
            book2["publisher"] = "机械工业出版社";
            var books = new JArray();
            books.Add(book1);
            books.Add(book1);

            var json = new JObject();
            json["books"] = books;
            Write(json);

        }

    }


}
