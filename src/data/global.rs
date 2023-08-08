use std::collections::HashMap;

use super::data::MemData;

struct DataItem {
    pub data: MemData,
    ref_count: i64,
}

impl DataItem {
    pub fn new(data: MemData) -> DataItem {
        DataItem { data, ref_count: 0 }
    }

    pub fn add_ref(&mut self) {
        self.ref_count += 1;
    }

    // Returns whether ref count is zero.
    pub fn deref(&mut self) -> bool {
        self.ref_count -= 1;
        self.ref_count <= 0
    }
}

pub struct GlobalData {
    pub variables: HashMap<i64, DataItem>,
    next_id: i64,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        GlobalData {
            variables: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, data: MemData) -> i64 {
        let id = self.next_id();
        let mut item = DataItem::new(data);
        item.add_ref();
        self.variables.insert(id, item);
        return id;
    }

    pub fn get(&self, id: i64) -> Option<&MemData> {
        if let Some(data) = self.variables.get(&id) {
            Some(&data.data)
        } else {
            None
        }
    }

    pub fn obtain(&mut self, id: i64) -> Option<&MemData> {
        if let Some(data) = self.variables.get_mut(&id) {
            data.add_ref();
            Some(&data.data)
        } else {
            None
        }
    }

    pub fn release(&mut self, id: i64) {
        if let Some(data) = self.variables.get_mut(&id) {
            if data.deref() {
                self.variables.remove(&id);
            }
        }
    }

    fn next_id(&mut self) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub struct Global {
    pub data: GlobalData,
}

impl Global {
    pub fn new() -> Global {
        Global {
            data: GlobalData::new(),
        }
    }
}
