extern crate micromark;
use micromark::micromark;

#[test]
fn dangerous_protocol_autolink() {
    assert_eq!(
        micromark("<javascript:alert(1)>"),
        "<p><a href=\"\">javascript:alert(1)</a></p>",
        "should be safe by default"
    );

    assert_eq!(
        micromark("<http://a>"),
        "<p><a href=\"http://a\">http://a</a></p>",
        "should allow `http:`"
    );

    assert_eq!(
        micromark("<https://a>"),
        "<p><a href=\"https://a\">https://a</a></p>",
        "should allow `https:`"
    );

    assert_eq!(
        micromark("<irc:///help>"),
        "<p><a href=\"irc:///help\">irc:///help</a></p>",
        "should allow `irc:`"
    );

    assert_eq!(
        micromark("<mailto:a>"),
        "<p><a href=\"mailto:a\">mailto:a</a></p>",
        "should allow `mailto:`"
    );
}

#[test]
fn dangerous_protocol_image() {
    assert_eq!(
        micromark("![](javascript:alert(1))"),
        "<p><img src=\"\" alt=\"\" /></p>",
        "should be safe by default"
    );

    assert_eq!(
        micromark("![](http://a)"),
        "<p><img src=\"http://a\" alt=\"\" /></p>",
        "should allow `http:`"
    );

    assert_eq!(
        micromark("![](https://a)"),
        "<p><img src=\"https://a\" alt=\"\" /></p>",
        "should allow `https:`"
    );

    assert_eq!(
        micromark("![](irc:///help)"),
        "<p><img src=\"\" alt=\"\" /></p>",
        "should not allow `irc:`"
    );

    assert_eq!(
        micromark("![](mailto:a)"),
        "<p><img src=\"\" alt=\"\" /></p>",
        "should not allow `mailto:`"
    );

    assert_eq!(
        micromark("![](#a)"),
        "<p><img src=\"#a\" alt=\"\" /></p>",
        "should allow a hash"
    );

    assert_eq!(
        micromark("![](?a)"),
        "<p><img src=\"?a\" alt=\"\" /></p>",
        "should allow a search"
    );

    assert_eq!(
        micromark("![](/a)"),
        "<p><img src=\"/a\" alt=\"\" /></p>",
        "should allow an absolute"
    );

    assert_eq!(
        micromark("![](./a)"),
        "<p><img src=\"./a\" alt=\"\" /></p>",
        "should allow an relative"
    );

    assert_eq!(
        micromark("![](../a)"),
        "<p><img src=\"../a\" alt=\"\" /></p>",
        "should allow an upwards relative"
    );

    assert_eq!(
        micromark("![](a#b:c)"),
        "<p><img src=\"a#b:c\" alt=\"\" /></p>",
        "should allow a colon in a hash"
    );

    assert_eq!(
        micromark("![](a?b:c)"),
        "<p><img src=\"a?b:c\" alt=\"\" /></p>",
        "should allow a colon in a search"
    );

    assert_eq!(
        micromark("![](a/b:c)"),
        "<p><img src=\"a/b:c\" alt=\"\" /></p>",
        "should allow a colon in a path"
    );
}

#[test]
fn dangerous_protocol_link() {
    assert_eq!(
        micromark("[](javascript:alert(1))"),
        "<p><a href=\"\"></a></p>",
        "should be safe by default"
    );

    assert_eq!(
        micromark("[](http://a)"),
        "<p><a href=\"http://a\"></a></p>",
        "should allow `http:`"
    );

    assert_eq!(
        micromark("[](https://a)"),
        "<p><a href=\"https://a\"></a></p>",
        "should allow `https:`"
    );

    assert_eq!(
        micromark("[](irc:///help)"),
        "<p><a href=\"irc:///help\"></a></p>",
        "should allow `irc:`"
    );

    assert_eq!(
        micromark("[](mailto:a)"),
        "<p><a href=\"mailto:a\"></a></p>",
        "should allow `mailto:`"
    );

    assert_eq!(
        micromark("[](#a)"),
        "<p><a href=\"#a\"></a></p>",
        "should allow a hash"
    );

    assert_eq!(
        micromark("[](?a)"),
        "<p><a href=\"?a\"></a></p>",
        "should allow a search"
    );

    assert_eq!(
        micromark("[](/a)"),
        "<p><a href=\"/a\"></a></p>",
        "should allow an absolute"
    );

    assert_eq!(
        micromark("[](./a)"),
        "<p><a href=\"./a\"></a></p>",
        "should allow an relative"
    );

    assert_eq!(
        micromark("[](../a)"),
        "<p><a href=\"../a\"></a></p>",
        "should allow an upwards relative"
    );

    assert_eq!(
        micromark("[](a#b:c)"),
        "<p><a href=\"a#b:c\"></a></p>",
        "should allow a colon in a hash"
    );

    assert_eq!(
        micromark("[](a?b:c)"),
        "<p><a href=\"a?b:c\"></a></p>",
        "should allow a colon in a search"
    );

    assert_eq!(
        micromark("[](a/b:c)"),
        "<p><a href=\"a/b:c\"></a></p>",
        "should allow a colon in a path"
    );
}
