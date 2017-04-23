package parse;

import java.io.IOException;

import java.io.InputStream;

import java.util.Properties;

/**
 * Created by wangyu on 2017/4/23.
 */
public class LoadProp {



    public static void main(String[] args) {

        LoadProp loadProp = new LoadProp();

        InputStream in = loadProp.getClass().getClassLoader().getResourceAsStream("resources/db.properties");

        Properties prop = new Properties();

        try {

            prop.load(in);

        } catch (IOException e) {
            e.printStackTrace();

        }

        System.out.println(prop.getProperty("name", "none"));

        System.out.println(prop.getProperty("age", "none"));

        }

}
