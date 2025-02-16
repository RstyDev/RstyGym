

    use chrono::{NaiveDate, NaiveDateTime};
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct BigIntDB {
        pub int: i64,
    }
    #[derive(FromRow)]
    pub struct IntDB {
        pub int: i32,
    }

    #[derive(FromRow)]
    pub struct DoubleDB {
        pub double: f64,
    }

    #[derive(FromRow)]
    pub struct FloatDB {
        pub float: f32,
    }

    #[derive(FromRow)]
    pub struct BoolDB {
        pub val: bool,
    }

    #[derive(FromRow)]
    pub struct StringDB {
        pub string: String,
    }

    #[derive(FromRow)]
    pub struct DayDB {
        pub id: i64,
        pub state: String,
        pub date: NaiveDate,
        pub week: i64,
    }
    #[derive(FromRow)]
    pub struct WeekDB {
        //TODO!
    }

