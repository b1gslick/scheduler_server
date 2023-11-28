pub mod store {

    use crate::types::activities::{Activity, ActivityId};
    use crate::types::time_spent::{TimeSpent, TimeSpentId};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[derive(Clone, Debug)]
    pub struct Store {
        pub activities: Arc<RwLock<HashMap<ActivityId, Activity>>>,
        pub time_spent: Arc<RwLock<HashMap<TimeSpentId, TimeSpent>>>,
    }

    impl Store {
        pub fn new() -> Self {
            Store {
                activities: Arc::new(RwLock::new(Self::init())),
                time_spent: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        fn init() -> HashMap<ActivityId, Activity> {
            let file = include_str!("../activity.json");
            serde_json::from_str(file).expect("can't read questions.json")
        }
    }
}
