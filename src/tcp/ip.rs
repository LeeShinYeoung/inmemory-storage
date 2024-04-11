use std::{net::SocketAddr, str::FromStr};

use ipnet::IpNet;
use serde::Deserialize;

#[derive(Debug, Clone)]
struct Ip(IpNet);
impl<'de> Deserialize<'de> for Ip {
  fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let str: String = Deserialize::deserialize(deserializer)?;
    IpNet::from_str(&str)
      .map(|ip| Self(ip))
      .map_err(|_| serde::de::Error::custom("invalid ip"))
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct IpNetList(Vec<Ip>);
impl IpNetList {
  pub fn contains(&self, addr: &SocketAddr) -> bool {
    for ip in self.0.iter().map(|i| i.0) {
      if ip.contains(&addr.ip()) {
        return true;
      }
    }
    false
  }
}
