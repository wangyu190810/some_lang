package parse;

/**
 * Created by wangyu on 2017/4/21.
 */import java.sql.*;
public class ConnDB {

    public Connection conn = null;
    public Statement stmt = null;
    public ResultSet rs = null;

    public ConnDB() {
    }

    /*~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~*/
    public static Connection getConnection() {

        PropertiesUtils rc =  new PropertiesUtils("db.properties");
        String url = rc.getValue("db.sys.url");
        String user = rc.getValue("db.sys.username");
        String password = rc.getValue("db.sys.password");
        String driver = rc.getValue("db.sys.driver");

//        PropertiesUtils.loadFile(".db.properties");
//        db.sys.driver=com.mysql.jdbc.Driver
//        db.sys.url=jdbc:mysql://127.0.0.1:3306/test?useUnicode=true&characterEncoding=UTF-8&zeroDateTimeBehavior=convertToNull
//        db.sys.username=root
//        db.sys.password=2016
//
//        String url = "jdbc:mysql://127.0.0.1:3306/test?useUnicode=true&characterEncoding=UTF-8&serverTimezone=UTC";
//        String user = "root";
//        String password = "2016";
//        String driver =  "com.mysql.cj.jdbc.Driver";

        Connection conn = null;
        try {
            Class.forName(driver);
//            conn = DriverManager.getConnection(url, user, password);
            conn = (Connection) DriverManager.getConnection(url, user, password);
        } catch (Exception ee) {
            ee.printStackTrace();
        }
        if (conn == null) {
            System.err.println("error~~~~~~~~~~~~~~~");
        }
        return conn;
    }

    public static void main(String[] args) throws SQLException{
        Connection connection = ConnDB.getConnection();
        if (connection != null){
            System.out.println("connected");
        }
    }
}