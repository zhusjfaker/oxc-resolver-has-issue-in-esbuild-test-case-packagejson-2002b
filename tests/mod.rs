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
    fn test_resolver_package_json_bad_main() {
        let options = ResolveOptions {
            alias_fields: vec![vec!["browser".into()]],
            exports_fields: vec![vec!["exports".into()]],
            condition_names: vec!["node".into(), "import".into(), "require".into()],
            main_fields: vec!["main".into()],
            ..ResolveOptions::default()
        };
        let dir = path_resolve("test_package_json_bad_main/src");
        let resolver = Resolver::new(options);
        let res = resolver.resolve(dir, "demo-pkg").unwrap();
        println!("res-> \n {:#?}", res);
        /*
        node_modules/demo-pkg/package.json
        {
          "main": "./does-not-exist.js"
        }
        the main field in package.json is invalid
        but oxc_resolver will try to resolve index.js
         */
    }
}
