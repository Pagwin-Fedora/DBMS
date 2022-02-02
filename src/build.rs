fn main(){
    const_declaration!(CONFIG_DIR:PathBuf = config_dir().unwrap().join("MessageShim.yaml"))
}
