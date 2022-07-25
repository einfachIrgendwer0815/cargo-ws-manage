use cargo_ws_manage::fs as cws_fs;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;
use test_context::{test_context, TestContext};

struct TestFiles {
    name: String,
}

impl TestContext for TestFiles {
    fn setup() -> TestFiles {
        let dir_name = format!("test_files_{}", thread_rng().gen_range(0..10000));
        match fs::create_dir(dir_name.as_str()) {
            Ok(_) => TestFiles {
                name: dir_name.clone(),
            },
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => TestFiles {
                    name: dir_name.clone(),
                },
                _ => panic!("{}", e),
            },
        }
    }

    fn teardown(self) {
        if let Err(_) = fs::remove_dir_all(self.name) {};
    }
}

mod test_dir {
    use super::*;

    #[test_context(TestFiles)]
    #[test]
    fn test_mkdir(ctx: &mut TestFiles) {
        let raw_path = format!("{}/test_mkdir_not_recursive", &ctx.name);
        let path = Path::new(raw_path.as_str());

        let result = cws_fs::mkdir(&path, false);
        assert!(result.is_ok());
    }

    #[test_context(TestFiles)]
    #[test]
    fn test_mkdir_recursive(ctx: &mut TestFiles) {
        let raw_path = format!("{}/test_mkdir_recursive/inner_dir", &ctx.name);
        let path = Path::new(raw_path.as_str());

        let result = cws_fs::mkdir(&path, true);
        assert!(result.is_ok());
    }
}

mod test_toml {
    struct ReadTomlContext {
        test_files: TestFiles,
        pub filename: String,
    }

    impl TestContext for ReadTomlContext {
        fn setup() -> ReadTomlContext {
            let test_files = TestFiles::setup();
            let filename = format!("{}/toml_read.toml", test_files.name);

            let mut file = match fs::File::create(&filename) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };

            let data = "\
hello = \"world\"
name = \"Santa Claus\"

[section_a]
abc = \"CBA\"
xyz = 54626";

            match file.write(data.as_bytes()) {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            };

            ReadTomlContext {
                test_files,
                filename,
            }
        }

        fn teardown(self) {
            self.test_files.teardown();
        }
    }

    struct WriteTomlContext {
        test_files: TestFiles,
        filename: String,
    }

    impl TestContext for WriteTomlContext {
        fn setup() -> WriteTomlContext {
            let test_files = TestFiles::setup();
            let filename = format!("{}/toml_write.toml", test_files.name);

            WriteTomlContext {
                test_files,
                filename,
            }
        }

        fn teardown(self) {
            self.test_files.teardown();
        }
    }

    use super::*;

    #[test_context(ReadTomlContext)]
    #[test]
    fn read_toml_file(ctx: &mut ReadTomlContext) {
        #[derive(Deserialize)]
        struct Data {
            hello: String,

            section_a: SectionA,
        }

        #[derive(Deserialize)]
        struct SectionA {
            abc: String,
            xyz: u32,
        }

        let data = cws_fs::read_toml_file::<Data>(Path::new(&ctx.filename)).unwrap();

        assert_eq!(data.hello, "world");
        assert_eq!(data.section_a.abc, "CBA");
        assert_eq!(data.section_a.xyz, 54626);
    }

    #[test_context(ReadTomlContext)]
    #[test]
    fn read_toml_file_with_optionals(ctx: &mut ReadTomlContext) {
        #[derive(Deserialize)]
        struct Data {
            hello: String,
            name: Option<String>,

            section_a: SectionA,
        }

        #[derive(Deserialize)]
        struct SectionA {
            abc: String,
            ghi: Option<bool>,
            xyz: u32,
        }

        let data = cws_fs::read_toml_file::<Data>(Path::new(&ctx.filename)).unwrap();

        assert_eq!(data.hello, "world");
        assert_eq!(data.name, Some(String::from("Santa Claus")));
        assert_eq!(data.section_a.abc, "CBA");
        assert_eq!(data.section_a.xyz, 54626);
        assert_eq!(data.section_a.ghi, None);
    }

    #[test_context(WriteTomlContext)]
    #[test]
    fn write_toml_file(ctx: &mut WriteTomlContext) {
        #[derive(Serialize)]
        struct Data {
            hello: String,

            section_a: SectionA,
        }

        #[derive(Serialize)]
        struct SectionA {
            abc: String,
            xyz: u32,
        }

        let data = Data {
            hello: String::from("world"),
            section_a: SectionA {
                abc: String::from("CBA"),
                xyz: 54626,
            },
        };

        let result = cws_fs::write_toml_file(Path::new(&ctx.filename), &data, false);

        assert!(result.is_ok());

        let mut file = fs::File::open(&ctx.filename).unwrap();

        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(
            buffer,
            "hello = \"world\"\n\n[section_a]\nabc = \"CBA\"\nxyz = 54626\n"
        );
    }

    #[test_context(WriteTomlContext)]
    #[test]
    fn write_toml_file_with_optionals(ctx: &mut WriteTomlContext) {
        #[derive(Serialize)]
        struct Data {
            hello: String,
            name: Option<String>,

            section_a: SectionA,
        }

        #[derive(Serialize)]
        struct SectionA {
            abc: String,
            ghi: Option<bool>,
            xyz: u32,
        }

        let data = Data {
            hello: String::from("world"),
            name: None,
            section_a: SectionA {
                abc: String::from("CBA"),
                ghi: Some(true),
                xyz: 54626,
            },
        };

        let result = cws_fs::write_toml_file(Path::new(&ctx.filename), &data, false);

        assert!(result.is_ok());

        let mut file = fs::File::open(&ctx.filename).unwrap();

        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(
            buffer,
            "hello = \"world\"\n\n[section_a]\nabc = \"CBA\"\nghi = true\nxyz = 54626\n"
        );
    }
}
