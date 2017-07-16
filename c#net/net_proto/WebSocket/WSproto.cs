using System;
namespace ws_proto{
    class WSProto{
        public void run(){
            String a = "aa";
            String b = "bb";
            String end = "____";
            end = String.Join(end,a);
            end = String.Join(a,b);
            Console.WriteLine("new namespace {0}",end);   
        }
    }

    
}