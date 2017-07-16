package main

import ("fmt"
		"log"
		"net"
		"time"
)

func establishConn(i int) net.Conn {
    conn, err := net.Dial("tcp", "0.0.0.0:8080")
    if err != nil {
        log.Printf("%d: dial error: %s", i, err)
        return nil
    }
    log.Println(i, ":connect to server ok")
	fmt.Printf("end")
    return conn
}

func main() {
    var sl []net.Conn
    for i := 1; i < 10; i++ {
        conn := establishConn(i)
        if conn != nil {
            sl = append(sl, conn)
        }
    }

    time.Sleep(time.Second * 10000)
}