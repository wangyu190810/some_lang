using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace WindowsFormsApplication
{
    public partial class Form1 : Form
    {
        ListBox listBox1 = new ListBox();

        String fileText;
        OpenFileDialog saveFileDialog1 = new OpenFileDialog();
        
        public Form1()
        {
            InitializeComponent();
        }

        private void add_button_Click(object sender, EventArgs e)
        {
            String name = this.textBox1.Text;
            String password = this.textBox2.Text;
            if (name.Equals("22too") && password.Equals("123456"))
            {
                MessageBox.Show("登录成功");
            }else
            {
                MessageBox.Show("登录失败，用户名或者密码错误");
            }
            

        }

   

        private void textBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        private void textBox2_TextChanged(object sender, EventArgs e)
        {

        }

        private void openFileDialog1_FileOk(object sender, CancelEventArgs e)
        {

        }

        private void folderBrowserDialog1_HelpRequest(object sender, EventArgs e)
        {

        }

        private void button2_Click(object sender, EventArgs e)
        {
            //OpenFileDialog fileDialog = new OpenFileDialog();
            //fileDialog.Multiselect = true;
            //fileDialog.Title = "请选择文件";
            //fileDialog.Filter = "所有文件(*.*)|*.*";
            //if (fileDialog.ShowDialog() == DialogResult.OK)
            //{
            //    string file = fileDialog.FileName;
            //    MessageBox.Show("已选择文件:" + file, "选择文件提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
            //}
            OpenFileDialog ofd = new OpenFileDialog();
            ofd.Title = " 请选择您要导入的模板文件：";
            ofd.Filter = "TextDocument(*.cmd)|*.cmd|TextDocument(*.txt)|*.txt";
            ofd.ShowDialog();
            System.IO.StreamReader sr = new System.IO.StreamReader( ofd.FileName ,System.Text.Encoding.Default);
            fileText = sr.ReadToEnd();
            this.richTextBox1_TextChanged(sender, e);

        }

        private void richTextBox1_TextChanged(object sender, EventArgs e)
        {
            richTextBox1.Text = fileText;
        }

        private void cancel_Click(object sender, EventArgs e)
            // cancel centent
        {
            this.richTextBox2.Text = "";
        }

        private void save_Click(object sender, EventArgs e)
        {
            //richTextBox2_TextChanged(sender, e);
            if (this.richTextBox2.Text == "")
            {
                return;
            }
            else
            {

                saveFileDialog1.DefaultExt = "txt";
                saveFileDialog1.Filter = "Text files (*.txt)|*.txt|All files (*.*)|*.*";
                if (this.saveFileDialog1.ShowDialog() == DialogResult.Cancel)
                    return;
                string FileName = this.saveFileDialog1.FileName;

                if (saveFileDialog1.ShowDialog() == DialogResult.OK && FileName.Length > 0)
                {
                    // Save the contents of the RichTextBox into the file.
                    richTextBox1.SaveFile(saveFileDialog1.FileName, RichTextBoxStreamType.PlainText);
                    MessageBox.Show("文件已成功保存");
                }
            }
        }

        private void richTextBox2_TextChanged(object sender, EventArgs e)
        {
            
        }
    }
}
