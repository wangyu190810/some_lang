using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.IO;
using System.Configuration;

namespace split_content
{
    public partial class Form1 : Form
    {
        OpenFileDialog openFileDialog1 = new OpenFileDialog();
  
        String filename;
        bool bIsSave;
        // 定义列，当前有多少列
        int col = 0;
        // 连接字符串
        char join_str = '\t';

        String split_flag;

        public bool BIsSave
        {
            get
            {
                return bIsSave;
            }

            set
            {
                bIsSave = value;
            }
        }

        

        public Form1()
        {
            InitializeComponent();
        }

        private void Form1_Load(object sender, EventArgs e)
        {
            
            this.richTextBox2.DragEnter += new DragEventHandler(richTextBox2_DragEnter);
            this.richTextBox2.DragDrop += new DragEventHandler(richTextBox2_DragDrop);
            this.richTextBox2.AllowDrop = true;


        }

        void richTextBox2_DragDrop(object sender, DragEventArgs e)
        {
            object filename = e.Data.GetData("FileDrop");
            if (filename != null)
            {
                var list = filename as string[];
                string filetype = list[0].Substring(list[0].LastIndexOf(@"\") + 1);

                if (filetype.IndexOf(".") != -1)
                {
                    string file_ext = filetype.Split('.')[1];
                    //String allow_file_ext = System.Configuration.ConfigurationSettings.AppSettings['lang'].ToString();
                    String allow_file_ext = System.Configuration.ConfigurationManager.AppSettings["lang"].ToString();
                    if (allow_file_ext.IndexOf(file_ext) != -1) {
                        richTextBox2.Clear();
                        richTextBox2.LoadFile(list[0], RichTextBoxStreamType.PlainText);
                    }                
                }
                else
                {
                    MessageBox.Show("Sorry, I cannot support this file type");
                }




            }
        }

        void richTextBox2_DragEnter(object sender, DragEventArgs e)
        {
            
        }

        private void 打开ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            
        }

        private void 打开ToolStripMenuItem1_Click(object sender, EventArgs e)
        {
            {
                openFileDialog1.FileName = "*.txt";
                openFileDialog1.Filter = "文本文档(*.txt)|*.txt|所有文件|*.*";
                if (openFileDialog1.ShowDialog() == DialogResult.OK)
                {
                    StreamReader sr = new StreamReader(openFileDialog1.FileName, Encoding.Default);
                    try
                    {
                        this.richTextBox2.Text = sr.ReadToEnd();
                    }
                    catch(Exception ex)
                    {
                        Console.WriteLine(ex);
                    }
                   
                    sr.Close();
                }
                filename = openFileDialog1.FileName;
                this.Text = Path.GetFileName(openFileDialog1.FileName) + " - 记事本";
                BIsSave = true;
            }
        }

        private void richTextBox2_TextChanged(object sender, EventArgs e)
        {

        }

        private void 空格分割ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            String data = "";
            foreach(String line in this.richTextBox2.Lines)
            {
                var val = System.Text.RegularExpressions.Regex.Split(line, @"\s{1,}");
                data += string.Join("\t", val);
                data += "\r\n";
            }
            this.richTextBox2.Text = data;
            
        }

        private void 保存ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            SaveFileDialog saveFileDialog1 = new SaveFileDialog();
            saveFileDialog1.InitialDirectory = "d:\\";
            saveFileDialog1.Filter = "ext files (*.txt)|*.txt|*.xls|All files(*.*)";
            saveFileDialog1.FilterIndex = 2;
            saveFileDialog1.RestoreDirectory = true;
            DialogResult dr = saveFileDialog1.ShowDialog();
            if (dr == DialogResult.OK && saveFileDialog1.FileName.Length > 0)
            {
                richTextBox2.SaveFile(saveFileDialog1.FileName, RichTextBoxStreamType.PlainText);
                MessageBox.Show("存储文件成功！", "保存文件");
            }
        }

        private void button1_Click(object sender, EventArgs e)
        {
            split_flag = textBox1.Text;
            String data = "";
            col = 0;
            foreach (String line in this.richTextBox2.Lines)
            {
                var val = line.Split(char.Parse(split_flag));
                data += string.Join(this.join_str.ToString(), val);
                data += "\r\n";
                if (val.Length > col) {
                    col = val.Length;
                }
            }
            this.richTextBox2.Text = data;
            String col_str = "";
            for (int i=0; i < col; i++)
            {   if (i == col-1){
                    col_str += i.ToString();
                }else{
                    col_str += i.ToString() + ",";
                }
            }
            this.textBox2.Text = col_str ;
        }

        private void textBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void button2_Click(object sender, EventArgs e)
        {
            var del_col = textBox2.Text.Split(',');
            
            String data = "";
            
            foreach (String line in this.richTextBox2.Lines)
            {
                String[] val = line.Split(this.join_str);
                var remove_data = new StringBuilder();
                foreach(string _line in del_col)
                { int del_line = -1;
                    try
                    {
                        del_line = int.Parse(_line);
                    }
                    catch (Exception  parse_error)
                    {
                        Console.WriteLine(parse_error);
                    }
                    if (del_line != -1) {
                        if (del_line < val.Length)
                        {
                            for (int i = 0; i < val.Length; i++)
                            {
                                if (val[del_line] != val[i])
                                {
                                    remove_data.Append(val[i]);
                                    // 当最后一行时，不再添加分割符号
                                    if (i != val.Length - 1)
                                    {
                                        remove_data.Append(this.split_flag);
                                    }
                                    
                                }
                            }
                                
                        }
                    }
                   
                }
                //for(int i =0; i < val.Length; i++)
                //{
                //    if (i < del_col.Length){
                //        if (del_col[i] == i.ToString())
                //        {

                //        }else
                //        {
                //            remove_data.Append(del_col[i]);
                //        }
                //    }
                //}
                //data += string.Join(this.split_flag, val);
                var new_line = remove_data.ToString();
             //   var new_line = String.Join(this.split_flag,remove_data);
                if (new_line != "" && new_line != "\r\n")
                {
                    data += new_line;
                    data += "\r\n";
                }
              
            }
            this.richTextBox2.Text = data;
        }

        private void textBox2_TextChanged(object sender, EventArgs e)
        {

        }

        private void menuStrip1_ItemClicked(object sender, ToolStripItemClickedEventArgs e)
        {

        }

        private void 退出ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            Application.Exit();
        }

        private void button3_Click(object sender, EventArgs e)
        {
            if (string.IsNullOrEmpty(dbfPath))
            {
                MessageBox.Show("还没选择文件夹！");
                return;
            }
            if (string.IsNullOrEmpty(comboBox1.Text))
            {
                MessageBox.Show("没有选择数据文件");
                return;
            }
            string connectString = string.Format(
                "Provider=Microsoft.Jet.OLEDB.4.0;Data Source={0};Extended Properties=dBASE IV;User ID=Admin;Password=;"
                , dbfPath);
            using (OleDbConnection connection = new OleDbConnection(connectString))
            {
                DataSet ds = new DataSet();
                try
                {
                    connection.Open();
                    OleDbDataAdapter command = new OleDbDataAdapter("select * from " + comboBox1.Text, connection);
                    command.Fill(ds, "ds");
                    MessageBox.Show(ds.Tables[0].Rows.Count.ToString());
                }
                catch (Exception ex)
                {
                    MessageBox.Show(string.Format("error:{0}", ex.Message));
                }
            }
        }
    }
}
