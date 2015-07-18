extern crate gcc;
//
fn main() {
    gcc::compile_library("libcuba.a", &["cubature/pcubature.c", "cubature/hcubature.c"]);//,  "src/hcubature.c",	"src/pcubature.c"]);
}
