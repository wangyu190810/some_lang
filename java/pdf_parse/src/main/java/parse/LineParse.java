package parse;

/**
 * Created by wangyu on 2017/4/21.
 */

import java.util.ArrayList;
import java.util.List;

public class LineParse {


    public String[] spilt(String line){
        String[] line_data  = null;
        String[] stock_data = line.split(" ");
        return stock_data;
    }
    public static int atoi(String num){
        String end_num;
        if (num.indexOf(",") > -1){
            String[] nums  = num.split(",");
            end_num = nums[0] + nums[1];
//            System.out.println(end_num);
        }else{
            end_num = num;
        }
        return Integer.parseInt(end_num);
    }


}
