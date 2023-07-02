use fancy_regex::Regex;
use lazy_static::lazy_static;

pub fn str_cmd_to_array(cmd_str: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
                              (?<=\s|^)(?<!\\)'
                              (?<quoted1>(?:[^']|\\')*)
                              (?<!\\)'(?=\s|$)
                            |
                              (?<=\s|^)(?<!\\)"
                              (?<quoted2>(?:[^"]|\\")*)
                              (?<!\\)"(?=\s|$)
                            |
                              (?<unquoted>
                                (?:
                                    (?<!\\)'
                                    (?:[^']|\\')*
                                    (?<!\\)'
                                |
                                    (?<!\\)"
                                    (?:[^"]|\\")*
                                    (?<!\\)"
                                |
                                    [^"'\s]+
                                )
                              +)
                          "#,
        )
        .unwrap();
    }

    RE.captures_iter(cmd_str)
        .map(|caps| caps.unwrap())
        .map(|caps| {
            if let Some(p) = caps.name("quoted1") {
                return p.as_str().replace(r#"\'"#, "'");
            };
            if let Some(p) = caps.name("quoted2") {
                return p.as_str().replace(r#"\""#, r#"""#);
            };
            if let Some(p) = caps.name("unquoted") {
                return p.as_str().to_string();
            };
            "".to_string()
        })
        .collect()
}

pub fn str_array_cmd_to_str_cmd(cmd_array: &Vec<String>) -> String {
    cmd_array
        .iter()
        .map(|s| quote_cmd_part(s))
        .collect::<Vec<String>>()
        .join(" ")
}

// return the one command string version of an element of the command array version
pub fn quote_cmd_part(cmd_part: &str) -> String {
    lazy_static! {
        static ref RE_DOUBLE_QUOTED: Regex = Regex::new(r#"^"(?:[^"]|\\")*"$"#).unwrap();
        static ref RE: Regex = Regex::new(
            r#"(?x)
                                            ^(?:
                                                  [^\s]+
                                                |
                                                    (?<!\\)'
                                                    ((?:[^']|\\')*)
                                                    (?<!\\)'
                                                |
                                                    (?<!\\)"
                                                    ((?:[^"]|\\")*)
                                                    (?<!\\)"
                                            )+
                                            $"#,
        )
        .unwrap();
    }
    if RE_DOUBLE_QUOTED.is_match(cmd_part).unwrap() {
        format!(
            "\"{}\"",
            cmd_part.replace('\\', r#"\\"#).replace('"', r#"\""#)
        )
    } else if RE.is_match(cmd_part).unwrap() {
        cmd_part.to_string()
    } else {
        format!("\"{}\"", cmd_part.replace('"', r#"\""#))
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd_str_spliter::{quote_cmd_part, str_array_cmd_to_str_cmd, str_cmd_to_array};

    #[test]
    fn test_quote_cmd_part() {
        assert_eq!(r#"a\"a"#, quote_cmd_part(r#"a\"a"#),);
        assert_eq!("aa", quote_cmd_part(r#"aa"#),);
        assert_eq!(r#""a a""#, quote_cmd_part(r#"a a"#),);
        assert_eq!(r#"a\"a"#, quote_cmd_part(r#"a\"a"#),);
        assert_eq!(r#"--bb="b""#, quote_cmd_part(r#"--bb="b""#),);
        assert_eq!(r#"--cc="c c""#, quote_cmd_part(r#"--cc="c c""#),);
        assert_eq!(r#"--dd="d d""#, quote_cmd_part(r#"--dd="d d""#),);
        assert_eq!(r#""\"a\\\" b\"""#, quote_cmd_part(r#""a\" b""#),);
    }

    #[test]
    fn test_str_array_cmd_to_str_cmd() {
        assert_eq!(
            r#"aa "" bb"#,
            str_array_cmd_to_str_cmd(&vec!["aa".to_string(), "".to_string(), "bb".to_string()]),
        );
        assert_eq!(
            "aa",
            str_array_cmd_to_str_cmd(&Vec::from(vec!["aa".to_string()])),
        );
        assert_eq!(
            "\"a a\"",
            str_array_cmd_to_str_cmd(&Vec::from(vec!["a a".to_string()])),
        );
        assert_eq!(r#"uu "d dd\"d dd" bb"#, "uu \"d dd\\\"d dd\" bb");
        assert_eq!(
            r#"aa "d dd\"d dd" bb"#,
            str_array_cmd_to_str_cmd(
                &vec!["aa", "d dd\"d dd", "bb"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            ),
        );

        assert_eq!(
            r#"aa "\"bbb\"" --cc "d dd\"d dd" "ee ee" "ff f" 'g " ""\'gg' hh'hh h'hh"h h" "" ii"ii"i --jj="j j""#,
            str_array_cmd_to_str_cmd(
                &vec![
                    "aa",
                    "\"bbb\"",
                    "--cc",
                    "d dd\"d dd",
                    "ee ee",
                    "ff f",
                    "'g \" \"\"\\'gg'",
                    "hh'hh h'hh\"h h\"",
                    "",
                    "ii\"ii\"i",
                    "--jj=\"j j\""
                ]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
            ),
        );
    }

    #[test]
    fn test_str_cmd_to_array() {
        assert_eq!(["aa", "bb"], &str_cmd_to_array(r#"aa "bb""#,)[..],);
        assert_eq!(["aa", "bb"], &str_cmd_to_array(r#"aa "bb" "#,)[..],);
        assert_eq!(
            [r#""aa aa"="bb bb""#],
            &str_cmd_to_array(r#"  "aa aa"="bb bb"   "#,)[..],
        );

        assert_eq!(
            [
                "aa",
                "bbb",
                "--cc",
                "d dd\"d dd",
                "ee ee",
                "ff f",
                "g \" \"\"'gg",
                "hh'hh h'hh\"h h\"",
                "",
                "ii\"ii\"i",
                "--jj=\"j j\""
            ],
            &str_cmd_to_array(
                r#" aa   "bbb"   --cc "d dd\"d dd" "ee ee" "ff f" 'g " ""\'gg' hh'hh h'hh"h h"  "" ii"ii"i --jj="j j" "#,
            )[..],
        );

        assert_eq!(
            [
                "arg1",
                "-arg2",
                "--arg3=val3",
                "--arg4=\"val 4\"",
                "--arg5='val 5'",
                "arg6",
                "arg7",
                "arg8\"8 8\"'88 88'",
            ],
            &str_cmd_to_array(
                r#"arg1 -arg2 --arg3=val3 --arg4="val 4" --arg5='val 5' "arg6" 'arg7' arg8"8 8"'88 88'"#,
            )[..],
        );

        assert_eq!(
            vec!["aa", r#""bb cc"="dd ee""#],
            str_cmd_to_array(r#"aa "bb cc"="dd ee""#),
        );
        assert_eq!(
            vec!["deno", "test", "-A", r#"--ignore="vendor,npm""#],
            str_cmd_to_array(r#"deno test -A --ignore="vendor,npm""#),
        );
        assert_eq!(
            vec!["aa", r#"a"bb cc "d"#, "e"],
            str_cmd_to_array(r#"aa a"bb cc "d e"#),
        );
        assert_eq!(
            vec!["aa", "bb", "c  c\"c c", "d d'd d", " e e ", "ff"],
            str_cmd_to_array(r#"  aa  bb  "c  c\"c c"   'd d\'d d' " e e " ff  "#),
        );

        assert_eq!(vec!["echo", "aa", "bb"], str_cmd_to_array("echo aa bb"),);
        assert_eq!(
            vec!["echo", "aa bb", "cc dd"],
            str_cmd_to_array(r#"echo 'aa bb' "cc dd""#),
        );
        assert_eq!(
            str_cmd_to_array(r#"echo 'aa "c c" bb'"#),
            vec!["echo", r#"aa "c c" bb"#],
        );
        assert_eq!(vec!["echo", "'aa'"], str_cmd_to_array(r#"echo "'aa'""#),);
        assert_eq!(vec!["echo", r#""aa""#], str_cmd_to_array(r#"echo '"aa"'"#),);
        assert_eq!(vec!["echo", "''"], str_cmd_to_array(r#"echo "''""#),);
        assert_eq!(vec!["echo", r#""""#], str_cmd_to_array(r#"echo '""'"#),);

        assert_eq!(
            vec!["aa", "bb", "cc", "dd", "ee"],
            str_cmd_to_array(r#"aa bb cc"dd ee"#),
        );
        assert_eq!(
            vec!["aa", "bb", "cc", "dd", "ee"],
            str_cmd_to_array(r#"aa bb cc "dd ee"#),
        );
        assert_eq!(
            vec!["aa", "bb", "cc", "dd ee"],
            str_cmd_to_array(r#"aa bb cc "dd ee""#),
        );
        assert_eq!(
            vec!["aa", "b", "cc", "d", "e", "f"],
            str_cmd_to_array(r#"aa b cc d "e" f"#),
        );
        assert_eq!(
            vec!["00", r#""a\" b""#, "zz"],
            str_cmd_to_array(r#"00 "\"a\\" b\"" zz"#),
        );
        assert_eq!(
            vec!["00", r#"a" b"#, "zz"],
            str_cmd_to_array(r#"00 "a\" b" zz"#),
        );
    }

    #[test]
    fn test_str_cmd_sync() {
        let array = [
            "aa",
            "bbb",
            "\"bbb\"",
            // FIXME
            // r#""bbb"#,
            r#"a" b"#,
            "--cc",
            "d dd\"d dd",
            "ee ee",
            "ff f",
            "g \" \"\"\\'gg",
            "hh'hh h'hh\"h h\"",
            "",
            "ii\"ii\"i",
            "--jj=\"j j\"",
            "arg1",
            "-arg2",
            "--arg3=val3",
            "--arg4=\"val 4\"",
            "--arg5='val 5'",
            "arg6",
            "arg7",
            "arg8\"8 8\"'88 88'",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let str = str_array_cmd_to_str_cmd(&array);
        let array_res = str_cmd_to_array(&str);
        assert_eq!(&array[..], &array_res[..]);
    }
}
