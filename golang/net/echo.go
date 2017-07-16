package main

import ("fmt"
		"log"
		"net"
		"os"
		
)


func client() {
	log.Println("Hello, world")

	netListen, err := net.Listen("tcp", "localhost:8000")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}

	defer netListen.Close()

	log.Println("Waiting for clients")

	for {
		conn, err := netListen.Accept()
		if err != nil {
			continue
		}

		log.Println(conn.RemoteAddr().String(), " tcp connect success")
		go handleConnection(conn)
	}
}

func handleConnection(conn net.Conn) {
	var buf [1]byte
	n, err := conn.Read(buf[:1])
	if n != 0 || err == nil{
		fmt.Errorf("Read = %v, %v; want 0, non-nil", n, err)
	}else{
		fmt.Printf("%s",buf)
	}
}