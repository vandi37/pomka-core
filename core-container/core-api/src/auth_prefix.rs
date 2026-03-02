use std::str::FromStr;

#[derive(Debug, Clone, PartialEq )]
pub enum AuthPrefix {
    AdminAccess,
    AdminRefresh,
    BotAccess,
    BotRefresh,
    Userbot,
}

impl AuthPrefix {
    pub fn cut_prefix<'a>(s: &'a str) -> Option<(Self, &'a str)> {
        let (prefix, value) = s.split_once(' ')?;
        Some((AuthPrefix::from_str(prefix).ok()?, value))
    }
}

impl ToString for AuthPrefix {
    fn to_string(&self) -> String {
        match self {
            &Self::AdminAccess => "Admin-Access",
            &Self::AdminRefresh => "Admin-Refresh",
            &Self::BotAccess => "Bot-Access",
            &Self::BotRefresh => "Bot-Refresh",
            &Self::Userbot => "Userbot",
        }.to_string()
    }
}

impl FromStr for AuthPrefix {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin-Access" => Ok(Self::AdminAccess),
            "Admin-Refresh" => Ok(Self::AdminRefresh),
            "Bot-Access" => Ok(Self::BotAccess),
            "Bot-Refresh" => Ok(Self::BotRefresh),
            "Userbot" => Ok(Self::Userbot),
            _ => Err(())
        }
    }
}