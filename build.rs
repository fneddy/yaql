extern crate cpp_build;
extern crate pkg_config;

trait AddPkg {
    fn add_pkg_config(&mut self, pkg: pkg_config::Library);
}
impl AddPkg for cpp_build::Config {
    fn add_pkg_config(&mut self, pkg: pkg_config::Library) {
        for p in pkg.include_paths.into_iter() {
            self.include(p);
        }
        for p in pkg.link_paths.into_iter() {
            self.flag(&format!("-L{:?}", p));
        }
        for p in pkg.libs.into_iter() {
            self.flag(&format!("-l{}", p));
        }
        for p in pkg.framework_paths.into_iter() {
            self.flag(&format!("-F{:?}", p));
        }
        for p in pkg.frameworks.into_iter() {
            self.flag(&format!("-framework {}", p));
        }
    }
}

fn main() {
    let mut conf = cpp_build::Config::new();

    let core = pkg_config::probe_library("Qt5Core").unwrap();
    let qml = pkg_config::probe_library("Qt5Qml").unwrap();

    conf.add_pkg_config(core);
    conf.add_pkg_config(qml);


    conf.build("src/lib.rs");
}
