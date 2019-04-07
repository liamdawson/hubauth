#[macro_use] extern crate nom;
const EMPTY_BYTE_SLICE: &[u8] = &[];

fn is_base64_char(b: u8) -> bool {
    nom::is_alphanumeric(b) || b == 43 || b == 47
}

named!(alg<&[u8], &[u8]>, alt!(
    tag_no_case!("dsa") |
    tag_no_case!("rsa") |
    tag_no_case!("ecdsa") |
    tag_no_case!("ed25519")));

named!(key_type<&[u8], (&[u8], &[u8])>, pair!(tag_no_case!("ssh-"), alg));
named!(base64<&[u8], (&[u8], &[u8])>, pair!(take_while!(is_base64_char), is_a!("=")));
named!(pubkey<&[u8], String>, do_parse!(
        key_type: key_type >>
        take_while!(nom::is_space) >>
        key: base64 >>
        (
            format!("ssh-{} {}", std::str::from_utf8(key_type.1).unwrap(), std::str::from_utf8(&[key.0, key.1].concat()).unwrap())
        )
    )
);

named!(pubkey_with_comment<&[u8], String>, do_parse!(
    pubkey: pubkey >>
    take_while!(nom::is_space) >>
    comment_pair: many_till!(none_of!("\r\n"), eof!()) >>
    (
        if comment_pair.0.len() > 0 {
            format!("{} {}", pubkey, comment_pair.0.into_iter().collect::<String>())
        } else {
            pubkey
        }
    )
));

#[cfg(test)]
mod tests {
    #[test]
    fn it_normalizes_spaces_in_a_pubkey() {
        assert_eq!(super::pubkey("ssh-rsa        AAAAB3NzuQ==".as_bytes()).expect("ssh key was not recognised").1, "ssh-rsa AAAAB3NzuQ==".to_owned());
    }

    #[test]
    fn it_normalizes_spaces_in_a_pubkey_with_comment() {
        assert_eq!(super::pubkey_with_comment("ssh-rsa        AAAAB3NzuQ==        liam@example.com".as_bytes()).expect("ssh key was not recognised").1, "ssh-rsa AAAAB3NzuQ== liam@example.com".to_owned());
    }
}
