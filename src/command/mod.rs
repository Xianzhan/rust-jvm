use clap::{App, Arg};

// 解析命令
#[derive(Debug)]
pub struct Arguments {
    pub classpath: Vec<String>,
    pub main_class: String,
}

pub fn parse() -> Arguments {
    let app = app();
    let matches = app.get_matches();

    // 默认添加当前路径
    let mut classpath = vec![".".to_string()];
    let cp_option = matches.value_of("classpath");
    if let Some(cp) = cp_option {
        let mut sp = cp.split(';');
        while let Some(p) = sp.next() {
            classpath.push(p.to_string());
        }
    }

    // main class
    let mc_option = matches.value_of("main-class");
    let main_class = match mc_option {
        None => panic!("main-class 必须设置"),
        Some(mc) => mc.to_string(),
    };

    Arguments {
        classpath,
        main_class,
    }
}

fn app() -> App<'static, 'static> {
    let app = App::new("java")
        .version("1.0")
        .author("xianzhan <424447832@qq.com>")
        .about("Java Virtual Machine")
        .usage("java [--cp classpath] -m MainClass")
        .arg(
            Arg::with_name("classpath")
                .long("classpath")
                .alias("cp")
                .takes_value(true)
                .help("目录和 jar 文件的类搜索路径, 若有多个以 ';' 分隔"),
        )
        .arg(
            Arg::with_name("main-class")
                .long("main-class")
                .alias("mc")
                .short("m")
                .takes_value(true)
                .help("指定第一个加载的 class 文件"),
        );
    app
}

#[cfg(test)]
mod tests {
    use super::app;

    #[test]
    fn test_app() {
        let app = app();
        let matches = app.get_matches_from(vec!["java", "-m", "java.lang.Object"]);
        let main_class = matches.value_of("main-class");
        assert_eq!(main_class, Some("java.lang.Object"));
    }
}
