extern crate hc128;
extern crate data_encoding;

use data_encoding::HEXUPPER;
use hc128::HC128;


macro_rules! hex {
    ( $value:expr ) => {
        HEXUPPER.decode($value).unwrap()
    }
}
macro_rules! test {
    (
        $key:expr,
        $iv:expr,
        [ $(
            $range:expr => $value:expr,
        )+ ]
    ) => {
        $(
            assert_eq!(
                HC128::new(&$key, &$iv)
                    .skip($range.start)
                    .take($range.end - $range.start + 1)
                    .collect::<Vec<_>>(),
                $value
            );
        )+
    }
}

#[test]
fn test() {
    test!{
        hex!(b"80000000000000000000000000000000"),
        hex!(b"00000000000000000000000000000000"),
        [
            0..63 => hex!(b"378602B98F32A74847515654AE0DE7ED\
                        8F72BC34776A065103E51595521FFE47\
                        F9AF0A4CB47999CFA26D33BF80954598\
                        9D53DEBFE7A9EFD8B9109CA6EFADDF83"),
            192..255 => hex!(b"E7F8DCC6A1D42ECF6A49651F7C610657\
                        B1DF6E58FBEF6A246D6D4CAA83858839\
                        86325BE2B4185B4D63D4BF766C5F4B73\
                        0B89C3CD66018155DFE9D37B6F5C1251"),
            256..319 => hex!(b"6D21763B2FEBADB212AC71388FF93586\
                        48AA1A0E874D3B6932D7F80A5657F88D\
                        A44BDC16AA21E531E3E473CFE6FCA9EE\
                        20739339CE4F2DAC793210C8CC20897F"),
            448..511 => hex!(b"5BB39DF39C64BFA13F2AAE924D3DF4FA\
                        22899838ADB609806C022C36180A3E46\
                        A547CFF7F4DE1151A81AED3646B2D86E\
                        1F0F3C22C92D3459593ED599D1A535DF"),
        ]
    };

    test!{
        hex!(b"0053A6F94C9FF24598EB3E91E4378ADD"),
        hex!(b"0D74DB42A91077DE45AC137AE148AF16"),
        [

               0..63 => hex!(b"2E1ED12A8551C05AF41FF39D8F9DF933\
                               122B5235D48FC2A6F20037E69BDBBCE8\
                               05782EFC16C455A4B3FF06142317535E\
                               F876104C32445138CB26EBC2F88A684C"),
        65472..65535 => hex!(b"1D92C4EBF6A256F0D0B0365160D72E90\
                               CA10D7086C58BE13E9325A5088F447D1\
                               572466248CD275A736B83674739899CA\
                               3146963E00E170C6B9DC8B2BE912A5C2"),
        65536..65599 => hex!(b"878A21CA440BA0D659F24A5C986D6CF0\
                               3EA0DD962337935BA0932FAD9599EF61\
                               D805800038AFE4208394C73AA044262C\
                               18490F742A2B7424ED56EF3D1B0F53AF"),
      131008..131071 => hex!(b"99387AFF42EE8C9D4D8400808322114C\
                               F4DF77CDAA363B0E4AFD0D8FF17D3D2C\
                               3303984867021922368A76F7CBD20266\
                               5A962140C8E6C1336CC4071B38ABB957"),
        ]
    };

    test!{
        hex!(b"0A5DB00356A9FC4FA2F5489BEE4194E7"),
        hex!(b"1F86ED54BB2289F057BE258CF35AC128"),
        [
               0..63 => hex!(b"82168AB0023B79AAF1E6B4D823855E14\
                               A7084378036A951B1CFEF35173875ED8\
                               6CB66AB8410491A08582BE40080C3102\
                               193BA567F9E95D096C3CC60927DD7901"),
        65472..65535 => hex!(b"2A30BFDE279B750D56B0B10A79BDA0DB\
                               21C246D133F4B91E4ECAF80DA7AAC425\
                               646523F6BB762D688BFE2DB1852B77E7\
                               733BC1005CF3D7CFAEC4BD966DCA6773"),
        65536..65599 => hex!(b"991EC57DE1BDFFE2C70A0196A8902C91\
                               D3CE6C63E4B8D81C83AABE7BF370D1B5\
                               4D0B72B0C3C857621A7BBE2B72EBD81F\
                               50B25E08A9D492AFDDD37B983E9E2E4A"),
      131008..131071 => hex!(b"BC301B9FD7C554C592EFD092A435C2C6\
                               E74CBBF905CE424FE5872EEFE8DC62BF\
                               F93C3917BD37D142CFCA623B84C2652E\
                               0E61BB5C5D5387AD95EBA7A5ADF16F81"),
        ]
    };

    test!{
        hex!(b"0F62B5085BAE0154A7FA4DA0F34699EC"),
        hex!(b"288FF65DC42B92F960C72E95FC63CA31"),
        [
               0..63 => hex!(b"1CD8AEDDFE52E217E835D0B7E84E2922\
                               D04B1ADBCA53C4522B1AA604C42856A9\
                               0AF83E2614BCE65C0AECABDD8975B557\
                               00D6A26D52FFF0888DA38F1DE20B77B7"),
        65472..65535 => hex!(b"BB599F93F4F244D717CA9818212B06D5\
                               6D99AD4CA1F78725DBA89EA1D1F05B27\
                               093A17D745396D8CFD0256CD50674046\
                               13108E2200A8F1C49075B376A7460515"),
        65536..65599 => hex!(b"996C074A7C7C524F539037A8A9F3D193\
                               3BC311B548BD567F8AE1B4325C51C5F3\
                               4B0DE1B4A4651829108CA92AE23D57C7\
                               0EAFA766097DB0539BE77E6500703746"),
      131008..131071 => hex!(b"43EF1ADFE8265C46FF7FBA43B78F899F\
                               22C3B9F069B786982145D601627CDC49\
                               2D27BB8D70FF6DA908F2606A0C44690C\
                               8502F9CFB3BD6CBFC9205470E3ABA387"),
        ]
    }
}
