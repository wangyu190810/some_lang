/**
 * Created by wangyu on 2017/4/21.
 */

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;

import org.apache.pdfbox.io.RandomAccessBuffer;
import org.apache.pdfbox.pdfparser.PDFParser;
import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.text.PDFTextStripper;

import java.io.IOException;
import java.io.PrintWriter;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;

import parse.*;

public class PdfRead {

    public static void main(String[] args) throws SQLException{
        PropertiesUtils rc =  new PropertiesUtils("sys.properties");
        String uploaddir = rc.getValue("file.uploaddir");
        String movedir = rc.getValue("file.movedir");

        System.out.println(uploaddir);
//        List<String> filse = getFileName("F://test//");
//        List<String> filse = getFileName("F:\\test\\");
        List<String> filse = getFileName(uploaddir);
        String newpath = movedir;
        Connection connection = ConnDB.getConnection();
//        String insert = "INSERT INTO tb_user_position_day_pdf (stock_name,stock_code,account, amount, date, filename) VALUES('?','?','?','?',?,'?')";
        String insert = "INSERT INTO tb_user_position_day_pdf (stock_name,stock_code,account, amount, date, filename) VALUES(?,?,?,?,?,?)";
        PreparedStatement  stmt = connection.prepareStatement(insert);
//        PreparedStatement  stmt = null;
        connection.prepareStatement(insert);
        for (int i = 0; i < filse.size(); i++) {
            Parse(filse.get(i), stmt, newpath);

        }
        for (int i = 0; i < filse.size(); i++) {
            MoveFile.move(filse.get(i),newpath);

        }

    }

    private static void Parse(String filename, PreparedStatement stmt, String newpath){

        File pdfFile = null;
        pdfFile = new File(filename);
        PDDocument document = null;
        try {
            // 方式一：
            /**
             InputStream input = null;
             input = new FileInputStream( pdfFile );
             //加载 pdf 文档
             PDFParser parser = new PDFParser(new RandomAccessBuffer(input));
             parser.parse();
             document = parser.getPDDocument();
             **/

            // 方式二：
            document = PDDocument.load(pdfFile);

            // 获取页码
            int pages = document.getNumberOfPages();

            // 读文本内容
            PDFTextStripper stripper = new PDFTextStripper();
            // 设置按顺序输出
            stripper.setSortByPosition(true);
            stripper.setStartPage(1);
            stripper.setEndPage(pages);
            String content = stripper.getText(document);
            String[] line_data = content.split("\n");
            boolean flag  = false;
            String[] filename_spilt = filename.split("_");
//            int date = Integer.parseInt(filename_spilt[1].split(".")[0]);
            int date = 20170405;
            String account  = filename_spilt[0];
            account = "800148";
            String  stock_code;
            String  stock_name;
            int amount;
            String amount_str;
            for (String line: line_data){
                if (line.indexOf("HK - HK Stock (HKD)") > -1){
                    flag = true;
//                    System.out.println(line);
                }
                if (flag){

                    if (StockData(line) && line.length()>4){
                        System.out.println(line);
                        String[] stock_data = line.split(" ");
                        int split_len = stock_data.length;
                        if (split_len == 9){
                            stock_code = stock_data[0];
                            stock_name = stock_data[1];
                            amount_str = stock_data[split_len - 5];
                        }else if (split_len == 10){
                            stock_code = stock_data[0];
                            stock_name = stock_data[1] + stock_data[2] + stock_data[3];
                            amount_str =  stock_data[split_len - 5];
                        }else{
                            stock_code = stock_data[0];
                            stock_name = stock_data[1];
                            amount_str = stock_data[split_len -5];
                        }
                        amount = LineParse.atoi(amount_str);
                        System.out.print(stock_code);
                        System.out.print(stock_name);
                        System.out.print(amount);
                        System.out.print(account);
                        System.out.print(filename);
                        System.out.print(date);
                        System.out.println();
//                        stmt.setString(1,stock_name );
//                        stmt.setString(2,stock_code );
//                        stmt.setString(3,account );
//                        stmt.setInt(4,amount );
//                        stmt.setInt(5,date );
//                        stmt.setString(6, filename);
//                        stmt.executeUpdate();
                    }
                }
            }


        } catch (Exception e) {
            System.out.println(e);
        }
        try {
            document.close();
        }catch (Exception e){
            e.printStackTrace();
        }
//        pdfFile.delete();
    }


    private static boolean StockData(String content){
        String[] stock_data  = content.split(" ");
        String str = stock_data[0];
        try {
            int stock_code = Integer.parseInt(str);
            return true;
        } catch (NumberFormatException e) {
//            e.printStackTrace();
            return false;
       }
    }
    public static  List<String> getFileName(String path) {

        List<String> filse = new ArrayList<String>();
        File f = new File(path);
        if (!f.exists()) {
            System.out.println(path + " not exists");
            return filse;
        }

        File fa[] = f.listFiles();
        for (int i = 0; i < fa.length; i++) {
            File fs = fa[i];
            if (fs.isDirectory()) {
                System.out.println(fs.getName() + " [目录]");
            } else {
                filse.add(path + fs.getName());
            }
        }
        return filse;
    }


}
