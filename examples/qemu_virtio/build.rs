fn main() {
    // 编译汇编文件
    let asm_file = "src/entry.S";
    
    println!("cargo:rerun-if-changed={}", asm_file);
    println!("cargo:rerun-if-changed=src/linker.ld");
    
    // 使用 cc 编译汇编
    cc::Build::new()
        .file(asm_file)
        .flag("-march=rv64gc")
        .flag("-mabi=lp64d")
        .compile("entry");
}
