using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Web.Script.Serialization;


namespace OCRparse
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="sender"></param>
        /// <param name="e"></param>
        private void button1_Click(object sender, EventArgs e)
        {
           
            OpenFileDialog dialog = new OpenFileDialog();
            String text = "";
            
            if (dialog.ShowDialog() == DialogResult.OK)
            {
                String path  = dialog.FileName;
                object result =  OcrDemo.GeneralBasic(path);
               
                model.Image image = JSON.parse<model.Image>(result.ToString());
                
                List<model.WordsResult> wordsResults = image.words_result;

                List<decimal> tops =  new List<decimal>();
                decimal start_avg;
                decimal end_avg;
                decimal change = 0.1m;
               // tops.Add(0.0m);
                foreach (model.WordsResult wordsResult in wordsResults)
                {  
                   

                    if (tops.LongCount() == 0)
                    {
                        tops.Add(wordsResult.location.top);
                        text += wordsResult.words;
                        continue;
                    }
                    else {
                        start_avg = tops.Average();
                        tops.Add(wordsResult.location.top);
                    }
                    

                    end_avg = tops.Average();
                    if (start_avg / end_avg > change)
                    {
                        text += "\n";
                        text += wordsResult.words;
                        tops.Clear();
                        tops.Add(0.0m);
                    }
                    else {

                        text += wordsResult.words;
                        text += "\t";
                    }  
                    

                }

                //  Console.WriteLine(string.Format("{0}:{1}", j.Key, j.Value));
                //  text += j.Value.ToString();


            }
                this.richTextBox1.Text = text;
                
            
   
           
        }

        private void label1_Click(object sender, EventArgs e)
        {

        }

        private void textBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void richTextBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void button2_Click(object sender, EventArgs e)
        {
              String str = model.JsonString.test_json;
            String end_str = "";
            String tmp = "";
            model.Image image = JSON.parse<model.Image>(str);
            end_str += image.log_id.ToString();
            List<model.WordsResult> wordsResults = image.words_result;
            foreach (model.WordsResult wordsResult in wordsResults) {
              //  wordsResult.location.top
               // end_str += wordsResult.words;


            }
         




            // String example_str = model.JsonString.example_str;
            //List < model.Person >persons =  JSON.parse<List<model.Person>>(example_str);
            //foreach (model.Person person in persons) {
            //   tmp = person.Name;
            //end_str += tmp;
            // }

            var data = model.TestJon.run();
            //model.Image image = JSON.parse<model.Image>(str);

            end_str += JSON.stringify(data);
            //end_str = data.words_result.ToString();

            this.richTextBox1.Text = end_str;

        }

        private void label1_Click_1(object sender, EventArgs e)
        {

        }

        private void button2_Click_1(object sender, EventArgs e)
        {

        }
    }
}
