#[cfg(test)]
mod gen_num {
    use crate::assert_evals_to;
    use crate::assert_llvm_evals_to;
    use indoc::indoc;
    use roc_std::RocOrder;

    #[test]
    fn nat_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : Nat
                    i = 1

                    i
                "#
            ),
            1,
            usize
        );
    }

    #[test]
    fn i128_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : I128
                    i = 128

                    i
                "#
            ),
            128,
            i128
        );
    }
    #[test]
    fn i64_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                app "test" provides [ main ] to "./platform"

                main =
                    i : I64
                    i = 64

                    i
                "#
            ),
            64,
            i64
        );
    }
    #[test]
    fn i32_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : I32
                    i = 32

                    i
                "#
            ),
            32,
            i32
        );
    }
    #[test]
    fn i16_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : I16
                    i = 16

                    i
                "#
            ),
            16,
            i16
        );
    }
    #[test]
    fn i8_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : I8
                    i = 8

                    i
                "#
            ),
            8,
            i8
        );
    }

    #[test]
    fn i128_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : I128
                    f = 0x123

                    f
                "#
            ),
            0x123,
            i128
        );
    }
    #[test]
    fn i64_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : I64
                    f = 0x123

                    f
                "#
            ),
            0x123,
            i64
        );
    }
    #[test]
    fn i32_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : I32
                    f = 0x123

                    f
                "#
            ),
            0x123,
            i32
        );
    }
    #[test]
    fn i16_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : I16
                    f = 0x123

                    f
                "#
            ),
            0x123,
            i16
        );
    }
    #[test]
    fn i8_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : I8
                    f = 0xA

                    f
                "#
            ),
            0xA,
            i8
        );
    }

    #[test]
    fn u128_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : U128
                    i = 128

                    i
                "#
            ),
            128,
            u128
        );
    }
    #[test]
    fn u64_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : U64
                    i = 64

                    i
                "#
            ),
            64,
            u64
        );
    }
    #[test]
    fn u32_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : U32
                    i = 32

                    i
                "#
            ),
            32,
            u32
        );
    }
    #[test]
    fn u16_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : U16
                    i = 16

                    i
                "#
            ),
            16,
            u16
        );
    }
    #[test]
    fn u8_signed_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    i : U8
                    i = 8

                    i
                "#
            ),
            8,
            u8
        );
    }

    #[test]
    fn u128_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : U128
                    f = 0x123

                    f
                "#
            ),
            0x123,
            i128
        );
    }
    #[test]
    fn u64_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : U64
                    f = 0x123

                    f
                "#
            ),
            0x123,
            u64
        );
    }
    #[test]
    fn u32_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : U32
                    f = 0x123

                    f
                "#
            ),
            0x123,
            u32
        );
    }
    #[test]
    fn u16_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : U16
                    f = 0x123

                    f
                "#
            ),
            0x123,
            u16
        );
    }
    #[test]
    fn u8_hex_int_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : U8
                    f = 0xA

                    f
                "#
            ),
            0xA,
            u8
        );
    }

    #[test]
    fn f64_float_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : F64
                    f = 3.6

                    f
                "#
            ),
            3.6,
            f64
        );
    }
    #[test]
    fn f32_float_alias() {
        assert_evals_to!(
            indoc!(
                r#"
                    f : F32
                    f = 3.6

                    f
                "#
            ),
            3.6,
            f32
        );
    }

    #[test]
    fn f64_sqrt() {
        // FIXME this works with normal types, but fails when checking uniqueness types
        assert_evals_to!(
            indoc!(
                r#"
                    when Num.sqrt 100 is
                        Ok val -> val
                        Err _ -> -1
                "#
            ),
            10.0,
            f64
        );
    }

    #[test]
    fn f64_round_old() {
        assert_evals_to!("Num.round 3.6", 4, i64);
    }

    #[test]
    fn f64_abs() {
        assert_evals_to!("Num.abs -4.7", 4.7, f64);
        assert_evals_to!("Num.abs 5.8", 5.8, f64);
        //assert_evals_to!("Num.abs Num.maxFloat", f64::MAX, f64);
        //assert_evals_to!("Num.abs Num.minFloat", -f64::MIN, f64);
    }

    #[test]
    fn i64_abs() {
        assert_evals_to!("Num.abs -6", 6, i64);
        assert_evals_to!("Num.abs 7", 7, i64);
        assert_evals_to!("Num.abs 0", 0, i64);
        assert_evals_to!("Num.abs -0", 0, i64);
        assert_evals_to!("Num.abs -1", 1, i64);
        assert_evals_to!("Num.abs 1", 1, i64);
        assert_evals_to!("Num.abs 9_000_000_000_000", 9_000_000_000_000, i64);
        assert_evals_to!("Num.abs -9_000_000_000_000", 9_000_000_000_000, i64);
        assert_evals_to!("Num.abs Num.maxInt", i64::MAX, i64);
        assert_evals_to!("Num.abs (Num.minInt + 1)", -(i64::MIN + 1), i64);
    }

    #[test]
    #[should_panic(
        expected = r#"Roc failed with message: "integer absolute overflowed because its argument is the minimum value"#
    )]
    fn abs_min_int_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.abs Num.minInt
                "#
            ),
            0,
            i64
        );
    }

    #[test]
    fn gen_if_fn() {
        assert_evals_to!(
            indoc!(
                r#"
                    limitedNegate = \num ->
                        x =
                            if num == 1 then
                                -1
                            else if num == -1 then
                                1
                            else
                                num
                        x

                    limitedNegate 1
                "#
            ),
            -1,
            i64
        );
    }

    #[test]
    fn gen_float_eq() {
        assert_evals_to!(
            indoc!(
                r#"
                    1.0 == 1.0
                "#
            ),
            true,
            bool
        );
    }

    #[test]
    fn gen_add_f64() {
        assert_evals_to!(
            indoc!(
                r#"
                    1.1 + 2.4 + 3
                "#
            ),
            6.5,
            f64
        );
    }

    #[test]
    fn gen_wrap_add_nums() {
        assert_evals_to!(
            indoc!(
                r#"
                    add2 = \num1, num2 -> num1 + num2

                    add2 4 5
                "#
            ),
            9,
            i64
        );
    }

    #[test]
    fn gen_div_f64() {
        // FIXME this works with normal types, but fails when checking uniqueness types
        assert_evals_to!(
            indoc!(
                r#"
                    when 48 / 2 is
                        Ok val -> val
                        Err _ -> -1
                "#
            ),
            24.0,
            f64
        );
    }

    #[test]
    fn gen_int_eq() {
        assert_evals_to!(
            indoc!(
                r#"
                    4 == 4
                "#
            ),
            true,
            bool
        );
    }

    #[test]
    fn gen_int_neq() {
        assert_evals_to!(
            indoc!(
                r#"
                    4 != 5
                "#
            ),
            true,
            bool
        );
    }

    #[test]
    fn gen_wrap_int_neq() {
        assert_evals_to!(
            indoc!(
                r#"
                    wrappedNotEq : a, a -> Bool
                    wrappedNotEq = \num1, num2 ->
                        num1 != num2

                    wrappedNotEq 2 3
                "#
            ),
            true,
            bool
        );
    }

    #[test]
    fn gen_add_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    1 + 2 + 3
                "#
            ),
            6,
            i64
        );
    }

    #[test]
    fn gen_sub_f64() {
        assert_evals_to!(
            indoc!(
                r#"
                    1.5 - 2.4 - 3
                "#
            ),
            -3.9,
            f64
        );
    }

    #[test]
    fn gen_sub_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    1 - 2 - 3
                "#
            ),
            -4,
            i64
        );
    }

    #[test]
    fn gen_mul_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    2 * 4 * 6
                "#
            ),
            48,
            i64
        );
    }

    #[test]
    fn gen_div_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    when 1000 // 10 is
                        Ok val -> val
                        Err _ -> -1
                "#
            ),
            100,
            i64
        );
    }

    #[test]
    fn gen_div_by_zero_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    when 1000 // 0 is
                        Err DivByZero -> 99
                        _ -> -24
                "#
            ),
            99,
            i64
        );
    }

    #[test]
    fn gen_rem_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    when Num.rem 8 3 is
                        Ok val -> val
                        Err _ -> -1
                "#
            ),
            2,
            i64
        );
    }

    #[test]
    fn gen_rem_div_by_zero_i64() {
        assert_evals_to!(
            indoc!(
                r#"
                    when Num.rem 8 0 is
                        Err DivByZero -> 4
                        Ok _ -> -23
                "#
            ),
            4,
            i64
        );
    }

    #[test]
    fn gen_is_zero_i64() {
        assert_evals_to!("Num.isZero 0", true, bool);
        assert_evals_to!("Num.isZero 1", false, bool);
    }

    #[test]
    fn gen_is_positive_i64() {
        assert_evals_to!("Num.isPositive 0", false, bool);
        assert_evals_to!("Num.isPositive 1", true, bool);
        assert_evals_to!("Num.isPositive -5", false, bool);
    }

    #[test]
    fn gen_is_negative_i64() {
        assert_evals_to!("Num.isNegative 0", false, bool);
        assert_evals_to!("Num.isNegative 3", false, bool);
        assert_evals_to!("Num.isNegative -2", true, bool);
    }

    #[test]
    fn gen_is_positive_f64() {
        assert_evals_to!("Num.isPositive 0.0", false, bool);
        assert_evals_to!("Num.isPositive 4.7", true, bool);
        assert_evals_to!("Num.isPositive -8.5", false, bool);
    }

    #[test]
    fn gen_is_negative_f64() {
        assert_evals_to!("Num.isNegative 0.0", false, bool);
        assert_evals_to!("Num.isNegative 9.9", false, bool);
        assert_evals_to!("Num.isNegative -4.4", true, bool);
    }

    #[test]
    fn gen_is_zero_f64() {
        assert_evals_to!("Num.isZero 0", true, bool);
        assert_evals_to!("Num.isZero 0_0", true, bool);
        assert_evals_to!("Num.isZero 0.0", true, bool);
        assert_evals_to!("Num.isZero 1", false, bool);
    }

    #[test]
    fn gen_is_odd() {
        assert_evals_to!("Num.isOdd 4", false, bool);
        assert_evals_to!("Num.isOdd 5", true, bool);
    }

    #[test]
    fn gen_is_even() {
        assert_evals_to!("Num.isEven 6", true, bool);
        assert_evals_to!("Num.isEven 7", false, bool);
    }

    #[test]
    fn sin() {
        assert_evals_to!("Num.sin 0", 0.0, f64);
        assert_evals_to!("Num.sin 1.41421356237", 0.9877659459922529, f64);
    }

    #[test]
    fn cos() {
        assert_evals_to!("Num.cos 0", 1.0, f64);
        assert_evals_to!("Num.cos 3.14159265359", -1.0, f64);
    }

    #[test]
    fn tan() {
        assert_evals_to!("Num.tan 0", 0.0, f64);
        assert_evals_to!("Num.tan 1", 1.557407724654902, f64);
    }

    #[test]
    fn bitwise_and() {
        assert_evals_to!("Num.bitwiseAnd 20 20", 20, i64);
        assert_evals_to!("Num.bitwiseAnd 25 10", 8, i64);
        assert_evals_to!("Num.bitwiseAnd 200 0", 0, i64);
    }

    #[test]
    fn bitwise_xor() {
        assert_evals_to!("Num.bitwiseXor 20 20", 0, i64);
        assert_evals_to!("Num.bitwiseXor 15 14", 1, i64);
        assert_evals_to!("Num.bitwiseXor 7 15", 8, i64);
        assert_evals_to!("Num.bitwiseXor 200 0", 200, i64);
    }

    #[test]
    fn bitwise_or() {
        assert_evals_to!("Num.bitwiseOr 1 1", 1, i64);
        assert_evals_to!("Num.bitwiseOr 1 2", 3, i64);
    }

    #[test]
    fn lt_i64() {
        assert_evals_to!("1 < 2", true, bool);
        assert_evals_to!("1 < 1", false, bool);
        assert_evals_to!("2 < 1", false, bool);
        assert_evals_to!("0 < 0", false, bool);
    }

    #[test]
    fn lte_i64() {
        assert_evals_to!("1 <= 1", true, bool);
        assert_evals_to!("2 <= 1", false, bool);
        assert_evals_to!("1 <= 2", true, bool);
        assert_evals_to!("0 <= 0", true, bool);
    }

    #[test]
    fn gt_i64() {
        assert_evals_to!("2 > 1", true, bool);
        assert_evals_to!("2 > 2", false, bool);
        assert_evals_to!("1 > 1", false, bool);
        assert_evals_to!("0 > 0", false, bool);
    }

    #[test]
    fn gte_i64() {
        assert_evals_to!("1 >= 1", true, bool);
        assert_evals_to!("1 >= 2", false, bool);
        assert_evals_to!("2 >= 1", true, bool);
        assert_evals_to!("0 >= 0", true, bool);
    }

    #[test]
    fn lt_f64() {
        assert_evals_to!("1.1 < 1.2", true, bool);
        assert_evals_to!("1.1 < 1.1", false, bool);
        assert_evals_to!("1.2 < 1.1", false, bool);
        assert_evals_to!("0.0 < 0.0", false, bool);
    }

    #[test]
    fn lte_f64() {
        assert_evals_to!("1.1 <= 1.1", true, bool);
        assert_evals_to!("1.2 <= 1.1", false, bool);
        assert_evals_to!("1.1 <= 1.2", true, bool);
        assert_evals_to!("0.0 <= 0.0", true, bool);
    }

    #[test]
    fn gt_f64() {
        assert_evals_to!("2.2 > 1.1", true, bool);
        assert_evals_to!("2.2 > 2.2", false, bool);
        assert_evals_to!("1.1 > 2.2", false, bool);
        assert_evals_to!("0.0 > 0.0", false, bool);
    }

    #[test]
    fn gte_f64() {
        assert_evals_to!("1.1 >= 1.1", true, bool);
        assert_evals_to!("1.1 >= 1.2", false, bool);
        assert_evals_to!("1.2 >= 1.1", true, bool);
        assert_evals_to!("0.0 >= 0.0", true, bool);
    }

    #[test]
    fn gen_order_of_arithmetic_ops() {
        assert_evals_to!(
            indoc!(
                r#"
                    1 + 3 * 7 - 2
                "#
            ),
            20,
            i64
        );
    }

    #[test]
    fn gen_order_of_arithmetic_ops_complex_float() {
        assert_evals_to!(
            indoc!(
                r#"
                    3 - 48 * 2.0
                "#
            ),
            -93.0,
            f64
        );
    }

    #[test]
    fn if_guard_bind_variable_false() {
        assert_evals_to!(
            indoc!(
                r#"
                wrapper = \{} ->
                    when 10 is
                        x if x == 5 -> 0
                        _ -> 42

                wrapper {}
                "#
            ),
            42,
            i64
        );
    }

    #[test]
    fn if_guard_bind_variable_true() {
        assert_evals_to!(
            indoc!(
                r#"
                wrapper = \{} ->
                    when 10 is
                        x if x == 10 -> 42
                        _ -> 0

                wrapper {}
                "#
            ),
            42,
            i64
        );
    }

    #[test]
    fn tail_call_elimination() {
        assert_evals_to!(
            indoc!(
                r#"
                    sum = \n, accum ->
                        when n is
                            0 -> accum
                            _ -> sum (n - 1) (n + accum)

                    sum 1_000_000 0
                "#
            ),
            500000500000,
            i64
        );
    }

    #[test]
    fn int_negate() {
        assert_evals_to!("Num.neg 123", -123, i64);
        assert_evals_to!("Num.neg Num.maxInt", -i64::MAX, i64);
        assert_evals_to!("Num.neg (Num.minInt + 1)", i64::MAX, i64);
    }

    #[test]
    #[should_panic(
        expected = r#"Roc failed with message: "integer negation overflowed because its argument is the minimum value"#
    )]
    fn neg_min_int_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.neg Num.minInt
                "#
            ),
            0,
            i64
        );
    }

    #[test]
    fn gen_wrap_int_neg() {
        assert_evals_to!(
            indoc!(
                r#"
                    wrappedNeg = \num -> -num

                    wrappedNeg 3
                "#
            ),
            -3,
            i64
        );
    }

    #[test]
    fn gen_basic_fn() {
        assert_evals_to!(
            indoc!(
                r#"
                    always42 : Num.Num (Num.Integer Num.Signed64) -> Num.Num (Num.Integer Num.Signed64)
                    always42 = \_ -> 42

                    always42 5
                "#
            ),
            42,
            i64
        );
    }

    #[test]
    fn int_to_float() {
        assert_evals_to!("Num.toFloat 0x9", 9.0, f64);
    }

    #[test]
    fn num_to_float() {
        assert_evals_to!("Num.toFloat 9", 9.0, f64);
    }

    #[test]
    fn float_to_float() {
        assert_evals_to!("Num.toFloat 0.5", 0.5, f64);
    }

    #[test]
    fn int_compare() {
        assert_evals_to!("Num.compare 0 1", RocOrder::Lt, RocOrder);
        assert_evals_to!("Num.compare 1 1", RocOrder::Eq, RocOrder);
        assert_evals_to!("Num.compare 1 0", RocOrder::Gt, RocOrder);
    }

    #[test]
    fn float_compare() {
        assert_evals_to!("Num.compare 0.01 3.14", RocOrder::Lt, RocOrder);
        assert_evals_to!("Num.compare 3.14 3.14", RocOrder::Eq, RocOrder);
        assert_evals_to!("Num.compare 3.14 0.01", RocOrder::Gt, RocOrder);
    }

    #[test]
    fn pow() {
        assert_evals_to!("Num.pow 2.0 2.0", 4.0, f64);
    }

    #[test]
    fn ceiling() {
        assert_evals_to!("Num.ceiling 1.1", 2, i64);
    }

    #[test]
    fn floor() {
        assert_evals_to!("Num.floor 1.9", 1, i64);
    }

    #[test]
    fn pow_int() {
        assert_evals_to!("Num.powInt 2 3", 8, i64);
    }

    #[test]
    fn atan() {
        assert_evals_to!("Num.atan 10", 1.4711276743037347, f64);
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "integer addition overflowed!"#)]
    fn int_add_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                9_223_372_036_854_775_807 + 1
                "#
            ),
            0,
            i64
        );
    }

    #[test]
    fn int_add_checked() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.addChecked 1 2 is
                    Ok v -> v
                    _ -> -1
                "#
            ),
            3,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                when Num.addChecked 9_223_372_036_854_775_807 1 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
            ),
            -1,
            i64
        );
    }

    #[test]
    fn int_add_wrap() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.addWrap 9_223_372_036_854_775_807 1
                "#
            ),
            std::i64::MIN,
            i64
        );
    }

    #[test]
    fn float_add_checked_pass() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.addChecked 1.0 0.0 is
                    Ok v -> v
                    Err Overflow -> -1.0
                "#
            ),
            1.0,
            f64
        );
    }

    #[test]
    fn float_add_checked_fail() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.addChecked 1.7976931348623157e308 1.7976931348623157e308 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
            ),
            -1.0,
            f64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "float addition overflowed!"#)]
    fn float_add_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                    1.7976931348623157e308 + 1.7976931348623157e308
                    "#
            ),
            0.0,
            f64
        );
    }

    #[test]
    fn num_max_int() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.maxInt
                "#
            ),
            i64::MAX,
            i64
        );
    }

    #[test]
    fn num_min_int() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.minInt
                "#
            ),
            i64::MIN,
            i64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "integer subtraction overflowed!"#)]
    fn int_sub_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                -9_223_372_036_854_775_808 - 1
                "#
            ),
            0,
            i64
        );
    }

    #[test]
    fn int_sub_wrap() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.subWrap -9_223_372_036_854_775_808 1
                "#
            ),
            std::i64::MAX,
            i64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "float subtraction overflowed!"#)]
    fn float_sub_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                    -1.7976931348623157e308 - 1.7976931348623157e308
                "#
            ),
            0.0,
            f64
        );
    }

    #[test]
    fn int_sub_checked() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.subChecked 5 2 is
                    Ok v -> v
                    _ -> -1
                "#
            ),
            3,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                when Num.subChecked Num.minInt 1 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
            ),
            -1,
            i64
        );
    }

    #[test]
    fn float_sub_checked() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.subChecked 1.0 0.0 is
                    Ok v -> v
                    Err Overflow -> -1.0
                "#
            ),
            1.0,
            f64
        );

        assert_evals_to!(
            indoc!(
                r#"
                when Num.subChecked -1.7976931348623157e308 1.7976931348623157e308 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
            ),
            -1.0,
            f64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "integer multiplication overflowed!"#)]
    fn int_positive_mul_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                9_223_372_036_854_775_807 * 2
                "#
            ),
            0,
            i64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "integer multiplication overflowed!"#)]
    fn int_negative_mul_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                (-9_223_372_036_854_775_808) * 2
                "#
            ),
            0,
            i64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "float multiplication overflowed!"#)]
    fn float_positive_mul_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                    1.7976931348623157e308 * 2
                "#
            ),
            0.0,
            f64
        );
    }

    #[test]
    #[should_panic(expected = r#"Roc failed with message: "float multiplication overflowed!"#)]
    fn float_negative_mul_overflow() {
        assert_evals_to!(
            indoc!(
                r#"
                    -1.7976931348623157e308 * 2
                "#
            ),
            0.0,
            f64
        );
    }

    #[test]
    fn int_mul_wrap() {
        assert_evals_to!(
            indoc!(
                r#"
                Num.mulWrap Num.maxInt 2
                "#
            ),
            -2,
            i64
        );
    }

    #[test]
    fn int_mul_checked() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.mulChecked 20 2 is
                    Ok v -> v
                    _ -> -1
                "#
            ),
            40,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                when Num.mulChecked Num.maxInt 2 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
            ),
            -1,
            i64
        );
    }

    #[test]
    fn float_mul_checked() {
        assert_evals_to!(
            indoc!(
                r#"
                when Num.mulChecked 20.0 2.0 is
                    Ok v -> v
                    Err Overflow -> -1.0
                "#
            ),
            40.0,
            f64
        );

        assert_evals_to!(
            indoc!(
                r#"
                when Num.mulChecked 1.7976931348623157e308 2 is
                    Err Overflow -> -1
                    Ok v -> v
                "#
            ),
            -1.0,
            f64
        );
    }

    #[test]
    fn shift_left_by() {
        assert_evals_to!("Num.shiftLeftBy 0 0b0000_0001", 0b0000_0001, i64);
        assert_evals_to!("Num.shiftLeftBy 1 0b0000_0001", 0b0000_0010, i64);
        assert_evals_to!("Num.shiftLeftBy 2 0b0000_0011", 0b0000_1100, i64);
    }

    #[test]
    #[ignore]
    fn shift_right_by() {
        // Sign Extended Right Shift
        assert_evals_to!("Num.shiftRightBy 0 0b0100_0000i8", 0b0001_0000, i8);
        assert_evals_to!("Num.shiftRightBy 1 0b1110_0000u8", 0b1111_0000u8 as i8, i8);
        assert_evals_to!("Num.shiftRightBy 2 0b1100_0000u8", 0b1111_0000u8 as i8, i8);
    }

    #[test]
    #[ignore]
    fn shift_right_zf_by() {
        // Logical Right Shift
        assert_evals_to!("Num.shiftRightBy 1 0b1100_0000u8", 0b0011_0000, i64);
        assert_evals_to!("Num.shiftRightBy 2 0b0000_0010u8", 0b0000_0001, i64);
        assert_evals_to!("Num.shiftRightBy 3 0b0000_1100u8", 0b0000_0011, i64);
    }
}