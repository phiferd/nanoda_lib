use crate::name::{is_id_first, is_id_rest};
use crate::tests::util::test_ctx;
use std::error::Error;
use std::borrow::Cow;

#[test]
fn pfx_test_anon() -> Result<(), Box<dyn Error>> {
    test_ctx(None, |ctx| {
        assert_eq!(ctx.get_pfx(ctx.anonymous()), ctx.anonymous());
    })
}

#[test]
fn pfx_test_str0() -> Result<(), Box<dyn Error>> {
    test_ctx(None, |ctx| {
        let aaa = ctx.str1("aaa");
        assert_eq!(ctx.get_pfx(aaa), aaa);
    })
}

#[test]
fn pfx_test_num0() -> Result<(), Box<dyn Error>> {
    test_ctx(None, |ctx| {
        let anon = ctx.anonymous();
        let n0 = ctx.num(anon, 123);
        assert_eq!(ctx.get_pfx(n0), n0);
    })
}

#[test]
fn pfx_test_str1() -> Result<(), Box<dyn Error>> {
    test_ctx(None, |ctx| {
        let bbb = ctx.alloc_string(Cow::from("bbb"));
        let ccc = ctx.alloc_string(Cow::from("ccc"));
        let a = ctx.str1("aaa");
        let b = ctx.str(a, bbb);
        let c = ctx.str(b, ccc);
        let d = ctx.num(c, 1234);
        let pfx1 = ctx.get_pfx(d);
        let pfx2 = ctx.get_pfx(pfx1);
        assert_eq!(pfx1, a);
        assert_eq!(pfx2, pfx1);
    })
}

#[test]
fn identifier_characters() {
    // Both ends of each of Lean's ranges, and the characters just outside them.
    let first =
        ['a', 'z', 'A', 'Z', '_', 'α', 'ω', 'Α', 'Ω', 'ϊ', 'ϻ', 'ἀ', '῾', '℀', '⅏', '𝒜', '𝖟', 'À', 'ÿ', 'Ā', 'ſ'];
    let rest = ['0', '9', '\'', '!', '?', '₀', '₉', 'ₐ', 'ₜ', 'ᵢ', 'ᵪ', 'ⱼ'];
    let neither = [
        'λ', 'Π', 'Σ', '×', '÷', 'ΐ', 'Ϊ', 'ΰ', 'ϼ', 'ỿ', '\u{1fff}', '\u{20ff}', '⅐', '𝒛', '𝖠', '¿', 'ƀ', 'ⁿ', '₊',
        '\u{208f}', '\u{209d}', 'ᵡ', 'ᵫ', 'ⱻ', 'ⱽ', '.', ' ', '«', '»', '-', '#',
    ];
    for c in first {
        assert!(is_id_first(c) && is_id_rest(c), "{c:?}");
    }
    for c in rest {
        assert!(!is_id_first(c) && is_id_rest(c), "{c:?}");
    }
    for c in neither {
        assert!(!is_id_first(c) && !is_id_rest(c), "{c:?}");
    }
}
