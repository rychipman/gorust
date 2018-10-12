package main

/*
#cgo LDFLAGS: -L./util/target/debug/ -lgorust_util
#include "./gorust_util.h"
*/
import "C"
import "fmt"

func main() {
	printHello("joe")
	printHello("patrick")
	fmt.Printf("got hello string: %q\n", getHello())
}

func printHello(name string) {
	C.print_hello_string(C.CString(name))
}

func getHello() string {
	return C.GoString(C.get_hello_string())
}
