#[cfg(test)]
mod tests {
    use oxc_resolver::{ResolveOptions, Resolver};
    use std::{env, ffi::OsString, path::Path};

    pub fn path_resolve(path: &str) -> String {
        let work_cwd = {
            match env::var("CARGO_MANIFEST_DIR") {
                Ok(_val) => env!("CARGO_MANIFEST_DIR").to_string(),
                Err(_) => match std::env::current_exe() {
                    Ok(val) => val.parent().unwrap().to_str().unwrap().to_string(),
                    Err(_) => std::env::current_dir()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                },
            }
        };
        let os_work_cwd = OsString::from(work_cwd);
        Path::new(&os_work_cwd)
            .join(path)
            .into_os_string()
            .into_string()
            .unwrap()
    }

    #[test]
    fn test_resolver_inner_package_json() {
        let options = ResolveOptions {
            alias_fields: vec![vec!["browser".into()]],
            ..ResolveOptions::default()
        };
        let fold_path = path_resolve("esbuild-test/src");
        let dir = Path::new(fold_path.as_str());
        let resolver = Resolver::new(options.clone());
        let res = resolver.resolve(dir, "pkg/sub").unwrap();
        println!("res-> \n {:#?}", res);
        // up case is right
        // down case is wrong
        let fold_path = path_resolve("esbuild-test/src/node_modules/pkg/sub");
        let dir = Path::new(fold_path.as_str());
        let resolver = Resolver::new(options);
        let res = resolver.resolve(dir, "sub").unwrap();
        // line 30 has error but expected require('sub') to be resolved to './sub/sub' sub/bar.js
        // because of the browser field in the package.json  "./sub/sub": "./sub/bar.js"
        println!("res-> \n {:#?}", res);
    }

    #[test]
    fn test_resolver_exports_package_json() {
        let options = ResolveOptions {
            alias_fields: vec![vec!["browser".into()]],
            exports_fields: vec![vec!["exports".into()]],
            condition_names: vec!["node".into(), "import".into(), "require".into()],
            ..ResolveOptions::default()
        };
        let dir = Path::new("/Users/jason.zhu/Desktop/github/rolldown/crates/rolldown/tests/esbuild/packagejson/test_package_json_exports_default_over_import_and_require/src");
        let resolver = Resolver::new(options);
        let res = resolver.resolve(dir, "pkg").unwrap();
        println!("res-> \n {:#?}", res);
    }

    #[test]
    fn test_resolver_exports_order_package_json() {
        let options = ResolveOptions {
            alias_fields: vec![vec!["browser".into()]],
            exports_fields: vec![vec!["exports".into()]],
            condition_names: vec!["node".into(), "import".into(), "require".into()],
            ..ResolveOptions::default()
        };
        let dir = Path::new("/Users/jason.zhu/Desktop/github/rolldown/crates/rolldown/tests/esbuild/packagejson/test_package_json_exports_alternatives/src");
        let resolver = Resolver::new(options);
        let res = resolver.resolve(dir, "pkg/apples/green.js").unwrap();
        println!("res-> \n {:#?}", res);
    }

    #[test]
    fn test_resolver_package_json_bad_main() {
        let options = ResolveOptions {
            alias_fields: vec![vec!["browser".into()]],
            exports_fields: vec![vec!["exports".into()]],
            condition_names: vec!["node".into(), "import".into(), "require".into()],
            main_fields: vec!["main".into()],
            ..ResolveOptions::default()
        };
        let dir = Path::new("/Users/jason.zhu/Desktop/github/rolldown/crates/rolldown/tests/esbuild/packagejson/test_package_json_bad_main/src");
        let resolver = Resolver::new(options);
        let res = resolver.resolve(dir, "demo-pkg").unwrap();
        println!("res-> \n {:#?}", res);
    }
}
