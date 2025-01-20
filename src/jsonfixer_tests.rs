
#[cfg(test)]
mod tests {
    use crate::jsonfixer_error::SyntaxError;
    use crate::JsonFixer;
    use crate::JsonFixerError;
    use crate::JsonFixerConfig;

    /*
    ************************** Remove whitespaces *************************
     */


    #[test]
    fn test_empty_object() {
        let input = "{}";
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), "{}");
    }

    #[test]
    fn test_empty_array() {
        let input = "[]";
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), "[]");
    }

    #[test]
    fn test_basic_object() {
        let input = r#"{"name":"John","age":30}"#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), r#"{"name":"John","age":30}"#);
    }

    #[test]
    fn test_basic_array() {
        let input = r#"[1,2,3,4,5]"#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), r#"[1,2,3,4,5]"#);
    }

    #[test]
    fn test_nested_structures() {
        let input = r#"{"users":[{"name":"John","age":30},{"name":"Jane","age":25}]}"#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), input);
    }

    #[test]
    fn test_whitespace() {
        let input = r#"
        {
            "name": "John",
            "age": 30
        }
        "#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), r#"{"name":"John","age":30}"#);
    }

    #[test]
    fn test_unquoted_keys() {
        let input = r#"{name: "John", age: 30}"#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), r#"{"name":"John","age":30}"#);
    }

    #[test]
    fn test_single_quotes() {
        let input = r#"{'name': 'John', 'age': 30}"#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), r#"{"name":"John","age":30}"#);
    }

    #[test]
    fn test_trailing_commas() {
        let cases = vec![
            (r#"{"name": "John", "age": 30,}"#, r#"{"name":"John","age":30}"#),
            (r#"[1, 2, 3,]"#, r#"[1,2,3]"#),
            (r#"{"arr": [1, 2, 3,],}"#, r#"{"arr":[1,2,3]}"#),
        ];

        for (input, expected) in cases {
            let mut fixer = JsonFixer::new(input);
            assert_eq!(fixer.fix().unwrap(), expected);
        }
    }

    #[test]
    fn test_multiple_commas() {
        let cases = vec![
            (r#"[1,,,2,,,3]"#, r#"[1,2,3]"#),
            (r#"{"a":1,,,"b":2,,,"c":3}"#, r#"{"a":1,"b":2,"c":3}"#),
        ];

        for (input, expected) in cases {
            let mut fixer = JsonFixer::new(input);
            assert_eq!(fixer.fix().unwrap(), expected);
        }
    }

    #[test]
    fn test_string_escapes() {
        let input = r#""Hello \"hello\\nnew line\" ""#;
        let expect = r#""Hello \"hello\nnew line\" ""#;
        let mut fixer = JsonFixer::new(input);
        let output = fixer.fix().unwrap();
        println!("input : {:?}", input);
        println!("expect : {:?}", expect);
        println!("output : {:?}", output);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_numbers() {
        let cases = vec![
            (r#"{"num1": .123}"#, r#"{"num1":0.123}"#),
            (r#"{"num2": 123.}"#, r#"{"num2":123}"#),
            (r#"{"num": 42}"#, r#"{"num":42}"#),
            (r#"{"num": -42}"#, r#"{"num":-42}"#),
            (r#"{"num": 3.14}"#, r#"{"num":3.14}"#),
            (r#"{"num": -3.14}"#, r#"{"num":-3.14}"#,),
            (r#"{"num": 1e5}"#, r#"{"num":1e5}"#),
            (r#"{"num": 1.23e-4}"#, r#"{"num":1.23e-4}"#),
            (r#"{"num": -1.23e+4}"#, r#"{"num":-1.23e+4}"#),
        ];

        for input in cases {
            let mut fixer = JsonFixer::new(input.0);
            let result = fixer.fix();
           // println!("Test Error2: {:?}" ,result);
            assert_eq!(result.unwrap(), input.1);
        }
    }

    #[test]
    fn test_boolean_and_null() {
        let input = r#"{"active": true, "verified": false, "data": null}"#;
        let output = r#"{"active":true,"verified":false,"data":null}"#;
        let mut fixer = JsonFixer::new(input);
        assert_eq!(fixer.fix().unwrap(), output);
    }

    #[test]
    fn test_error_unmatched_quotes() {
        let input = r#"{"name": "John"#;
        let mut fixer = JsonFixer::new(input);
        assert!(matches!(fixer.fix(), Err(JsonFixerError::Syntax(SyntaxError::UnmatchedQuotes(_)))));
    }

    #[test]
    fn test_error_unexpected_end() {
        let input = r#"{"name": "John", p"#;
        let mut fixer = JsonFixer::new(input);
        let result = fixer.fix();
        //println!("Error : {:?}",result);
        assert!(matches!(result, Err(JsonFixerError::Syntax(SyntaxError::UnexpectedEndOfInput(_)))));
    }

    #[test]
    fn test_error_invalid_number() {
        let cases = vec![
            
            (r#"{"num3": 1.2.3}"#, r#"{"num3":1.2.3}"#),
            (r#"{"num4": --123}"#, r#"{"num4":--123}"#),
            (r#"{"num5": 1e}"#, r#"{"num5":1e}"#),
        ];

        for input in cases {
            let mut fixer = JsonFixer::new(input.0);
            let result = fixer.fix();

            //println!("Test Error1: {:?}" ,result);
            assert!(matches!(result, Err(JsonFixerError::Syntax(SyntaxError::InvalidNumber(_, _)))));
        }
    }

    #[test]
    fn test_error_unexpected_token() {
        let input = r#"{"name" _: "John", "age": 30}"#;  // Missing comma
        let mut fixer = JsonFixer::new(input);
        let result = fixer.fix();
        //println!("Test Error0: {:?}" ,result);

        assert!(matches!(result, Err(JsonFixerError::Syntax(SyntaxError::UnexpectedToken(_, _)))));
    }
    #[test]
    fn test_fix_missing_comma() {
        let input = r#"{"name": "John" "age": 30 "id": 0 }"#;
        let output = r#"{"name":"John","age":30,"id":0}"#;
        let mut fixer = JsonFixer::new(input);
        let result = fixer.fix();
       
        assert_eq!(result.unwrap(), output);
        
    }

    /*
    ************************** Preserve *************************
     */
    
    #[test]
    fn test_object_preserve() {
        let inputs = vec![
            ("{   }", "{   }"),
            (r#"{  
            
            }"#,
            r#"{  
            
            }"#
            ),
            (r#"{  

            }"#,
            r#"{  

            }"#
            ),
            (r#"{  
                "key1": 30,
            }"#,
            r#"{  
                "key1": 30
            }"#
            ),
            (r#"{  
                "key1": 30,
                key2 : "value",
                key3 : {
                    other : 12,
                    name : "hashdiese"
                    numbers: [  1, 2, 
                    3,
                    ],
                }
            }"#,
            r#"{  
                "key1": 30,
                "key2" : "value",
                "key3" : {
                    "other" : 12,
                    "name" : "hashdiese",
                    "numbers": [  1, 2, 
                    3
                    ]
                }
            }"#
            )
        ];
        
        let mut config = JsonFixerConfig::default();
        config.preserve = true;
        //config.sort_keys = true;

        for input in inputs {
            
            let mut fixer = JsonFixer::with_config(input.0, config.clone());
            let result = fixer.fix().unwrap();
            println!("Input     : {}", input.0);
            println!("Expected  : {}", input.1);
            println!("Output    : {}", result);
            assert_eq!(result, input.1);
        }
    }

    #[test]
    fn test_object_unpreserve() {
        let inputs = vec![
            ("{   }", "{}"),
            (r#"{  
            
            }"#,
            r#"{}"#
            ),
            (r#"{  

            }"#,
            r#"{}"#
            ),
            (r#"{  
                "key1": 30,
            }"#,
            r#"{"key1":30}"#
            ),
            (r#"{  
                "key1": 30,
                key2 : "value",
                key3 : {
                    other : 12,
                    name : "hashdiese"
                    numbers: [  1, 2, 
                    3,
                    ],
                }
            }"#,
            r#"{"key1":30,"key2":"value","key3":{"other":12,"name":"hashdiese","numbers":[1,2,3]}}"#
            )
        ];
        
        let mut config = JsonFixerConfig::default();
        config.preserve = false;
        config.sort_keys = false;

        for input in inputs {
            
            let mut fixer = JsonFixer::with_config(input.0, config.clone());
            let result = fixer.fix().unwrap();
            println!("Input     : {}", input.0);
            println!("Expected  : {}", input.1);
            println!("Output    : {}", result);
            assert_eq!(result, input.1);
        }
    }

    #[test]
    fn test_object_space_between() {
        let inputs = vec![
            ("{   }", "{}"),
            (r#"{  
            
            }"#,
            r#"{}"#
            ),
            (r#"{  

            }"#,
            r#"{}"#
            ),
            (r#"{  
                "key1": 30,
            }"#,
            r#"{ "key1": 30 }"#
            ),
            (r#"{  
                "key1": 30,
                key2 : "value",
                key3 : {
                    other : 12,
                    name : "hashdiese"
                    numbers: [  1, 2, 
                    3,
                    ],
                }
            }"#,
            r#"{ "key1": 30, "key2": "value", "key3": { "other": 12, "name": "hashdiese", "numbers": [ 1, 2, 3 ] } }"#
            )
        ];
        
        let mut config = JsonFixerConfig::default();
        config.preserve = false;
        config.sort_keys = false;
        config.space_between = true;

        for input in inputs {
            
            let mut fixer = JsonFixer::with_config(input.0, config.clone());
            let result = fixer.fix().unwrap();
            println!("Input     : {}", input.0);
            println!("Expected  : {}", input.1);
            println!("Output    : {}", result);
            assert_eq!(result, input.1);
        }
    }
    #[test]
    fn test_object_pretty() {
        let inputs = vec![
            ("{   }", "{}"),
            (r#"{  
            
            }"#,
            r#"{}"#
            ),
            (r#"{  

            }"#,
            r#"{}"#
            ),
            (r#"  {  
                "key1": 30,
            }  "#,
            r#"{
    "key1": 30
}"#
            ),
            (r#"{  
                "key1": 30,
                key2 : "value",
                key3 : {
                    other : 12,
                    name : "hashdiese"
                    numbers: [  1, 2, 
                    3,
                    ],
                }
            }"#,
            r#"{
    "key1": 30,
    "key2": "value",
    "key3": {
        "other": 12,
        "name": "hashdiese",
        "numbers": [
            1,
            2,
            3
        ]
    }
}"#
            )
        ];
        let __inputs = vec![ (
        r#"{  
            "key3" : {
                "other" : 12,

                ,

                lang: "fr",
            }
            
        }"#,
        r#"{  
            "key3" : {

                "lang": "fr",
                "other" : 12
            }
            
        }"#
        )
    ];
        let mut config = JsonFixerConfig::default();
        config.preserve = false;
        config.sort_keys = false;
        config.space_between = false;
        config.beautify = true;
        config.indent_size = 4;

        for input in inputs {
            
            let mut fixer = JsonFixer::with_config(input.0, config.clone());
            let result = fixer.fix().unwrap();
            println!("Input     : {}", input.0);
            println!("Expected  : {}", input.1);
            println!("Output    : {}", result);
            assert_eq!(result, input.1);
        }
    }

}
