mod source;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::source::http::Http;
use crate::source::Result;

#[derive(Debug)]
pub struct Consensus {
    v4: Option<Ipv4Addr>,
    v6: Option<Ipv6Addr>,
}

impl Consensus {
    pub fn from_ips<T: IntoIterator<Item = IpAddr>>(ips: T) -> Self {
        let mut votes_v4 = HashMap::new();
        let mut votes_v6 = HashMap::new();
        for ip in ips {
            match ip {
                IpAddr::V4(ip) => votes_v4.entry(ip).and_modify(|c| *c += 1).or_insert(1),
                IpAddr::V6(ip) => votes_v6.entry(ip).and_modify(|c| *c += 1).or_insert(1),
            };
        }

        Self {
            v4: sort_votes(votes_v4),
            v6: sort_votes(votes_v6),
        }
    }

    pub async fn get() -> Result<Self> {
        let http_source = Http::new()?;
        let ips = http_source.get_ips().await;
        Ok(Self::from_ips(ips))
    }
}

fn sort_votes<U: Copy>(votes: HashMap<U, i32>) -> Option<U> {
    let mut ordered_votes: Vec<_> = votes.iter().collect();
    ordered_votes.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
    ordered_votes.pop().map(|(ip, _)| *ip)
}

#[cfg(test)]
mod tests {
    use crate::Consensus;
    use tokio_test::block_on;

    #[test]
    fn it_works() {
        async fn do_test() {
            match Consensus::get().await {
                Ok(c) => println!("{:#?}", c),
                Err(err) => println!("{}", err),
            }
        }
        block_on(do_test());
    }
}
