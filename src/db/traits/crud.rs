use candid::CandidType;
use serde::{Deserialize, Serialize};
use super::table::{TableSubscribable, TableEventKind, Table, TableEvent};

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
}

pub trait Crud<TN, K, V> 
    where 
        K: Ord + Serialize,
        V: Serialize, 
        Self: Table<TN, K, V> {
    
    fn insert(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if self.get_data().0.contains_key(&k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn find_by_id<'a>(
        &'a self,
        k: &'a K
    ) -> Option<&'a V> {
       self.get_data().0.get(k)
    }

    fn find_by_id_mut<'a>(
        &'a mut self,
        k: &'a K
    ) -> Option<&'a mut V> {
       self.get_data_mut().0.get_mut(k)
    }

    fn get<'a>(
        &'a self,
        k: &'a K
    ) -> &'a V {
        self.get_data().0.get(k).unwrap()
    }

    fn get_mut<'a>(
        &'a mut self,
        k: &'a K
    ) -> &'a mut V {
        self.get_data_mut().0.get_mut(k).unwrap()
    }

    fn update(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if !self.get_data().0.contains_key(&k) {
            Err("Not found".to_string())
        }
        else {
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn delete(
        &mut self,
        k: &K
    ) -> Result<(), String> {
        match self.get_data_mut().0.remove(k) {
            None => Err("Not found".to_string()),
            _ => Ok(())
        }
    }
}

pub trait CrudSubscribable<TN, K, V> 
    where 
        K: Ord + Serialize,
        V: Serialize, 
        Self: Table<TN, K, V> + Crud<TN, K, V> + TableSubscribable<TN, K, V> {
    fn insert_and_notify(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if self.get_data().0.contains_key(&k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.notify(&TableEvent {
                table_name: &self.get_schema().name,
                kind: TableEventKind::Create, 
                pkey: Self::get_pkey(&k),
                keys: Self::get_keys(&v)
            });
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn update_and_notify(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if !self.get_data().0.contains_key(&k) {
            Err("Not found".to_string())
        }
        else {
            self.notify(&TableEvent {
                table_name: &self.get_schema().name,
                kind: TableEventKind::Update, 
                pkey: Self::get_pkey(&k),
                keys: Self::get_keys(&v)
            });
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn delete_and_notify(
        &mut self,
        k: &K
    ) -> Result<(), String> {
        let v = self.get_data_mut().0.remove(k);
        if let Some(v) = v {
            self.notify(&TableEvent {
                table_name: &self.get_schema().name,
                kind: TableEventKind::Delete, 
                pkey: Self::get_pkey(k),
                keys: Self::get_keys(&v)
            });
            Ok(())
        }
        else {
            Err("Not found".to_string())
        }
    }
}

