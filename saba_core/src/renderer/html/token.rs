use crate::renderer::html::attribute::Attribute;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer{
    state: State,
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer{
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }

    fn is_eof(&self) -> bool {
        self.pos > self.input.len()
    }

    // 現在の位置の文字を1文字返す
    fn consume_next_input(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        c
    }

    // タグトークンを作成する
    fn create_tag(&mut self, start_tag_token: bool) {
        if start_tag_token {
            self.latest_token = Some(HtmlToken::StartTag {
                tag: String::new(),
                self_closing: false,
                attributes: Vec::new(),
            });
        } else {
            self.latest_token = Some(HtmlToken::EndTag {
                tag: String::new(),
            });
        }
    }

    // 現在の位置から一つ戻った位置の文字を返す
    fn reconsume_input(&mut self) -> char {
        self.reconsume = false;
        self.input[self.pos - 1]
    }

    fn append_tag_name(&mut self, c: char) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    ref mut tag,
                    self_closing: _,
                    attributes: _,
                }
                | HtmlToken::EndTag { ref mut tag } => tag.push(c),
                _ => panic!("`latest_token` should be either StartTag or End Tag"),
            }
            
        }
    }

    fn take_latest_token(&mut self) -> Option<HtmlToken> {
        assert!(self.latest_token.is_some());

        let t = self.latest_token.as_ref().cloned();
        self.latest_token = None;
        assert!(self.latest_token.is_none());
        t
    }

    fn start_new_attribute(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                     tag: _, 
                     self_closing: _,
                     ref mut attributes,
                     } => {
                        attributes.push(Attribute::new());
                     }
                _ => panic!("`latest_token` should be either StartTag or End Tag"),
                }
            }
        }

    fn append_attribute(&mut self, c: char, is_name: bool) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    let len = attributes.len();
                    assert!(len > 0);

                    attributes[len - 1].addr_char(c, is_name);
                }

                _ => panic!("`latest_token` should be either StartTag or End Tag"),
            }
        }
    }

    fn set_self_closing_flag(&mut self) {
        assert!(self.latest_token.is_some());

        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    ref mut self_closing,
                    attributes: _,
                } => *self_closing = true, // ここがメイン
                _ => panic!("`latest_token` should be either StartTag or End Tag"),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    // 開始タグ
    StartTag {
        tag: String,
        self_closing: bool, // 自己終了タグかどうか
        attributes: Vec<Attribute>, // 属性
    },
    // 終了タグ
    EndTag {
        tag: String,
    },

    // 文字
    Char(char),
    // ファイルの終了（EOF)
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// https://html.spec.whatwg.org/multipage/parsing.html#data-state
    Data,
    /// https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
    TagOpen,
    /// https://html.spec.whatwg.org/multipage/parsing.html#end-tag-open-state
    EndTagOpen,
    /// https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state
    TagName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state
    BeforeAttributeName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state
    AttributeName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-name-state
    AfterAttributeName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state
    BeforeAttributeValue,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state
    AttributeValueDoubleQuoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state
    AttributeValueSingleQuoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(unquoted)-state
    AttributeValueUnquoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
    AfterAttributeValueQuoted,
    /// https://html.spec.whatwg.org/multipage/parsing.html#self-closing-start-tag-state
    SelfClosingStartTag,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-state
    ScriptData,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-less-than-sign-state
    ScriptDataLessThanSign,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-open-state
    ScriptDataEndTagOpen,
    /// https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-name-state
    ScriptDataEndTagName,
    /// https://html.spec.whatwg.org/multipage/parsing.html#temporary-buffer
    TemporaryBuffer,
}

// rustの組み込みtraitの一つ、Iterator
// トークンを一つずつ返す
impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }


    loop {
        let c = match self.reconsume {
            true => self.reconsume_input(),
            false => self.consume_next_input(),
        };

        match self.state {
            State::Data => {
                // 開始タグを検出した場合、タグオープン状態に遷移する
                if c == '<' {
                    self.state = State::TagOpen;
                    continue;
                }

                // ファイルの終了（EOF)トークンを返す
                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                // 文字トークンを返す
                return Some(HtmlToken::Char(c));
            }

            // タグオープン状態
            State::TagOpen => {
                // 終了タグを検出した場合、終了タグオープン状態に遷移する
                if c == '/' {
                    self.state = State::EndTagOpen;
                    continue;
                }

                // タグ名を検出した場合、タグ名状態に遷移する
                if c.is_ascii_alphabetic() {
                    // 現在の文字を再度取り扱うためにreconsumeをtrueにする
                    self.reconsume = true;
                    self.state = State::TagName;
                    self.create_tag(true);
                    continue;
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.reconsume = true;
                self.state = State::Data;
            }

            State::EndTagOpen => {
                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }   

                if c.is_ascii_alphabetic() {
                    self.reconsume = true;
                    self.state = State::TagName;
                    self.create_tag(false);
                    continue;
                }
            }

            State::TagName => {
                if c == ' ' {
                    self.state = State::BeforeAttributeName;
                    continue;
                }

                if c == '/' {
                    self.state = State::SelfClosingStartTag;
                    continue;
                }

                if c == '>' {
                    self.state = State::Data;
                    return self.take_latest_token();
                }

                if c.is_ascii_uppercase() {
                    self.append_tag_name(c.to_ascii_lowercase());
                    continue;
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.append_tag_name(c);
            }

            State::BeforeAttributeName => {
                if c == '/' || c == '>' || self.is_eof() {
                    self.reconsume = true;
                    self.state = State::AfterAttributeName;
                    continue;
                }

                self.reconsume = true;
                self.state = State::AttributeName;
                self.start_new_attribute();
            }

            // 属性名状態
            State::AttributeName => {
                if c == ' ' || c == '/' || c == '>' || self.is_eof() {
                    self.reconsume = true;
                    self.state = State::AfterAttributeName;
                    continue;
                }

                // イコールの時
                if c == '=' {
                    self.state = State::BeforeAttributeValue;
                    continue;
                }

                if c.is_ascii_uppercase() {
                    self.append_attribute(c.to_ascii_lowercase(),/*is_name */true);
                    continue;   
                }

                self.append_attribute(c, /*is_name */true);
            }

            State::AfterAttributeName => {
                if c == ' ' {
                    continue;
                }

                if c == '/' {
                    self.state = State::SelfClosingStartTag;
                    continue;
                }

                if c == '=' {
                    self.state = State::BeforeAttributeValue;
                    continue;
                }

                if c == '>' {
                    self.state = State::Data;
                    return self.take_latest_token();
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.reconsume = true;
                self.state = State::AttributeName;
                self.start_new_attribute();
            }

            State::BeforeAttributeValue => {
                // 空白文字は無視する
                if c == ' ' {
                    continue;
                }

                if c == '"' {
                    self.state = State::AttributeValueDoubleQuoted;
                    continue;
                }

                if c == '\'' {
                    self.state = State::AttributeValueSingleQuoted;
                    continue;
                }

                self.reconsume = true;
                self.state = State::AttributeValueUnquoted;
            }
            State::AttributeValueDoubleQuoted => {
                if c == '"' {
                    self.state = State::AfterAttributeValueQuoted;
                    continue;
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.append_attribute(c, /*is_name */false);
            }

            State::AttributeValueSingleQuoted => {
                if c == '\'' {
                    self.state = State::AfterAttributeValueQuoted;
                    continue;
                
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.append_attribute(c, /*is_name */false);
            }

            State::AttributeValueUnquoted => {
                if c == ' ' {
                    self.state = State::BeforeAttributeName;
                    continue;
                }

                if c == '>' {
                    self.state = State::Data;
                    return self.take_latest_token();
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.append_attribute(c, /*is_name */false);
            }

            State::AfterAttributeValueQuoted => {
                if c == ' ' {
                    self.state = State::BeforeAttributeName;
                    continue;
                }

                if c == '/' {
                    self.state = State::SelfClosingStartTag;
                    continue;
                }

                if c == '>' {
                    self.state = State::Data;
                    return self.take_latest_token();
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                self.reconsume = true;
                self.state = State::BeforeAttributeValue;
            }

            State::SelfClosingStartTag => {
                if c == '>' {
                    self.set_self_closing_flag();
                    self.state = State::Data;
                    return self.take_latest_token();
                }

                if self.is_eof() {
                    // invalid parse error
                    return Some(HtmlToken::Eof);
                }

                self.reconsume = true;
            }
            State::ScriptData => {
                if c == '<' {
                    self.state = State::ScriptDataLessThanSign;
                    continue;
                }

                if self.is_eof() {
                    return Some(HtmlToken::Eof);
                }

                return Some(HtmlToken::Char(c));
            }

            State::ScriptDataLessThanSign => {
                if c == '/' {
                    // 一時的なバッファを空文字でリセットする
                    self.buf = String::new();
                    self.state = State::ScriptDataEndTagOpen;
                    continue;
                }

                self.reconsume = true;
                self.state = State::ScriptData;
                return Some(HtmlToken::Char('<'));
            }

            State::ScriptDataEndTagOpen => {
                if c.is_ascii_alphabetic() {
                    self.reconsume = true;
                    self.state = State::ScriptDataEndTagName;
                    self.create_tag(false);
                    continue;
                }
                self.reconsume = true;
                self.state = State::ScriptData;
                return Some(HtmlToken::Char('<'));
            }

            State::ScriptDataEndTagName => {
                if c == '>' {
                    self.state = State::ScriptData;
                    return self.take_latest_token();
                }

                if c.is_ascii_alphabetic() {
                    self.buf.push(c);
                    self.append_tag_name(c.to_ascii_lowercase());
                    continue;
                }

                self.state = State::TemporaryBuffer;
                self.buf = String::from("</") + &self.buf;
                self.buf.push(c);
                continue;
            }

            State::TemporaryBuffer => {
                self.reconsume = true;

                if self.buf.chars().count() == 0 {
                    self.state = State::ScriptData;
                    continue;
                }

                // 最初の1文字を削除する

                let c = self.buf.chars().nth(0)
                .expect("self.buf should have at least one character");

                self.buf.remove(0);
                return Some(HtmlToken::Char(c));
            }
        }
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_empty() {
        let html = "".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        assert_eq!(tokenizer.next(), None);
    }
    #[test]
    fn test_start_and_end_tag() {
        let html = "<body></body>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "body".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::EndTag {
                tag: "body".to_string(),
            },
        ];

        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_attributes() {
        let html = "<p class=\"A\" id=\"B\" foo=bar></p>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let mut attr1 = Attribute::new();
        attr1.addr_char('c', true);
        attr1.addr_char('l', true);
        attr1.addr_char('a', true);
        attr1.addr_char('s', true);
        attr1.addr_char('s', true);
        attr1.addr_char('A', false);

        let mut attr2 = Attribute::new();
        attr2.addr_char('i', true);
        attr2.addr_char('d', true);
        attr2.addr_char('B', false);

        let mut attr3 = Attribute::new();
        attr3.addr_char('f', true);
        attr3.addr_char('o', true);
        attr3.addr_char('o', true);
        attr3.addr_char('b', false);
        attr3.addr_char('a', false);
        attr3.addr_char('r', false);

        let expected = [
            HtmlToken::StartTag {
                tag: "p".to_string(),
                self_closing: false,
                attributes: vec![attr1, attr2, attr3],
            },
            HtmlToken::EndTag {
                tag: "p".to_string(),
            },
        ];

        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_self_closing_tag() {
        let html = "<img />".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "img".to_string(),
                self_closing: true,
                attributes: Vec::new(),
            },
        ];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }

    #[test]
    fn test_script_tag() {
        let html = "<script>js code;</script>".to_string();
        let mut tokenizer = HtmlTokenizer::new(html);
        let expected = [
            HtmlToken::StartTag {
                tag: "script".to_string(),
                self_closing: false,
                attributes: Vec::new(),
            },
            HtmlToken::Char('j'),
            HtmlToken::Char('s'),
            HtmlToken::Char(' '),
            HtmlToken::Char('c'),
            HtmlToken::Char('o'),
            HtmlToken::Char('d'),
            HtmlToken::Char('e'),
            HtmlToken::Char(';'),
            HtmlToken::EndTag {
                tag: "script".to_string(),
            },
        ];
        for e in expected {
            assert_eq!(Some(e), tokenizer.next());
        }
    }
}
