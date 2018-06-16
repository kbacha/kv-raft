extern crate protoc;
extern crate protoc_rust;
extern crate protobuf_codegen_pure;

use protobuf_codegen_pure::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/public/proto",
        input: &["proto/public/public.proto", "proto/public/request.proto", "proto/public/response.proto"],
        includes: &["proto/public"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
