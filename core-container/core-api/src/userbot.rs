

use signed_token::Signed;


pub fn get_userbot(token: &str, signed: &Signed) -> Option<(i64, i64)> {
    let (signed_token, id) = token.split_once(':')?;
    Some((signed.verify(signed_token)?, id.parse().ok()?))
}