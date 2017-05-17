using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace strategy
{
    public partial class Form1 : Form
    {
        double total = 0;
        
        public Form1()
        {
            InitializeComponent();
        }

        private void textBox1_TextChanged(object sender, EventArgs e)
        {
            
        }

        private void richTextBox1_TextChanged(object sender, EventArgs e)
        {

        }

        private void textBox2_TextChanged(object sender, EventArgs e)
        {

        }

        private void button1_Click(object sender, EventArgs e)
        {
            CashContext csuper =  new CashContext(comboBox1.SelectedItem.ToString());

            double totalPrice = csuper.getResutl( Convert.ToDouble(textBox1.Text) * Convert.ToDouble(textBox2.Text));
            total += totalPrice;
            listBox1.Items.Add("单价：" + textBox1.Text + "数量" + textBox2.Text + comboBox1.SelectedItem + "合计： " + total);
            label2.Text = total.ToString();

        }

        private void listBox1_SelectedIndexChanged(object sender, EventArgs e)
        {
           
        }

        private void label1_Click(object sender, EventArgs e)
        {

        }

        private void comboBox1_SelectedIndexChanged(object sender, EventArgs e)
        {

        }

        private void Form1_Load(object sender, EventArgs e)
        {
            comboBox1.Items.AddRange(new object[] { "正常收费", "打8折", "满300返还100" });
            comboBox1.SelectedIndex = 0;
            listBox1.Text = "";
        }

        private void label4_Click(object sender, EventArgs e)
        {

        }
    }
}
