package parse;

/**
 * Created by wangyu on 2017/4/21.
 */
import java.io.File;

public class MoveFile {

    public static void main(String[] args) {
        String newpath = "F://test//move//";
        String path =  "F://test//movea.txt";
       move(path, newpath);
    }

    public  static boolean move(String filename, String  new_path){
        try {
            File orgin  = new File(filename);
            if (orgin.renameTo(new File(new_path + orgin.getName()))){
                return true;
            } else {
                return false;
            }
        }catch ( Exception e){
            e.printStackTrace();
            return false;
        }

    }
}