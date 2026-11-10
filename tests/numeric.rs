mod support;

use support::Case;

fn runs(source: &str, input: &str, expected: &str) {
    let case = Case::new(source);
    for options in [&[][..], &["--no-opt"][..]] {
        let output = case.run_with_options(input, options);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, expected.as_bytes());
    }
}

#[test]
fn float_uses_ordinary_shared_values_and_ieee_arithmetic() {
    runs(
        include_str!("../examples/beta09/float.rusk"),
        "",
        "0.00125 0.00125 0.0025\n2.500 -3 -2 -1 -2\n-0 -inf Yes No Yes\nYes\n",
    );
}

#[test]
fn float_tokens_patterns_and_input_round_trip() {
    runs(
        r#"
in a, b, c, d, e: Float;
wt a, b, c, d, e;
wt is_finite(a), is_inf(d), e < a, e <= a, e > a, e >= a;
wt Float(Str(a)) == a, Float(Str(b)) == b, 1.0 / Float(Str(c));
wt format_fixed(-a, 1000).len(), format_fixed(d, 3), format_fixed(c, 2);
let pattern := match -1.0e0 {0.0 => 0, -1e0 => 1, _ => 2,};
let zero := match -0.0 {0.0 => 1, _ => 0,};
wt pattern, zero, [1.0, 2E+0], (Float(3), 4e0);
let total := 0;
for i in 0..3 {total += i;}
wt total, Int(-9.223372036854776e18), try_read::<Float>().is_none();
"#,
        "1.7976931348623157e308 5e-324 -0 +inf nan",
        "1.7976931348623157e+308 5e-324 -0 inf nan\nYes Yes No No No No\nYes Yes -inf\n1311 inf -0.00\n1 1 1 2 3 4\n3 -9223372036854775808 Yes\n",
    );
}

#[test]
fn float_reports_bad_literals_and_missing_implicit_conversions() {
    for (source, expected) in [
        ("wt 1e400;", "Float literal out of range"),
        ("wt 1e-400;", "Float literal out of range"),
        ("let x:Float:=1;", "expected Float"),
        ("wt 1.0+2;", "no matching"),
    ] {
        let output = Case::new(source).compile();
        assert!(!output.status.success(), "{source}");
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(
            error.contains("main.rusk:1:") && error.contains(expected),
            "{error}"
        );
    }
}

#[test]
fn invalid_float_input_conversions_and_precision_report_locations() {
    for (source, input, expected) in [
        ("in x:Float;", "1e-400", "Float out of range"),
        ("in x:Float;", "1e400", "Float out of range"),
        ("in x:Float;", "4e+", "invalid Float"),
        ("wt Float(\"0x1p2\");", "", "invalid Float"),
        (
            "wt Int(Float(\"inf\"));",
            "",
            "Float cannot be converted to Int",
        ),
        (
            "wt Int(Float(\"nan\"));",
            "",
            "Float cannot be converted to Int",
        ),
        (
            "wt Int(9.223372036854776e18);",
            "",
            "Float cannot be converted to Int",
        ),
        ("wt format_fixed(1.0, -1);", "", "Float precision"),
        ("wt format_fixed(1.0, 1001);", "", "Float precision"),
    ] {
        let output = Case::new(source).run(input);
        assert_eq!(output.status.code(), Some(1), "{source}");
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(
            error.contains("main.rusk:1:") && error.contains(expected),
            "{error}"
        );
    }
}
