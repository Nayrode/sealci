use super::xx_netmask_width;
use futures::stream::TryStreamExt;
use rtnetlink::{new_connection, Handle};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub enum Error {
    GetIndexByName {
        name: String,
        cause: rtnetlink::Error,
    },
    CreateBridge(rtnetlink::Error),
    NoLinkPresent(String),
    AddAddress(rtnetlink::Error),
    AttachLink {
        link_name: String,
        cause: rtnetlink::Error,
    },
    SetStateAsUp(rtnetlink::Error),
}

#[derive(Clone)]
pub struct Bridge {
    name: String,
    handle: Handle,
}

impl Bridge {
    pub async fn new(name: &str) -> Result<Self, Error> {
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);

        let br = Self {
            name: name.into(),
            handle,
        };
        br.create_bridge_if_not_exist().await?;

        Ok(br)
    }

    async fn get_index_by_name(&self, name: &str) -> Result<u32, Error> {
        let option = self
            .handle
            .link()
            .get()
            .match_name(name.into())
            .execute()
            .try_next()
            .await
            .map_err(|err| Error::GetIndexByName {
                name: name.into(),
                cause: err,
            })?;

        match option {
            Some(a) => Ok(a.header.index),
            None => Err(Error::NoLinkPresent(name.into())),
        }
    }

    async fn create_bridge_if_not_exist(&self) -> Result<(), Error> {
        let result = self.get_index_by_name(&self.name).await;
        if result.is_ok() {
            println!("bridge is already presents");

            return Ok(());
        }

        println!("bridge not found, creating bridge...");

        self.handle
            .clone()
            .link()
            .add()
            .bridge(self.name.clone())
            .execute()
            .await
            .map_err(Error::CreateBridge)
    }

    pub async fn set_addr(&self, addr: Ipv4Addr, netmask: Ipv4Addr) -> Result<(), Error> {
        let bridge_index = self.get_index_by_name(&self.name).await?;
        let prefix_len = xx_netmask_width(netmask.octets());

        let addr_get_request = self
            .handle
            .address()
            .get()
            .set_link_index_filter(bridge_index)
            .set_address_filter(IpAddr::V4(addr))
            .set_prefix_length_filter(prefix_len)
            .execute()
            .try_next()
            .await;
        if let Ok(Some(_)) = addr_get_request {
            println!("address {:?} already exists for bridge", addr);

            return Ok(());
        }

        println!(
            "addr not found, set addr {} with mask {} for bridge",
            addr, netmask
        );
        self.handle
            .address()
            .add(bridge_index, IpAddr::V4(addr), prefix_len)
            .execute()
            .await
            .map_err(Error::AddAddress)
    }

    pub async fn attach_link(&self, link_name: String) -> Result<(), Error> {
        let link_index = self.get_index_by_name(&link_name).await?;
        let master_index = self.get_index_by_name(&self.name).await?;

        self.handle
            .link()
            .set(link_index)
            .controller(master_index)
            .execute()
            .await
            .map_err(|err| Error::AttachLink {
                link_name,
                cause: err,
            })
    }

    pub async fn set_up(&self) -> Result<(), Error> {
        let bridge_index = self.get_index_by_name(&self.name).await?;

        self.handle
            .link()
            .set(bridge_index)
            .up()
            .execute()
            .await
            .map_err(Error::SetStateAsUp)
    }
}
