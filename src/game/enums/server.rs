use std::fmt::{Display, Formatter};

pub enum Server {
    US_EAST,
    US_WEST,
    EUROPE,
    SOUTH_KOREA,
    ASIA,
    SOUTH_AMERICA,
    AUSTRALIA,
    JAPAN,
    MIDDLE_EAST,
    SOUTH_AFRICA,
    UNKNOWN, // DEBUG or DEBUG_GLOBAL
}

impl Server {
    pub fn ip(&self) -> &str {
        match self {
            Server::US_EAST => "45.56.113.95",
            Server::US_WEST => "45.79.69.110",
            Server::EUROPE => "172.105.248.252",
            Server::SOUTH_KOREA => "52.79.53.242",
            Server::ASIA => "139.162.49.99",
            Server::SOUTH_AMERICA => "54.94.231.111",
            Server::AUSTRALIA => "45.79.238.85",
            Server::JAPAN => "139.162.86.191",
            Server::MIDDLE_EAST => "15.185.65.160",
            Server::SOUTH_AFRICA => "13.245.48.94",
            Server::UNKNOWN => "194.195.115.5",
        }
    }
    pub fn iter() -> Vec<Server> {
        vec![
            Server::US_EAST,
            Server::US_WEST,
            Server::EUROPE,
            Server::SOUTH_KOREA,
            Server::ASIA,
            Server::SOUTH_AMERICA,
            Server::AUSTRALIA,
            Server::JAPAN,
            Server::MIDDLE_EAST,
            Server::SOUTH_AFRICA,
            Server::UNKNOWN,
        ]
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::US_EAST => write!(f, "US East"),
            Server::US_WEST => write!(f, "US West"),
            Server::EUROPE => write!(f, "Europe"),
            Server::SOUTH_KOREA => write!(f, "South Korea"),
            Server::ASIA => write!(f, "Asia"),
            Server::SOUTH_AMERICA => write!(f, "South America"),
            Server::AUSTRALIA => write!(f, "Australia"),
            Server::JAPAN => write!(f, "Japan"),
            Server::MIDDLE_EAST => write!(f, "Middle East"),
            Server::SOUTH_AFRICA => write!(f, "South Africa"),
            Server::UNKNOWN => write!(f, "Unknown"),
        }
    }
}
