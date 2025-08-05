use alloc::string::String;
use alloc::vec::Vec;
use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer{
    state: State,
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    imput: Vec<char>,
    buf: String,
}

impl HtmlTokenizer{
    pub fn new(html: String) -> Self {
        Self{
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }
}

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
