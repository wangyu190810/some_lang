using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Serialization;
using System.Text;
using System.Threading.Tasks;

namespace OCRparse.model
{
    [DataContract]
    class Image
    {


        [DataMember]
        public string log_id { get; set; }


        [DataMember]
        public string words_result_num { get; set; }


        [DataMember]
        public List<WordsResult> words_result { get; set; }




    }


    [DataContract]
    class WordsResult
    {

        [DataMember]
        public Localtion location { get; set; }


        [DataMember]
        public string words { get; set; }


    }

    [DataContract]
    class Localtion
    {
        [DataMember]
        public int width { get; set; }

        [DataMember]
        public int top { get; set; }

        [DataMember]
        public int height { get; set; }

        [DataMember]
        public int left { get; set; }

    }

    class TestJon{

        public static Image run() {
            
            Localtion location = new Localtion() {
                width = 1,
                top = 1,
                height =1,
                left =1
            };

            WordsResult words_result_data = new WordsResult() {
                location = location,
                words = "白艳"

            };
            List<WordsResult> words_result = new List<WordsResult>();
            words_result.Add(words_result_data);

            var data = new Image()
            {
                log_id = "144398530",
                words_result_num = "13",
                words_result = words_result,


            };
            return data;
        }

       

    }

        class JsonString
        {
            public static String test_json = @"{""log_id"":144398530,""words_result_num"": 13,
  ""words_result"": [
    {
      ""location"": {
        ""width"": 32,
        ""top"": 24,
        ""height"": 17,
        ""left"": 98
      },
      ""words"": ""白艳""
    },
    {
      ""location"": {
        ""width"": 129,
        ""top"": 25,
        ""height"": 16,
        ""left"": 190
      },
      ""words"": ""610115199102050522""
    }
  ]
}";


            public static String example_str = @"[{""Name"":""lj"",""Age"":12,""Alive"":true,
""FavoriteFilms"":[""Up"",""Avatar""],""Child"":null
},
    {""Name"":""cy"",""Age"":28,""Alive"":false,""FavoriteFilms"":null,""Child"":{""Name"":""lj"",
""Age"":12,""Alive"":true,""FavoriteFilms"":[""Up"",""Avatar""],""Child"":null}}]";
        }

        [DataContract]
        public class Person
        {
            [DataMember(Order = 0, IsRequired = true)]
            public string Name { get; set; }

            [DataMember(Order = 1)]
            public int Age { get; set; }

            [DataMember(Order = 2)]
            public bool Alive { get; set; }

            [DataMember(Order = 3)]
            public string[] FavoriteFilms { get; set; }

            [DataMember(Order = 4)]
            public Person Child { get; set; }
        }
    

}
