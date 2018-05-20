from ctypes import cdll

src = "../../target/release/libembed.dylib"

lib = cdll.LoadLibrary(src)

lib.process()

print("done!")
