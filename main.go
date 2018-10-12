package main

/*
#cgo LDFLAGS: -L./util/target/debug/ -lgorust_util
#include "./gorust_util.h"
*/
import "C"

func main() {
	C.print_hello_string(C.CString("Gopher"))
}
