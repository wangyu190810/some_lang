
using System;
using System.Collections.Generic;
using System.IO;
using System.Configuration;

namespace OCRparse
{
    class OcrDemo
    {
        private static String api_key;
        private static String secrest_key;

        public void OCRparse()
        {
            api_key = ConfigurationManager.AppSettings["Api_key"];
            secrest_key = ConfigurationManager.AppSettings["Secret_Key"];

        }

        public static object GeneralBasic(String img_path)
        {
            api_key = ConfigurationManager.AppSettings["Api_key"];
            secrest_key = ConfigurationManager.AppSettings["Secret_Key"];
            var client = new Baidu.Aip.Ocr.Ocr( api_key, secrest_key);
            var image = File.ReadAllBytes(img_path);

            // 通用文字识别
            var result = client.GeneralWithLocatin(image,null);
            return result;
        }
        public static void GeneralEnhanced()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");

            // 带生僻字版
            var result = client.GeneralEnhanced(image, null);
        }

        public static void GeneralWithLocatin()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");

            // 带位置版本
            var result = client.GeneralWithLocatin(image, null);
        }

        public static void WebImage()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");

            // 网图识别
            var result = client.WebImage(image, null);
        }





        public static void BankCard()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");

            // 银行卡识别
            var result = client.BankCard(image);
        }

        public static void Idcard()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");

            // 身份证正面识别
            var result = client.IdCardFront(image);
            // 身份证背面识别
            result = client.IdCardBack(image);
        }

        public static void DrivingLicense()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");
            var result = client.DrivingLicense(image);
        }

        public static void VehicleLicense()
        {
            var client = new Baidu.Aip.Ocr.Ocr("Api Key", "Secret Key");
            var image = File.ReadAllBytes("图片文件路径");
            var result = client.VehicleLicense(image);
        }

       





}

}