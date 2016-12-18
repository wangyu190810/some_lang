package com;

/**
 * Created by wangyu on 2016/12/18.
 */

import redis.clients.jedis.Jedis;


public class RedisJava {
//
//    public void json() throws MalformedURLException, IOException {
//        String url = "http://freemusicarchive.org/api/get/genres.json?api_key=60BLHNQCAOUFPIBZ&limit=2";
//        String genreJson = IOUtils.toString(new URL(url));
//        JSONObject json = new JSONObject(genreJson);
//        // get the titl   e
//        System.out.println(json.get("title"));
//        // get the data
//        JSONArray genreArray = (JSONArray) json.get("dataset");
//        // get the first genre
//        JSONObject firstGenre = (JSONObject) genreArray.get(0);
//        System.out.println(firstGenre.get("genre_title"));
//    }
//
//    public static void main(String[] args) {
//        //Connecting to Redis server on localhost
//        Jedis jedis = new Jedis("localhost");
//        System.out.println("Connection to server sucessfully");
//        //check whether server is running or not
//        System.out.println("Server is running: "+jedis.ping());
//    }


        public static void main(String[] args) {
            System.out.println("Hello World!");

    }
}