// FIXME
pub fn cmd_from_str(str: &str) -> Vec<String> {
    use regex::Regex;
    // (?P<not_quoted_and_mixed>([^\s"']+)("((\\"|[^"])*)"|'((\\'|[^'])*)')(([^\s"']+)|'((\\'|[^'])*)'|"((\\"|[^"])*)")*)
    // |
    // (?P<quoted_and_mixed>("((\\"|[^"])*)"|'((\\'|[^'])*)')(([^\s"']+)|'((\\'|[^'])*)'|"((\\"|[^"])*)")+)
    //     |
    let re = Regex::new(
        r#"(?x)
                (?P<double_quoted>"((\\"|[^"])*)")
              |
                (?P<simple_quoted>'((\\'|[^'])*)')
              |
                (?P<not_quoted>([^\s"']+))
            "#,
    )
    .unwrap();

    re.captures_iter(str)
        .map(|caps| {
            // println!("caps={:?}", caps);
            caps.name("double_quoted")
                .map(|m| {
                    let str = m.as_str();
                    // println!("double_quoted={:?}  t={}", m, str);
                    str[1..str.len() - 1].replace("\\\"", "\"")
                })
                .or_else(|| {
                    caps.name("not_quoted_and_mixed").map(|m| {
                        let t: String = m.as_str().parse().unwrap();
                        // println!("not_quoted={:?}  t={}", m, t);
                        t
                    })
                })
                .or_else(|| {
                    caps.name("quoted_and_mixed").map(|m| {
                        let t: String = m.as_str().parse().unwrap();
                        // println!("not_quoted={:?}  t={}", m, t);
                        t
                    })
                })
                .or_else(|| {
                    caps.name("not_quoted").map(|m| {
                        let t: String = m.as_str().parse().unwrap();
                        // println!("not_quoted={:?}  t={}", m, t);
                        t
                    })
                })
                .or_else(|| {
                    caps.name("simple_quoted").map(|m| {
                        // let t: String = m.as_str().parse().unwrap();
                        // println!("simple_quoted={:?}  t={}", m, t);
                        let str = m.as_str();
                        str[1..str.len() - 1].replace("\\'", "'")
                    })
                })
                // .or_else(|| {
                //     caps.name("mixed").map(|m| {
                //         let t: String = m.as_str().parse().unwrap();
                //         println!("mixed={:?}  t={}", m, t);
                //         t
                //     })
                // })
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::cmd_from_string::cmd_from_str;

    #[test]
    fn test_regex() {
        // FIXME
        // assert_eq!(
        //     vec!["aa", r#""bb cc"="dd ee""#],
        //     cmd_from_str(r#"aa "bb cc"="dd ee""#),
        // );
        // assert_eq!(
        //     vec!["deno", "test", "-A", r#"--ignore="vendor,npm""#],
        //     cmd_from_str(r#"deno test -A --ignore="vendor,npm""#),
        // );
        // assert_eq!(
        //     vec!["aa", r#"a"bb cc "d"#, "e"],
        //     cmd_from_str(r#"aa a"bb cc "d e"#),
        // );
        assert_eq!(
            vec!["aa", "bb", "c  c\"c c", "d d'd d", " e e ", "ff"],
            cmd_from_str(r#"  aa  bb  "c  c\"c c"   'd d\'d d' " e e " ff  "#),
        );

        assert_eq!(vec!["echo", "aa", "bb"], cmd_from_str("echo aa bb"),);
        assert_eq!(
            vec!["echo", "aa bb", "cc dd"],
            cmd_from_str(r#"echo 'aa bb' "cc dd""#),
        );
        assert_eq!(
            vec!["echo", r#"aa "c c" bb"#],
            cmd_from_str(r#"echo 'aa "c c" bb'"#),
        );
        assert_eq!(vec!["echo", "'aa'"], cmd_from_str(r#"echo "'aa'""#),);
        assert_eq!(vec!["echo", r#""aa""#], cmd_from_str(r#"echo '"aa"'"#),);
        assert_eq!(vec!["echo", "''"], cmd_from_str(r#"echo "''""#),);
        assert_eq!(vec!["echo", r#""""#], cmd_from_str(r#"echo '""'"#),);

        assert_eq!(
            vec!["aa", "bb", "cc", "dd", "ee"],
            cmd_from_str(r#"aa bb cc"dd ee"#),
        );
        assert_eq!(
            vec!["aa", "bb", "cc", "dd", "ee"],
            cmd_from_str(r#"aa bb cc "dd ee"#),
        );
        assert_eq!(
            vec!["aa", "b", "cc", "d", "e", "f"],
            cmd_from_str(r#"aa b cc d "e" f"#),
        );
    }
}
