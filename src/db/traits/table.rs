use std::{collections::BTreeMap, cell::RefCell, rc::Rc};
use candid::Principal;
use ic_cdk::api::stable::{StableWriter, StableReader};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TableEventKind {
    Create,
    Update,
    Delete
}

#[derive(Clone, Debug)]
pub enum TableEventKey {
    Text(String),
    Principal(Principal),
}

#[derive(Clone, Debug)]
pub struct TableEvent<'a, TN> {
    pub table_name: &'a TN,
    pub kind: TableEventKind,
    pub pkey: TableEventKey,
    pub keys: Vec<TableEventKey>,
    //pub caller: Principal,
}

#[derive(Deserialize)]
pub struct TableData<K, V> (pub BTreeMap<K, V>)
    where 
        K: Ord + Serialize,
        V: Serialize;

pub struct TableSchema<TN> {
    pub version: f32,
    pub name: TN,
}

pub struct TableSubs<TN> (pub Vec<Rc<RefCell<dyn TableSubscriber<TN>>>>);

pub trait Table<TN, K, V>
    where 
        K: Ord + Serialize,
        V: Serialize {

    fn new(
    ) -> Self;

    fn get_schema(
        &self
    ) -> &TableSchema<TN>;
        
    fn get_data(
        &self
    ) -> &TableData<K, V>;

    fn get_data_mut(
        &mut self
    ) -> &mut TableData<K, V>;

    fn set_data(
        &mut self,
        data: TableData<K, V>
    );
}

pub trait TableVersioned<TN, K, V>
    where 
        K: Ord + Serialize,
        V: Serialize,
        Self: Table<TN, K, V> {
    fn migrate(
        &self,
        _from_version: f32,
        _buf: &[u8]
    ) -> Result<TableData<K, V>, String> {
        panic!("Not supported")
    }
}

pub trait TableSerializable<TN, K, V>
    where 
        K: Ord + Serialize,
        V: Serialize,
        Self: Table<TN, K, V> {
    fn serialize(
        &self,
        writer: &mut StableWriter
    ) -> Result<(), String> {
        let arr = rmp_serde::to_vec(&self.get_data().0)
            .map_err(|err| err.to_string())?;
        // store version
        writer.write(&f32::to_le_bytes(self.get_schema().version)).map_err(|e| e.to_string())?;
        // store size
        writer.write(&u64::to_le_bytes(arr.len() as u64)).map_err(|e| e.to_string())?;
        // store table
        writer.write(&arr).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub trait TableDeserializable<TN, K, V>
    where 
        K: Ord + Serialize + for<'a> Deserialize<'a>, 
        V: Serialize +  for<'a> Deserialize<'a>,
        Self: Table<TN, K, V> + TableVersioned<TN, K, V> {
    fn deserialize(
        &mut self, 
        reader: &mut StableReader,
        decode_data: bool
    ) -> Result<(), String> {
        // load version
        let mut version_buf = [0u8; 4];
        reader.read(&mut version_buf).map_err(|e| format!("{:?}", e))?;
        let version = f32::from_le_bytes(version_buf);
        // load size
        let mut size_buf = [0u8; 8];
        reader.read(&mut size_buf).map_err(|e| format!("{:?}", e))?;
        let size = u64::from_le_bytes(size_buf);
        // load table
        let mut table_buf = vec![0u8; size as usize];
        reader.read(&mut table_buf).map_err(|e| format!("{:?}", e))?;
        // decode table
        if decode_data {
            let data: TableData<K, V> = if version == self.get_schema().version {
                rmp_serde::from_slice(&table_buf)
                    .map_err(|err| err.to_string())?
            }
            else {
                self.migrate(version, &table_buf)?
            };
            self.set_data(data);
        }
        Ok(())
    }
}

pub trait TableSubscriber<TN> {
    fn on(
        &mut self,
        event: &TableEvent<TN>
    );
}

pub trait TableSubscribable<TN, K, V>
    where 
        K: Ord + Serialize,
        V: Serialize, {
    fn get_subs(
        &self
    ) -> &TableSubs<TN>;

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs<TN>;

    fn get_pkey(
        k: &K
    ) -> TableEventKey;

    fn get_keys(
        v: &V
    ) -> Vec<TableEventKey>;

    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber<TN>>>
    ) {
        self.get_subs_mut().0.push(tb);
    }

    fn notify (
        &self,
        event: &TableEvent<TN>
    ) {
        self.get_subs().0.iter()
            .for_each(|c| c.borrow_mut().on(event));
    }
}

