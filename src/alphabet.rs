use bitflags::bitflags;

bitflags! {
    pub struct Alphabets:u8 {
        const LOWERCASE = 0b00000001;
        const UPPERCASE = 0b00000010;
        const DIGIT     = 0b00000100;
        const SPECIAL   = 0b00001000;
        const EXTENDED  = 0b00010000;
    }
}

const LOWERCASE_CHARS:    &str = r#"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcde"#;
const UPPERCASE_CHARS:    &str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDE"#;
const DIGITS_CHARS:       &str = r#"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567"#;
const SPECIAL_CHARS:      &str = r#"~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?><~!@#$%^&*()-=_+{}[]|\';/.,?"#;
pub const EXTENDED_CHARS: &str = r#"¿ǃ¢£¤¥±«»×÷ǁǂ§©¬®°µ¶·¼½¾¹ƻ²³ƽƼªáÁàÀâÂäÄǎǍăĂāĀãÃåÅąĄǡǠǻǺǟǞæÆǽǼǣǢƀƁƃƂƅƄćĆċĊĉĈčČçÇƈƇƆďĎđĐƌƋƊðÐƍǆǅǄƉéÉèÈėĖêÊëËěĚĕĔēĒęĘǝƎƏƐƒƑǵġĠĝĜǧǦğĞģĢǥǤƓƔĥĤħĦƕƗƖĳĲȷĵĴǰĸǩǨķĶƙƘļĻƚłŁƛǉǈǇƜńŃǹǸňŇñÑņŅƝŉƞǌǋǊŋŊºóÓòÒôÔöÖǒǑŏŎōŌõÕǫǪőŐƟøØǿǾǭǬơƠƣƢœŒƥƤƦŕŔřŘŗŖśŚŝŜšŠşŞșȘƩƨƧƪßſťŤţŢƭƬƫƮțȚþÞŧŦúÚùÙûÛüÜǔǓŭŬūŪũŨůŮųŲűŰɄǘǗǜǛǚǙǖǕưƯƱƲŵŴƿýÝŷŶÿŸƴƳźŹżŻžŽƶƵƷǯǮƹƸƺƾ"#;

pub fn generate_alphabet(subalphabets: Alphabets) -> Vec<char> {
    let mut resulting_alphabet = Vec::new();
    if subalphabets.contains(Alphabets::UPPERCASE) {
        resulting_alphabet.extend(UPPERCASE_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::LOWERCASE) {
        resulting_alphabet.extend(LOWERCASE_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::DIGIT) {
        resulting_alphabet.extend(DIGITS_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::SPECIAL) {
        resulting_alphabet.extend(SPECIAL_CHARS.chars());
    }
    if subalphabets.contains(Alphabets::EXTENDED) {
        resulting_alphabet.extend(EXTENDED_CHARS.chars());
    }
    resulting_alphabet
}

